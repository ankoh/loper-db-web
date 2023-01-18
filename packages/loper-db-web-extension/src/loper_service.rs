use std::{collections::HashMap, cell::RefCell};
use neon::prelude::*;
use std::future::Future;
use super::tokio_runtime::scheduler;

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

    fn configure(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let on_success = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Configure everything

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn open_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let on_success = cx.argument::<JsFunction>(0)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Open the connection

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Close the session

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn create_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _connection_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Allocate the session id
            let session_id = 42;

            // Send the result
            channel.send(move |mut cx| {
                let descriptor = cx.empty_object();
                let session_id = cx.number(session_id);
                descriptor.set(&mut cx, "sessionId", session_id).unwrap();
                let args = vec![descriptor.upcast()];
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, args)?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _session_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Close the session

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _session_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let _query_text = cx.argument::<JsString>(0)?.value(&mut cx);
        let on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Execute a query

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn read_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _stream_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Read from the result stream

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
    fn close_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let _stream_id = cx.argument::<JsNumber>(0)?.value(&mut cx);
        let on_success = cx.argument::<JsFunction>(1)?.root(&mut cx);
        let on_error = cx.argument::<JsFunction>(2)?.root(&mut cx);
        let channel = cx.channel();
        scheduler(&mut cx)?.spawn(catch_error(on_error, channel.clone(), async move {
            // XXX Close the result stream

            // Send the result
            channel.send(move |mut cx| {
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, vec![])?;
                Ok(())
            });
            Ok(())
        }));
        Ok(cx.undefined())
    }
}

async fn catch_error<F>(on_error: neon::handle::Root<neon::types::JsFunction>, channel: neon::event::Channel, f: F)
where
    F: Future<Output = Result<(), String>> + Send + 'static,
    F::Output: Send + 'static,
{
    match f.await {
        Ok(()) => (),
        Err(e) => {
            channel.send(|mut cx| {
                let args = vec![cx.string(e).upcast()];
                let this = cx.undefined();
                on_error.into_inner(&mut cx).call(&mut cx, this, args).unwrap();
                Ok(())
            });
        }
    }
}