use neon::prelude::*;

mod tokio_runtime;
mod loper_service;
mod loper_service_api;
mod js_promise;
mod js_value;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    loper_service_api::export_functions(&mut cx)?;
    Ok(())
}