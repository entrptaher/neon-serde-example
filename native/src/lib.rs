extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use neon::prelude::*;

pub mod hashmap;
pub mod objecttuple;

register_module!(mut cx, {
    cx.export_function("hashmap_buffer_neon_value", hashmap::buffer_neon_value);
    cx.export_function("hashmap_buffer_serde_string", hashmap::buffer_serde_string);
    cx.export_function("objecttuple_string_neon_value", objecttuple::string_neon_value);
    cx.export_function("objecttuple_string_serde_string", objecttuple::string_serde_string);
    Ok(())
});
