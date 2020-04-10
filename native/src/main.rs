#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::time::Instant;
use std::fs::read_to_string; // use instead of std::fs::File
use std::path::Path;

mod hello;

fn main(){
    let json_file_path = Path::new("src/sample.json");
    let input = read_to_string(json_file_path).expect("file not found");
    hello::hello(input);
    // println!("{:#?}", hello::hello(input));
}