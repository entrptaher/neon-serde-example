// SERDE
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// NEON
extern crate neon;
use neon::prelude::*;

// STD
use rayon::prelude::*;
use hashbrown::HashMap;

// STRUCTS
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

struct BackgroundTask<'a> {
    buf: &'a [u8],
}

// FN
fn direct(mut cx: FunctionContext) -> JsResult<JsString> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = BackgroundTask { buf: &buf };

    let mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>> = serde_json::from_slice(&task.buf).unwrap();
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
    
    let str_data = serde_json::to_string(&list).unwrap();
    Ok(cx.string(str_data))
}

register_module!(mut cx, {
    cx.export_function("direct", direct);
    Ok(())
});