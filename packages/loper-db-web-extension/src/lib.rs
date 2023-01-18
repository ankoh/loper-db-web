use neon::prelude::*;

mod tokio_runtime;
mod loper_service;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    loper_service::export_functions(&mut cx)?;
    Ok(())
}