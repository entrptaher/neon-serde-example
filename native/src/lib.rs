extern crate serde;
extern crate serde_json;

use neon::prelude::*;
use neon_serde;

#[macro_use]
extern crate serde_derive;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Link {
    page_key: String,
    group_key: String,
    stuff: Value,
}

fn hello(mut cx: FunctionContext) -> JsResult<JsValue> {
    // read arguments and turn them into json
    let mut list = vec![];
    let arg0 = cx.argument::<JsValue>(0)?;
    let object: serde_json::Value = neon_serde::from_value(&mut cx, arg0)?;
    for (page_key, page) in object.as_object().unwrap().iter() {
        for (group_key, group) in page.as_object().unwrap().iter() {
            for stuff in group.as_array().unwrap() {
                // println!("{:#?}", stuff);
                let link = Link {
                    page_key: page_key.to_string(),
                    group_key: group_key.to_string(),
                    stuff: stuff.clone(),
                };
                list.push(link);
            }
        }
    }

    // do stuff with the object
    // println!("{:#?}", object);

    // return back to nodejs
    let result = neon_serde::to_value(&mut cx, &list)?;
    Ok(result)
    // Ok(cx.undefined().upcast())
}

register_module!(mut cx, { cx.export_function("hello", hello) });
