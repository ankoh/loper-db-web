use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

struct LoperService {}

impl LoperService {
    pub fn configure(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        Ok(cx.undefined())
    }
    pub fn open_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        Ok(cx.undefined())
    }
    pub fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        Ok(cx.undefined())
    }
    pub fn create_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        Ok(cx.undefined())
    }
    pub fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        Ok(cx.undefined())
    }
    pub fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        Ok(cx.undefined())
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("loper_service_configure", LoperService::configure)?;
    cx.export_function("loper_service_open_connection", LoperService::open_connection)?;
    cx.export_function("loper_service_close_connection", LoperService::close_connection)?;
    cx.export_function("loper_service_create_session", LoperService::create_session)?;
    cx.export_function("loper_service_close_session", LoperService::close_session)?;
    cx.export_function("loper_service_execute_query", LoperService::execute_query)?;
    Ok(())
}