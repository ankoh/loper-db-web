use crate::grpc_client::{GrpcClient, SlotId};

use super::js_promise::spawn_promise;
use neon::{prelude::*, types::buffer::TypedArray};
use tonic::Request;

pub fn export_functions(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("grpc_create_channel", grpc_create_channel)?;
    cx.export_function("grpc_close_channel", grpc_close_channel)?;
    cx.export_function("grpc_call_unary", grpc_call_unary)?;
    cx.export_function("grpc_call_server_stream", grpc_call_server_stream)?;
    cx.export_function("grpc_call_client_stream", grpc_call_client_stream)?;
    cx.export_function("grpc_call_with_bidi_stream", grpc_call_bidi_stream)?;
    cx.export_function("grpc_read_server_stream", grpc_read_server_stream)?;
    Ok(())
}

fn grpc_create_channel(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _url = cx.argument::<JsString>(2)?.value(&mut cx);
    spawn_promise(cx, async move { Ok(()) })
}

fn grpc_close_channel(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let _channel_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    spawn_promise(cx, async move { Ok(()) })
}

fn grpc_call_unary(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let param = cx.argument::<JsArrayBuffer>(2)?;
    let _param_owned = param.as_slice(&cx).to_vec();
    spawn_promise(cx, async move { Ok(()) })
}

fn grpc_call_server_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let path = cx.argument::<JsString>(3)?.value(&mut cx);
    let param = cx.argument::<JsArrayBuffer>(4)?;
    let param_owned = param.as_slice(&cx).to_vec();
    spawn_promise(cx, async move {
        let request = Request::new(param_owned);
        let stream_id = GrpcClient::call_server_stream(channel_id as SlotId, path, request).await?;
        Ok(stream_id)
    })
}

fn grpc_call_client_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let param = cx.argument::<JsArrayBuffer>(2)?;
    let _param_owned = param.as_slice(&cx).to_vec();
    spawn_promise(cx, async move { Ok(()) })
}

fn grpc_call_bidi_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let param = cx.argument::<JsArrayBuffer>(2)?;
    let _param_owned = param.as_slice(&cx).to_vec();
    spawn_promise(cx, async move { Ok(()) })
}

fn grpc_read_server_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
    let stream_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
    spawn_promise(cx, async move {
        let (_events, _done) =
            GrpcClient::read_server_stream(channel_id as SlotId, stream_id as SlotId).await?;
        Ok(())
    })
}

// fn grpc_(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let url = cx.argument::<JsString>(2)?.value(&mut cx);
//     spawn_promise(cx, async move {
//         let _client = HyperDatabaseServiceClient::connect(url)
//             .await
//             .map_err(|e| e.to_string())?;
//         // XXX Open a connection
//         Ok(())
//     })
// }
// fn close_connection(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
//     spawn_promise(cx, async move {
//         // XXX Close a connection
//         Ok(())
//     })
// }
// fn create_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
//     spawn_promise(cx, async move {
//         // XXX Create a session
//         Ok(())
//     })
// }
// fn close_session(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
//     let _session_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
//     spawn_promise(cx, async move {
//         // XXX Close the session
//         Ok(())
//     })
// }
// fn execute_query(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx) as SlotId;
//     let session_id = cx.argument::<JsNumber>(3)?.value(&mut cx) as SlotId;
//     let query_text = cx.argument::<JsString>(4)?.value(&mut cx);
//     spawn_promise(cx, async move {
//         let query_id = LoperService::execute_query(connection_id, session_id, query_text).await?;
//         Ok(query_id)
//     })
// }
// fn read_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx) as SlotId;
//     let session_id = cx.argument::<JsNumber>(3)?.value(&mut cx) as SlotId;
//     let query_id = cx.argument::<JsNumber>(4)?.value(&mut cx) as SlotId;
//     spawn_promise(cx, async move {
//         let result =
//             LoperService::read_query_result_stream(connection_id, session_id, query_id).await?;
//         Ok(result)
//     })
// }
// fn close_query_result_stream(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let _connection_id = cx.argument::<JsNumber>(2)?.value(&mut cx);
//     let _session_id = cx.argument::<JsNumber>(3)?.value(&mut cx);
//     let _query_id = cx.argument::<JsNumber>(4)?.value(&mut cx);
//     spawn_promise(cx, async move {
//         // XXX Close a query result stream
//         Ok(())
//     })
// }