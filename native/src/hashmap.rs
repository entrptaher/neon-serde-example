use neon::prelude::*;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

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
fn get_par_indexes(page: &String, group: &String, named_indexes: &Vec<NamedIndex>) -> Vec<Link> {
    println!("named_indexes.len(): {}", named_indexes.len());
    let mut list: Vec<JsValue> = Vec::with_capacity(512);
    named_indexes
        .into_par_iter()
        .map(|x| Link {
            page: page.clone(),
            group: group.clone(),
            name: x.name.clone(),
            index: x.index.clone(),
        })
        .collect()
}

fn get_indexes(page: &String, group: &String, named_indexes: &Vec<NamedIndex>) -> Vec<Link> {
    println!("named_indexes.len(): {}", named_indexes.len());
    let mut list: Vec<JsValue> = Vec::with_capacity(512);
    named_indexes
        .into_iter()
        .map(|x| Link {
            page: page.clone(),
            group: group.clone(),
            name: x.name.clone(),
            index: x.index.clone(),
        })
        .collect()
}

use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn hash_map(buffer: Buffer) -> Vec<Link> {
    let mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_slice(buffer.buf).unwrap();

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

// for (page, groups) in data.iter() {
//     for (group, named_indexes) in groups.iter() {

// list.extend(get_indexes(page, group, named_indexes))
// list.par_extend(named_indexes.into_par_iter().map(|x| Link {
//     page: page.clone(),
//     group: group.clone(),
//     name: x.name.clone(),
//     index: x.index.clone(),
// }))
//     }
// }
// let js_value = neon_serde::to_value(&mut cx, &list)?;
// Ok(js_value)
pub fn buffer_neon_value(mut cx: FunctionContext) -> JsResult<JsValue> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = Buffer { buf: &buf };
    let list = hash_map(task);
    let js_value = neon_serde::to_value(&mut cx, &list)?;
    Ok(js_value)
}

pub fn buffer_serde_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = Buffer { buf: &buf };
    let list = hash_map(task);
    let str_data = serde_json::to_string(&list).unwrap();
    Ok(cx.string(str_data))
}