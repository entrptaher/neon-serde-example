extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

pub mod hashmap;
pub mod objecttuple;

struct BackgroundTask {
    argument: usize,
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

impl Task for BackgroundTask {
    type Output = usize;
    type Error = String;
    type JsEvent = JsNumber;
    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let num = self.argument;
        Ok(fib(num as u64) as usize)
    }
    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        Ok(cx.number(result.unwrap() as f64))
    }
}

pub fn perform_async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let n = cx.argument::<JsNumber>(0)?.value() as usize;
    let cb = cx.argument::<JsFunction>(1)?;
    let task = BackgroundTask { argument: n };
    task.schedule(cb);
    Ok(cx.undefined())
}

register_module!(mut cx, {
    cx.export_function("perform_async_task", perform_async_task);

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
