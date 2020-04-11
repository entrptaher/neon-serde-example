extern crate neon;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate neon_serde;

use neon::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Link {
    page: String,
    group: String,
    name: String,
    index: usize,
}

#[derive(Serialize, Deserialize)]
struct NamedIndex {
    pub name: String,
    pub index: usize,
}
fn string(mut cx: FunctionContext) -> JsResult<JsValue> {
    let input: String = cx.argument::<JsString>(0)?.value();

    // let b: Handle<JsBuffer> = cx.argument(0)?;

    // let buffer = cx.borrow(&b, |data| {
    //     return data.as_slice::<u8>();
    // });

    let mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_str(&input).unwrap();

    let mut list: Vec<Link> = Vec::with_capacity(512);

    for (page, mut groups) in data.drain() {
        for (group, mut named_indexes) in groups.drain() {
            for NamedIndex { name, index } in named_indexes.drain(..) {
                let page = page.clone();
                let group = group.clone();
                list.push(Link {
                    page,
                    group,
                    name,
                    index,
                });
            }
        }
    }

    // return back to nodejs
    let result = neon_serde::to_value(&mut cx, &list)?;

    Ok(result)
}

fn buffer(mut cx: FunctionContext) -> JsResult<JsValue> {
    let b: Handle<JsBuffer> = cx.argument(0)?;

    let buffer = cx.borrow(&b, |data| {
        return data.as_slice::<u8>();
    });

    let mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_slice(&buffer).unwrap();

    let mut list: Vec<Link> = Vec::with_capacity(512);

    for (page, mut groups) in data.drain() {
        for (group, mut named_indexes) in groups.drain() {
            for NamedIndex { name, index } in named_indexes.drain(..) {
                let page = page.clone();
                let group = group.clone();
                list.push(Link {
                    page,
                    group,
                    name,
                    index,
                });
            }
        }
    }

    // return back to nodejs
    let result = neon_serde::to_value(&mut cx, &list)?;

    Ok(result)
}

register_module!(mut cx, {
    cx.export_function("string", string);
    cx.export_function("buffer", buffer);
    Ok(())
});
