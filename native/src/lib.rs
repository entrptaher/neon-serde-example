extern crate neon;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate neon_serde;

use neon::prelude::*;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    page_key: String,
    group_key: String,
    stuff: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stuff {
    name: String,
    index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    identifier: String,
    data: Vec<Stuff>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Page {
    identifier: String,
    data: Vec<Group>,
}

use rayon::prelude::*;

export! {
    fn string(input: String) -> String {
        // let list = vec![""];
        let mut list: Vec<Link> = vec![];
        let object: Vec<Page> = serde_json::from_str(&input).unwrap();
        // println!("{:#?}", object);
        for page in object.iter(){
            for group in page.data.iter(){
                for stuff in group.data.iter(){
                    let link = Link {
                        page_key: page.identifier.to_string(),
                        group_key: group.identifier.to_string(),
                        stuff: serde_json::to_value(stuff).unwrap(),
                    };
                    list.push(link);
                }
            }
        }
        let result = serde_json::to_string(&list).unwrap();
        return format!("{}", result);
        // return "{\"hello\": 123}".to_string();
    }
}
