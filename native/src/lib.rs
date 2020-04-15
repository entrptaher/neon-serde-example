extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

pub mod hashmap;
pub mod objecttuple;

struct BackgroundTask {
    argument: String,
}

impl Task for BackgroundTask {
    type Output = String;
    type Error = String;
    type JsEvent = JsString;
    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let input = self.argument.to_string();
        Ok(hashmap::hello(input))
    }
    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        Ok(cx.string(result.unwrap()))
    }
}

pub fn perform_async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsString>(0)?.value() as String;
    let cb = cx.argument::<JsFunction>(1)?;
    let task = BackgroundTask { argument: n };
    task.schedule(cb);
    Ok(cx.undefined())
}

pub fn perform_sync_task(mut cx: FunctionContext) -> JsResult<JsString> {
    let n = cx.argument::<JsString>(0)?.value() as String;
    let task = hashmap::hello(n);
    Ok(cx.string(task))
}

register_module!(mut cx, {
    cx.export_function("perform_async_task", perform_async_task);
    cx.export_function("perform_sync_task", perform_sync_task);

    cx.export_function("hashmap_buffer_neon_value", hashmap::buffer_neon_value)
        .expect("export function");
    cx.export_function("hashmap_buffer_serde_string", hashmap::buffer_serde_string)
        .expect("export function");
    cx.export_function("hashmap_string_serde_string", hashmap::string_serde_string)
        .expect("export function");
    cx.export_function(
        "objecttuple_string_neon_value",
        objecttuple::string_neon_value,
    )
    .expect("export function");
    cx.export_function(
        "objecttuple_string_serde_string",
        objecttuple::string_serde_string,
    )
    .expect("export function");
    Ok(())
});
