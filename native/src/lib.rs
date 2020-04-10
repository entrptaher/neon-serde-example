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

export! {
    fn hello(input: String) -> String {
        let mut data: HashMap<String, HashMap<String, Vec<NamedIndex>>>
            = serde_json::from_str(&input).unwrap();

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

        let result = serde_json::to_string(&list).unwrap();
        return format!("{}", result);
    }
}
