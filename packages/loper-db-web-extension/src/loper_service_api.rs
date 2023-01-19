use loper_db_proto_rs::hyper_database_service_client::HyperDatabaseServiceClient;
use super::js_promise::spawn_promise;
use neon::prelude::*;

pub fn export_functions(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("loper_configure", configure)?;
    cx.export_function("loper_open_connection", open_connection)?;
    cx.export_function("loper_close_connection", close_connection)?;
    cx.export_function("loper_create_session", create_session)?;
    cx.export_function("loper_close_session", close_session)?;
    cx.export_function("loper_execute_query", execute_query)?;
    cx.export_function(
        "loper_read_query_result_stream",
        read_query_result_stream,
    )?;
    cx.export_function(
        "loper_close_query_result_stream",
        close_query_result_stream,
    )?;
    Ok(())
}

fn configure(cx: FunctionContext) -> JsResult<JsUndefined> {
    spawn_promise(cx, async move {
        // XXX Configure everything
        Ok(())
    })
}
fn open_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let url = cx.argument::<JsString>(2)?.value(&mut cx);
    spawn_promise(cx, async move {
        let _client = HyperDatabaseServiceClient::connect(url).await.map_err(|e| e.to_string())?;
        // XXX Open a connection
        Ok(())
    })
}
fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    spawn_promise(cx, async move {
        // XXX Close a connection
        Ok(())
    })
}
fn create_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    spawn_promise(cx, async move {
        // XXX Create a session
        Ok(())
    })
}
fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let _session_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
    spawn_promise(cx, async move {
        // XXX Close the session
        Ok(())
    })
}
fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let _session_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let _query_id = cx.argument::<JsNumber>(4)?.value(&mut cx);
    let _query_text = cx.argument::<JsString>(5)?.value(&mut cx);
    spawn_promise(cx, async move {
        // XXX Execute a query
        Ok(())
    })
}
fn read_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let _session_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let _query_id = cx.argument::<JsNumber>(4)?.value(&mut cx);
    spawn_promise(cx, async move {
        // XXX Read a query result stream
        Ok(())
    })
}
fn close_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let _session_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
    let _query_id = cx.argument::<JsNumber>(4)?.value(&mut cx);
    spawn_promise(cx, async move {
        // XXX Close a query result stream
        Ok(())
    })
}