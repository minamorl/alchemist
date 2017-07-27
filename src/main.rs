#[macro_use]
extern crate log;
extern crate env_logger;
extern crate byteorder;

mod objects;
mod utils;
mod error;

use objects::{KeyValue};


fn main() {
    env_logger::init().unwrap();

    info!("Create KeyValue instance");
    let kv = KeyValue::new("キー", "ヴァリュー");
    let serialized = kv.serialize().unwrap();
    let deserialized = KeyValue::deserialize(&serialized).unwrap();
    info!("Assertion with deserialized.key");
    assert_eq!(deserialized.key, "キー");
    info!("Assertion with deserialized.value");
    assert_eq!(deserialized.value, "ヴァリュー");
}
