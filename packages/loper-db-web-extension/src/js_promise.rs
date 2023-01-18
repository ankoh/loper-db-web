use neon::prelude::*;
use std::future::Future;
use super::js_value::AsJsValue;

pub async fn create_promise<'a, Body, Value>(on_success: neon::handle::Root<neon::types::JsFunction>, on_error: neon::handle::Root<neon::types::JsFunction>, channel: neon::event::Channel, f: Body)
where
    Value: AsJsValue + Send + 'static,
    Body: Future<Output = Result<Value, String>> + Send + 'static,
{
    match f.await {
        Ok(value) => {
            channel.send(move |mut cx| {
                let args = vec![value.as_jsvalue(&mut cx)];
                let this = cx.undefined();
                on_success.into_inner(&mut cx).call(&mut cx, this, args)?;
                Ok(())
            });
        },
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