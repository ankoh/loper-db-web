use loper_db_proto_rs::hyper_database_service_client::HyperDatabaseServiceClient;
use once_cell::sync::OnceCell;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc;
use tokio::sync::Mutex;

type ConnectionId = u64;
type SessionId = u64;
type QueryId = u64;
type QueryResultChunkResult = Result<Option<loper_db_proto_rs::QueryResult>, String>;

#[derive(Default)]
struct LoperService {
    connections: HashMap<ConnectionId, LoperServiceConnection>,
    _next_connection_id: u64,
}

struct LoperServiceConnection {
    _connection_id: ConnectionId,
    _client: HyperDatabaseServiceClient<tonic::transport::Channel>,
    sessions: HashMap<SessionId, LoperServiceSession>,
    _next_session_id: u64,
}

struct LoperServiceSession {
    _connection_id: ConnectionId,
    _session_id: SessionId,
    client: HyperDatabaseServiceClient<tonic::transport::Channel>,
    queries: HashMap<QueryId, Arc<Mutex<LoperServiceQuery>>>,
    next_query_id: u64,
}

struct LoperServiceQuery {
    connection_id: ConnectionId,
    session_id: SessionId,
    query_id: QueryId,
    result_channel: mpsc::Receiver<QueryResultChunkResult>,
}

impl LoperServiceQuery {
    fn create(
        ci: ConnectionId,
        si: SessionId,
        qi: QueryId,
    ) -> (Self, mpsc::Sender<QueryResultChunkResult>) {
        let (sender, receiver) = mpsc::channel(10);
        let query = Self {
            connection_id: ci,
            session_id: si,
            query_id: qi,
            result_channel: receiver,
        };
        (query, sender)
    }
}

impl LoperService {
    /// Get the global service
    pub fn get() -> &'static Mutex<LoperService> {
        static SERVICE: OnceCell<Mutex<LoperService>> = OnceCell::new();
        SERVICE.get_or_init(|| Mutex::new(LoperService::default()))
    }

    /// Execute a query
    pub async fn execute_query(
        ci: ConnectionId,
        si: SessionId,
        text: String,
    ) -> Result<(), String> {
        // Create a query object
        let (mut client, query_id, sender) = {
            let mut svc = LoperService::get().lock().await;
            let conn = svc
                .connections
                .get_mut(&ci)
                .ok_or_else(|| format!("failed to resolve connection with id {}", ci))?;
            let mut sess = conn
                .sessions
                .get_mut(&si)
                .ok_or_else(|| format!("failed ot resolve session with id {}", si))?;
            let client = sess.client.clone();
            let (query, sender) = LoperServiceQuery::create(ci, si, sess.next_query_id);
            let query_id = query.query_id;
            let query = Arc::new(Mutex::new(query));
            sess.next_query_id += 1;
            sess.queries.insert(query_id, query);
            (client, query_id, sender)
        };

        // Execute the query
        let query_param = loper_db_proto_rs::QueryParam {
            query: text,
            ..Default::default()
        };
        let result_stream = client
            .execute_query(query_param)
            .await
            .map_err(|e| e.to_string())?;
        let mut result_stream = result_stream.into_inner();

        // Spawn the reader
        tokio::spawn(async move {
            loop {
                // Read a message or cancel if the receiver was closed
                let v = tokio::select! {
                    v = result_stream.message() => { v }
                    _ = sender.closed() => break
                };
                match v {
                    // Received a query result, send over channel
                    Ok(Some(r)) => {
                        if let Err(_) = sender.send(Ok(Some(r))).await {
                            // Do nothing if the receiver side was closed
                            debug_assert!(sender.is_closed());
                            break;
                        }
                    }
                    Ok(None) => {
                        // Reached EOS, forward and wait for receiver to close channel
                        sender.send(Ok(None)).await.ok();
                        sender.closed().await;
                        break;
                    }
                    Err(e) => {
                        // Received error, forward and wait for receiver to close channel
                        sender.send(Err(e.to_string())).await.ok();
                        sender.closed().await;
                        break;
                    }
                }
                // Otherwise continue with next message
            }
        });

        Ok(())
    }

    /// Read from a query result stream
    pub async fn read_query_result_stream(
        ci: ConnectionId,
        si: SessionId,
        qi: QueryId,
    ) -> Result<Vec<Option<loper_db_proto_rs::QueryResult>>, String> {
        // Resolve the query
        let query_mtx = {
            let svc = LoperService::get().lock().await;
            let conn = svc
                .connections
                .get(&ci)
                .ok_or_else(|| format!("failed to resolve connection with id {}", ci))?;
            let sess = conn
                .sessions
                .get(&si)
                .ok_or_else(|| format!("failed ot resolve session with id {}", si))?;
            let query = sess
                .queries
                .get(&qi)
                .ok_or_else(|| format!("failed to resolve query with id {}", qi))?;
            query.clone()
        };
        let mut query = query_mtx.lock().await;

        // Fetch all results from the channel
        let mut messages = Vec::new();
        loop {
            match query.result_channel.try_recv() {
                Ok(Ok(result)) => messages.push(result),
                Ok(Err(grpc_error)) => return Err(grpc_error),
                Err(_) => break,
            }
        }

        // Return early to the user if we fetched buffered messages
        if !messages.is_empty() {
            return Ok(messages);
        }

        // Otherwise block on the channel
        match query.result_channel.recv().await {
            Some(Ok(result)) => messages.push(result),
            Some(Err(grpc_error)) => return Err(grpc_error),
            None => (), // Channel shutdown
        }
        Ok(messages)
    }
}
