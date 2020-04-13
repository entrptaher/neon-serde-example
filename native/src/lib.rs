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
use std::collections::HashMap;
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

fn direct(mut cx: FunctionContext) -> JsResult<JsString> {
    let buf = cx.argument::<JsBuffer>(0)?;
    let buf = cx.borrow(&buf, |data| data.as_slice().to_vec());
    let task = Buffer { buf: &buf };

    let mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>> =
        serde_json::from_slice(task.buf).unwrap();

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
    let str_data = serde_json::to_string(&list).unwrap();
    Ok(cx.string(str_data))
}

struct BackgroundTask {
    argument: usize
}
 

fn fib(n: u64) -> u64 {
    if n <= 1 { return 1 }
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
    cx.export_function("par_array", perform_async_task);
    cx.export_function("direct", direct);
    Ok(())
});
