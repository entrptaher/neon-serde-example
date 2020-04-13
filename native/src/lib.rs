// SERDE
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// NEON
extern crate neon;
use neon::prelude::*;

// STD
use hashbrown::HashMap;
use rayon::prelude::*;
use std::collections::BTreeMap;

// STRUCTS
#[derive(Serialize, Deserialize)]
struct Link {
    page: String,
    group: String,
    name: String,
    index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct NamedIndex {
    pub name: String,
    pub index: usize,
}

struct Buffer<'a> {
    buf: &'a [u8],
}

// FN
fn get_indexes(page: &String, group: &String, named_indexes: &Vec<NamedIndex>) -> Vec<Link> {
    named_indexes.into_par_iter().map(|x| Link {
        page: page.clone(),
        group: group.clone(),
        name: x.name.clone(),
        index: x.index.clone(),
    }).collect()
}

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn direct(mut cx: FunctionContext) -> JsResult<JsString> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = Buffer { buf: &buf };

    let data: BTreeMap<String, BTreeMap<String, Vec<NamedIndex>>> =
        serde_json::from_slice(task.buf).unwrap();

    let mut list: Vec<Link> = Vec::with_capacity(512);
    for (page, groups) in data.iter() {
        for (group, named_indexes) in groups.iter() {
            list.extend(get_indexes(
                page,
                group,
                named_indexes,
            ))
        }
    }
    // let js_value = neon_serde::to_value(&mut cx, &list)?;
    // Ok(js_value)
    let str_data = serde_json::to_string(&list).unwrap();
    Ok(cx.string(str_data))
}

register_module!(mut cx, {
    cx.export_function("direct", direct);
    Ok(())
});
