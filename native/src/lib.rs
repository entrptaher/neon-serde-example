extern crate neon;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate neon_serde;

use neon::prelude::*;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Link {
    page_key: String,
    group_key: String,
    stuff: Value,
}

export! {
    fn hello(input: String) -> String {
        let mut list = vec![];
        let object: Value = serde_json::from_str(&input).unwrap();
        for (page_key, page) in object.as_object().unwrap().iter() {
            for (group_key, group) in page.as_object().unwrap().iter() {
                for stuff in group.as_array().unwrap() {
                    let link = Link {
                        page_key: page_key.to_string(),
                        group_key: group_key.to_string(),
                        stuff: stuff.clone(),
                    };
                    list.push(link);
                }
            }
        }
        let result = serde_json::to_string(&list).unwrap();
        return format!("{}", result);
    }
}
