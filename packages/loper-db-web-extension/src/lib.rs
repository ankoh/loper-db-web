use neon::prelude::*;

struct LoperServiceClient {}

impl LoperServiceClient {
    pub fn configure(mut cx: FunctionContext) -> JsResult<JsObject> {
        let service_info = cx.empty_object();
        Ok(service_info)
    }
    pub fn open_connection(mut cx: FunctionContext) -> JsResult<JsObject> {
        let connection_descriptor = cx.empty_object();
        Ok(connection_descriptor)
    }
    pub fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        Ok(cx.undefined())
    }
    pub fn create_session(mut cx: FunctionContext) -> JsResult<JsObject> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let session_descriptor = cx.empty_object();
        Ok(session_descriptor)
    }
    pub fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(1)?.value(&mut cx);
        Ok(cx.undefined())
    }
    pub fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(1)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        Ok(cx.undefined())
    }
    pub fn read_query_result_stream(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(1)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        let _stream_id = cx.argument::<JsNumber>(3)?.value(&mut cx);

        Ok(JsArrayBuffer::new(&mut cx, 1)?)
    }
    pub fn close_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(1)?.value(&mut cx);
        let _session_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        let _stream_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
        Ok(cx.undefined())
    }
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("loper_configure", LoperServiceClient::configure)?;
    cx.export_function("loper_open_connection", LoperServiceClient::open_connection)?;
    cx.export_function("loper_close_connection", LoperServiceClient::close_connection)?;
    cx.export_function("loper_create_session", LoperServiceClient::create_session)?;
    cx.export_function("loper_close_session", LoperServiceClient::close_session)?;
    cx.export_function("loper_execute_query", LoperServiceClient::execute_query)?;
    cx.export_function("loper_read_query_result_stream", LoperServiceClient::read_query_result_stream)?;
    cx.export_function("loper_close_query_result_stream", LoperServiceClient::close_query_result_stream)?;
    Ok(())
}