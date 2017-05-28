#[macro_use] extern crate log;
extern crate env_logger;
extern crate byteorder;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;

#[derive(Debug)]
enum AlchemistError {
    DeserializationFailed,
}

impl fmt::Display for AlchemistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlchemistError::DeserializationFailed =>
                write!(f, "deserialization was failed"),
        }
    }
}

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

fn u32tobytes(v: u32) -> Vec<u8> {
    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(v).unwrap();
    wtr
}

fn bytestou32(v: &[u8]) -> u32 {
    let mut rdr = Cursor::new(v);
    rdr.read_u32::<BigEndian>().unwrap()
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> KeyValue {
        KeyValue {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    fn serialize(&self) -> Vec<u8> {
        let key_as_bytes = self.key.as_bytes().to_vec();
        let value_as_bytes = self.value.as_bytes().to_vec();
        vec![
            u32tobytes(key_as_bytes.len() as u32).to_vec(),
            u32tobytes(value_as_bytes.len() as u32).to_vec(),
            key_as_bytes,
            value_as_bytes,
        ].concat()
    }
    pub fn deserialize(bytes: &[u8]) -> Result<Self, AlchemistError> {
        let key_length = bytestou32(&bytes[0..4]) as usize;
        let value_length = bytestou32(&bytes[4..8]) as usize;
        let key = String::from_utf8(bytes[8 .. 8 + key_length].to_vec())
            .map_err(|_| AlchemistError::DeserializationFailed)?;
        let value = String::from_utf8(bytes[8 + key_length .. 8 + key_length + value_length].to_vec())
            .map_err(|_| AlchemistError::DeserializationFailed)?;
        Ok(Self::new(&key, &value))
    }
}

fn main() {
    env_logger::init().unwrap();

    info!("Create KeyValue instance");
    let kv = KeyValue::new("キー", "ヴァリュー");
    let serialized = kv.serialize();
    let deserialized = KeyValue::deserialize(&serialized).unwrap();
    info!("Assertion with deserialized.key");
    assert_eq!(deserialized.key, "キー");
    info!("Assertion with deserialized.value");
    assert_eq!(deserialized.value, "ヴァリュー");
}
