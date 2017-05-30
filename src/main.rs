#[macro_use] extern crate log;
extern crate env_logger;
extern crate byteorder;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::fmt;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

const KEYVALUE_LENGTH_BYTE: usize = 4;

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

fn u32tobytes(v: u32) -> Result<Vec<u8>, AlchemistError> {
    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(v).map_err(|_| AlchemistError::DeserializationFailed)?;
    Ok(wtr)
}

fn bytestou32(v: &[u8]) -> Result<u32, AlchemistError> {
    let mut rdr = Cursor::new(v);
    rdr.read_u32::<BigEndian>().map_err(|_| AlchemistError::DeserializationFailed)
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    fn serialize(&self) -> Result<Vec<u8>, AlchemistError> {
        let key_as_bytes = self.key.as_bytes().to_vec();
        let value_as_bytes = self.value.as_bytes().to_vec();
        let key_length_as_bytes = u32tobytes(key_as_bytes.len() as u32)?.to_vec();
        let value_length_as_bytes = u32tobytes(value_as_bytes.len() as u32)?.to_vec();

        Ok(vec![
            key_length_as_bytes,
            value_length_as_bytes,
            key_as_bytes,
            value_as_bytes,
        ].concat())
    }
    pub fn deserialize(bytes: &[u8]) -> Result<Self, AlchemistError> {
        let key_length = bytestou32(&bytes[0..KEYVALUE_LENGTH_BYTE])? as usize;
        let value_length = bytestou32(&bytes[KEYVALUE_LENGTH_BYTE..KEYVALUE_LENGTH_BYTE * 2])? as usize;
        let key = String::from_utf8(bytes[KEYVALUE_LENGTH_BYTE * 2 .. KEYVALUE_LENGTH_BYTE * 2 + key_length].to_vec())
            .map_err(|_| AlchemistError::DeserializationFailed)?;
        let value = String::from_utf8(bytes[KEYVALUE_LENGTH_BYTE * 2 + key_length .. KEYVALUE_LENGTH_BYTE * 2 + key_length + value_length].to_vec())
            .map_err(|_| AlchemistError::DeserializationFailed)?;
        Ok(Self::new(&key, &value))
    }
}

#[derive(Debug)]
struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn create(listener: TcpListener) -> Self {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("Received new connection");
                    stream.write("Hello world".as_bytes());
                }
                Err(e) => { /* connection failed */ }
            }
        };
        Self {
            listener: listener,
        }
    }
}

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

    let server = Server::create(TcpListener::bind("127.0.0.1:8080").unwrap());
    let listner = server.listener;
}
