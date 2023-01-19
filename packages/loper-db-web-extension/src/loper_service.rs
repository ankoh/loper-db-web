use loper_db_proto_rs::hyper_database_service_client::HyperDatabaseServiceClient;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use once_cell::sync::OnceCell;

type ConnectionId = u64;
type SessionId = u64;
type QueryId = u64;

#[derive(Default)]
struct LoperService {
    _connections: HashMap<ConnectionId, LoperServiceConnection>,
    _next_connection_id: u64,
}

struct LoperServiceConnection {
    _connection_id: ConnectionId,
    _client: HyperDatabaseServiceClient<tonic::transport::Channel>,
    _sessions: HashMap<SessionId, LoperServiceSession>,
    _next_session_id: u64,
}

struct LoperServiceSession {
    _connection_id: ConnectionId,
    _session_id: SessionId,
    _client: HyperDatabaseServiceClient<tonic::transport::Channel>,
    _queries: HashMap<QueryId, Arc<Mutex<LoperServiceQuery>>>,
    _next_query_id: u64,
}

struct LoperServiceQuery {
    _connection_id: ConnectionId,
    _session_id: SessionId,
    _query_id: QueryId,
    // _result_stream: Mutex<tonic::codec::Streaming<loper_db_proto_rs::QueryResult>>,
    _signal_reader_to_stop: tokio::sync::Notify,
    _signal_reader_stopped: tokio::sync::Notify,
    _result_channel: tokio::sync::mpsc::Receiver<Result<loper_db_proto_rs::QueryResult, String>>,
}

impl LoperService {
    pub fn get() -> &'static Mutex<LoperService> {
        static SERVICE: OnceCell<Mutex<LoperService>> = OnceCell::new();
        SERVICE.get_or_init(|| Mutex::new(LoperService::default()))
    }

    pub async fn read_query_result_stream(ci: ConnectionId, si: SessionId, qi: QueryId) -> Result<Vec<loper_db_proto_rs::QueryResult>, String> {
        // Resolve the query
        let query_mtx = {
            let svc = LoperService::get().lock().await;
            let conn = svc._connections.get(&ci).unwrap(); // XXX
            let sess = conn._sessions.get(&si).unwrap();
            let query = sess._queries.get(&qi).unwrap();
            query.clone()
        };
        let mut query = query_mtx.lock().await;

        // Fetch all results from the channel
        let mut messages = Vec::new();
        loop {
            match query._result_channel.try_recv() {
                Ok(Ok(result)) => messages.push(result),
                Ok(Err(grpc_error)) => return Err(grpc_error),
                Err(_) => break
            }
        }

        // Return early to the user if we fetched buffered messages
        if !messages.is_empty() {
            return Ok(messages);
        }

        // Otherwise block on the channel
        match query._result_channel.recv().await {
            Some(Ok(result)) => messages.push(result),
            Some(Err(grpc_error)) => return Err(grpc_error),
            None => () // Channel shutdown
        }
        Ok(messages)
    }
}