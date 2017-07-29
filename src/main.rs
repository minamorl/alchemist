#[macro_use]
extern crate log;
extern crate env_logger;
extern crate byteorder;

mod objects;
mod utils;
mod error;
mod traits;

use objects::{KeyValue, PrimitiveType};
use traits::{Serializable, Deserializable};

fn main() {
    env_logger::init().unwrap();

    info!("Create KeyValue instance");
    let kv = KeyValue::new("キー", PrimitiveType::Str("ヴァリュー".to_string()));
    let serialized = kv.serialize().unwrap();
    let deserialized = serialized.deserialize().unwrap();
       
    info!("Assertion with deserialized.key");
    assert_eq!(deserialized.key, "キー");
    info!("Assertion with deserialized.value");
    match deserialized.value {
        PrimitiveType::Str(x) => assert_eq!(x, "ヴァリュー")
    };
}
