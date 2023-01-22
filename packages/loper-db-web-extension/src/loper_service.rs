use loper_db_proto_rs::hyper_database_service_client::HyperDatabaseServiceClient;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

type SlotId = usize;
type QueryResultChunkResult = Result<Option<loper_db_proto_rs::QueryResult>, String>;

#[derive(Default)]
struct LoperService {
    connections: Vec<Option<LoperServiceConnection>>,
}

struct LoperServiceConnection {
    client: HyperDatabaseServiceClient<tonic::transport::Channel>,
    sessions: Vec<Option<LoperServiceSession>>,
}

struct LoperServiceSession {
    client: HyperDatabaseServiceClient<tonic::transport::Channel>,
    queries: Vec<Option<Arc<Mutex<LoperServiceQuery>>>>,
}

struct LoperServiceQuery {
    result_channel: mpsc::Receiver<QueryResultChunkResult>,
}

/// Helper to allocate an element in a slot vector.
/// We allocate slots in vectors to return small and efficient handles to the user.
fn alloc_slot<'a, V>(elements: &'a mut Vec<Option<V>>) -> (SlotId, &'a mut Option<V>) {
    for i in 0..elements.len() {
        if elements[i].is_none() {
            return (i, &mut elements[i]);
        }
    }
    elements.push(None);
    let id = elements.len() - 1;
    return (id, &mut elements[id]);
}

/// Free the slot and
fn free_slot<'a, V>(elements: &'a mut Vec<Option<V>>, id: SlotId) {
    elements[id] = None;
    if id == (elements.len() - 1) {
        elements.pop();
        while elements.last().is_none() {
            elements.pop();
        }
    }
}

impl LoperServiceQuery {
    fn create() -> (Self, mpsc::Sender<QueryResultChunkResult>) {
        let (sender, receiver) = mpsc::channel(10);
        let query = Self {
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
    pub async fn execute_query(ci: SlotId, si: SlotId, text: String) -> Result<(), String> {
        // Create a query object
        let (mut client, query_id, sender) = {
            let mut svc = LoperService::get().lock().await;
            let conn = svc.connections[ci]
                .as_mut()
                .ok_or_else(|| format!("failed to resolve connection with id {}", ci))?;
            let sess = conn.sessions[si]
                .as_mut()
                .ok_or_else(|| format!("failed to resolve session with id {}", si))?;
            let client = sess.client.clone();
            let (query_id, query_out) = alloc_slot(&mut sess.queries);
            let (query, sender) = LoperServiceQuery::create();
            query_out.replace(Arc::new(Mutex::new(query)));
            (client, query_id, sender)
        };

        // Execute the query
        let mut result_stream = match {
            let query_param = loper_db_proto_rs::QueryParam {
                query: text,
                ..Default::default()
            };
            let result_stream = client
                .execute_query(query_param)
                .await
                .map_err(|e| e.to_string())?;
            Ok(result_stream.into_inner())
        } {
            Ok(s) => s,
            Err(e) => {
                // The query execution failed, free the slot
                let mut svc = LoperService::get().lock().await;
                let conn = svc.connections[ci].as_mut().unwrap();
                let sess = conn.sessions[si].as_mut().unwrap();
                free_slot(&mut sess.queries, query_id);
                return Err(e);
            }
        };

        // Spawn the reader to poll the query result
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
        Ok::<(), String>(())
    }

    /// Read from a query result stream
    pub async fn read_query_result_stream(
        ci: SlotId,
        si: SlotId,
        qi: SlotId,
    ) -> Result<Vec<Option<loper_db_proto_rs::QueryResult>>, String> {
        // Resolve the query
        let query_mtx = {
            let mut svc = LoperService::get().lock().await;
            let conn = svc.connections[ci]
                .as_mut()
                .ok_or_else(|| format!("failed to resolve connection with id {}", ci))?;
            let sess = conn.sessions[si]
                .as_mut()
                .ok_or_else(|| format!("failed ot resolve session with id {}", si))?;
            let query = sess.queries[qi]
                .as_mut()
                .ok_or_else(|| format!("failed to resolve query with id {}", qi))?;
            query.clone()
        };
        let mut query = query_mtx.lock().await;

        // Fetch all buffered results from the channel without waiting
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
