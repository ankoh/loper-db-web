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
    _queries: HashMap<QueryId, Arc<LoperServiceQuery>>,
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
    pub async fn get() -> &'static Arc<Mutex<LoperService>> {
        static SERVICE: OnceCell<Arc<Mutex<LoperService>>> = OnceCell::new();
        SERVICE.get_or_init(|| Arc::new(Mutex::new(LoperService::default())))
    }
}