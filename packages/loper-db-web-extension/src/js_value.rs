use neon::prelude::*;

pub trait AsJsValue {
    fn as_jsvalue<'a, C: Context<'a>>(self, _: &mut C) -> Handle<'a, JsValue>;
}

impl AsJsValue for () {
    fn as_jsvalue<'a, C: Context<'a>>(self, c: &mut C) -> Handle<'a, JsValue> {
        c.undefined().upcast()
    }
}