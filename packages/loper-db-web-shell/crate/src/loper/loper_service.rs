use super::tokens::{JsScriptTokens, ScriptTokens};
use crate::arrow_reader::ArrowStreamReader;
use js_sys::Uint8Array;
use std::sync::Arc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "LoperServiceConnection")]
    pub type JsLoperServiceConnection;

    #[wasm_bindgen(catch, method, js_name = "disconnect")]
    async fn disconnect(this: &JsLoperServiceConnection) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_name = "runQuery")]
    async fn run_query(this: &JsLoperServiceConnection, text: &str) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "LoperServiceClient")]
    pub type JsLoperServiceClient;

    #[wasm_bindgen(catch, method, js_name = "getVersion")]
    async fn get_version(this: &JsLoperServiceClient) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_name = "connect")]
    async fn connect(this: &JsLoperServiceClient) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch, method, js_name = "tokenize")]
    async fn tokenize(this: &JsLoperServiceClient, text: &str) -> Result<JsValue, JsValue>;
}

pub struct LoperServiceClient {
    bindings: JsLoperServiceClient,
}

impl LoperServiceClient {
    /// Create an async DuckDB from bindings
    pub fn from_bindings(bindings: JsLoperServiceClient) -> Self {
        Self { bindings }
    }
    /// Get the DuckDB version
    pub async fn get_version(&self) -> Result<String, js_sys::Error> {
        Ok(self
            .bindings
            .get_version()
            .await?
            .as_string()
            .unwrap_or_else(|| "?".to_string()))
    }
    /// Tokenize a script text
    pub async fn tokenize(&self, text: &str) -> Result<ScriptTokens, js_sys::Error> {
        let tokens: JsScriptTokens = self.bindings.tokenize(text).await?.into();
        Ok(tokens.into())
    }
    /// Create a new connection
    pub async fn connect(selfm: Arc<RwLock<Self>>) -> Result<LoperServiceConnection, js_sys::Error> {
        let db = selfm.read().unwrap();
        let conn: JsLoperServiceConnection = db.bindings.connect().await?.into();
        Ok(LoperServiceConnection::new(conn))
    }
}

pub struct LoperServiceConnection {
    connection: JsLoperServiceConnection,
}

impl LoperServiceConnection {
    /// Create a client connection
    pub fn new(conn: JsLoperServiceConnection) -> Self {
        Self {
            connection: conn,
        }
    }
    /// Disconnect a connection
    pub async fn disconnect(&self) -> Result<(), js_sys::Error> {
        self.connection
            .disconnect()
            .await?;
        Ok(())
    }
    /// Run a query
    pub async fn run_query(
        &self,
        text: &str,
    ) -> Result<Vec<arrow::record_batch::RecordBatch>, js_sys::Error> {
        // Run the RPC
        let js_buffers: js_sys::Array = self
            .connection
            .run_query(text)
            .await?
            .into();

        // Copy into wasm memory 
        let mut buffers: Vec<Vec<u8>> = Vec::new();
        for js_buffer in js_buffers.iter() {
            let buffer: Uint8Array = js_buffer.into();
            buffers.push(buffer.to_vec());
        }
        if buffers.is_empty() {
            return Ok(Vec::new());
        }

        // Decode the arrow ipc stream
        let mut out = Vec::with_capacity(buffers.len() - 1);
        let mut reader = ArrowStreamReader::try_new(&buffers[0]).map_err(|e| js_sys::Error::new(&e.to_string()))?;
        for buffer in buffers.iter().skip(1) {
            if let Some(batch) = reader.maybe_next(buffer).map_err(|e| js_sys::Error::new(&e.to_string()))? {
                out.push(batch);
            }
        }
        Ok(out)
    }
}
