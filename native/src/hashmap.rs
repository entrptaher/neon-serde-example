use neon::prelude::*;
use std::collections::HashMap;
// use rayon::prelude::*;
// use std::collections::BTreeMap;

// STRUCTS
#[derive(Serialize, Deserialize)]
pub struct Link {
    page: String,
    group: String,
    name: String,
    index: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NamedIndex {
    pub name: String,
    pub index: usize,
}

struct Buffer<'a> {
    buf: &'a [u8],
}

pub fn hash_map(mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>>) -> Vec<Link> {
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
    list
}

pub fn buffer_neon_value(mut cx: FunctionContext) -> JsResult<JsValue> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = Buffer { buf: &buf };
    let data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_slice(task.buf).unwrap();
    let list = hash_map(data);
    let js_value = neon_serde::to_value(&mut cx, &list)?;
    Ok(js_value)
}

pub fn buffer_serde_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = Buffer { buf: &buf };
    let data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_slice(task.buf).unwrap();
    let list = hash_map(data);
    let str_data = serde_json::to_string(&list).unwrap();
    Ok(cx.string(str_data))
}

pub fn string_serde_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let input: String = cx.argument::<JsString>(0)?.value();
    let data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_str(&input).unwrap();
    let list = hash_map(data);
    let str_data = serde_json::to_string(&list).unwrap();
    Ok(cx.string(str_data))
}

pub fn hello(input: String) -> String {
    let data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_str(&input).unwrap();
    let list = hash_map(data);
    serde_json::to_string(&list).unwrap()
}