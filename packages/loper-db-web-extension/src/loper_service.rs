use super::js_promise::create_promise;
use neon::prelude::*;
use std::{cell::RefCell, collections::HashMap};

pub fn export_functions(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("loper_configure", LoperService::configure)?;
    cx.export_function("loper_open_connection", LoperService::open_connection)?;
    cx.export_function("loper_close_connection", LoperService::close_connection)?;
    cx.export_function("loper_create_session", LoperService::create_session)?;
    cx.export_function("loper_close_session", LoperService::close_session)?;
    cx.export_function("loper_execute_query", LoperService::execute_query)?;
    cx.export_function(
        "loper_read_query_result_stream",
        LoperService::read_query_result_stream,
    )?;
    cx.export_function(
        "loper_close_query_result_stream",
        LoperService::close_query_result_stream,
    )?;
    Ok(())
}

#[derive(Default)]
struct LoperService {
    _connections: HashMap<usize, LoperServiceConnection>,
    _sessions: HashMap<usize, LoperServiceSession>,
    _queries: HashMap<usize, LoperServiceQueryRunner>,
}

struct LoperServiceConnection {}
struct LoperServiceSession {}
struct LoperServiceQueryRunner {}

impl LoperService {
    /// We prevent leaking service state refs by returning mut refs here.
    /// Users should resolve client state multiple times instead of introducing unnecessary state synchronization.
    fn _with_mut<F, R>(f: F) -> R
    where
        F: FnOnce(&mut LoperService) -> R,
    {
        thread_local! {
            static SERVICE: RefCell<LoperService> = RefCell::new(LoperService::default());
        }
        SERVICE.with(|cell| {
            let mut ref_guard = cell.borrow_mut();
            f(&mut ref_guard)
        })
    }

    fn configure(cx: FunctionContext) -> JsResult<JsUndefined> {
        create_promise(cx, async move {
            // XXX Configure everything
            Ok(())
        })
    }
    fn open_connection(cx: FunctionContext) -> JsResult<JsUndefined> {
        create_promise(cx, async move {
            // XXX Open a connection
            Ok(())
        })
    }
    fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        create_promise(cx, async move {
            // XXX Close a connection
            Ok(())
        })
    }
    fn create_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        create_promise(cx, async move {
            // XXX Create a session
            Ok(())
        })
    }
    fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _session_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        create_promise(cx, async move {
            // XXX Close the session
            Ok(())
        })
    }
    fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _session_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        let _query_text = cx.argument::<JsString>(3)?.value(&mut cx);
        create_promise(cx, async move {
            // XXX Execute a query
            Ok(())
        })
    }
    fn read_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _stream_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        create_promise(cx, async move {
            // XXX Read a query result stream
            Ok(())
        })
    }
    fn close_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _stream_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
        create_promise(cx, async move {
            // XXX Close a query result stream
            Ok(())
        })
    }
}
