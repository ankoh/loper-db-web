use std::{collections::HashMap, cell::RefCell};
use neon::prelude::*;

struct LoperServiceConnection {}
struct LoperServiceSession {}
struct LoperServiceQueryRunner {}

#[derive(Default)]
struct LoperService {
    connections: HashMap<usize, LoperServiceConnection>,
    sessions: HashMap<usize, LoperServiceSession>,
    queries: HashMap<usize, LoperServiceQueryRunner>,
}

impl LoperService {
    /// We prevent leaking service state refs by returning mut refs here.
    /// Users should resolve client state multiple times instead of introducing unnecessary state synchronization.
    fn with_mut<F, R>(f: F) -> R
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

    pub fn configure(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _on_success = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let _service_info = cx.empty_object();

        Ok(cx.undefined())
    }
    pub fn open_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _on_success = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(1)?.root(&mut cx);

        let _connection_descriptor = cx.empty_object();

        Ok(cx.undefined())
    }
    pub fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        Ok(cx.undefined())
    }
    pub fn create_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);

        let _session_descriptor = cx.empty_object();

        Ok(cx.undefined())
    }
    pub fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _session_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);

        Ok(cx.undefined())
    }
    pub fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _session_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);

        Ok(cx.undefined())
    }
    pub fn read_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _stream_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);

        Ok(cx.undefined())
    }
    pub fn close_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _stream_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let _on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);

        Ok(cx.undefined())
    }
}


pub fn export_functions(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("loper_configure", LoperService::configure)?;
    cx.export_function("loper_open_connection", LoperService::open_connection)?;
    cx.export_function("loper_close_connection", LoperService::close_connection)?;
    cx.export_function("loper_create_session", LoperService::create_session)?;
    cx.export_function("loper_close_session", LoperService::close_session)?;
    cx.export_function("loper_execute_query", LoperService::execute_query)?;
    cx.export_function("loper_read_query_result_stream", LoperService::read_query_result_stream)?;
    cx.export_function("loper_close_query_result_stream", LoperService::close_query_result_stream)?;
    Ok(())
}