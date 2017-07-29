use utils::{u32tobytes, bytestou32};
use error::AlchemistError;
use traits::{Serializable, Deserializable};
use std::string;
use std::marker::PhantomData;
const KEYVALUE_LENGTH_BYTE: usize = 4;
const VALUE_TYPE_LENGTH_BYTE: usize = 1;


#[derive(Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: PrimitiveType,
}

impl KeyValue {
    pub fn new(key: &str, value: PrimitiveType) -> Self {
        Self {
            key: key.to_string(),
            value,
        }
    }
}

#[derive(Debug)]
pub struct KeyValueHeader {
  key_length: usize,
  value_length: usize,
  value_type: u8,
}

impl KeyValueHeader {
  pub fn parse(bytes: &Vec<u8>) -> Result<Self, AlchemistError> {
    let key_length = bytestou32(&bytes[0..KEYVALUE_LENGTH_BYTE])? as usize;
    let value_length = bytestou32(&bytes[KEYVALUE_LENGTH_BYTE..KEYVALUE_LENGTH_BYTE * 2])? as
                        usize;
    Ok(Self {
      key_length,
      value_length,
      value_type: 0,
    })
  }
}

impl Serializable for KeyValue {
    fn serialize(&self) -> Result<Vec<u8>, AlchemistError> {
        let key_as_bytes = self.key.as_bytes().to_vec();
        let value_as_bytes = self.value.raw();
        let key_length_as_bytes = u32tobytes(key_as_bytes.len() as u32)?.to_vec();
        let value_length_as_bytes = u32tobytes(value_as_bytes.len() as u32)?.to_vec();

        Ok(vec![key_length_as_bytes,
                value_length_as_bytes,
                key_as_bytes,
                value_as_bytes]
                    .concat())
    }
}

#[derive(Debug)]
pub enum PrimitiveType {
  Str(String),
}

impl PrimitiveType {
  pub fn raw(&self) -> Vec<u8> {
    match *self {
      PrimitiveType::Str(ref x) => x.as_bytes().to_vec()
    }
  }
}

impl Deserializable<KeyValue> for Vec<u8> {
    fn deserialize(&self) -> Result<KeyValue, AlchemistError> {
        let header = KeyValueHeader::parse(self)?;
        let key = String::from_utf8(self[KEYVALUE_LENGTH_BYTE * 2..
                                    KEYVALUE_LENGTH_BYTE * 2 + header.key_length]
                                            .to_vec())
                .map_err(|_| AlchemistError::DeserializationFailed)?;
        let raw_value = self[KEYVALUE_LENGTH_BYTE * 2 + header.key_length..
                                        KEYVALUE_LENGTH_BYTE * 2 + header.key_length + header.value_length]
                                                .to_vec();
        match header.value_type {
          _ => {
              Ok(KeyValue::new(&key, PrimitiveType::Str(
              String::from_utf8(raw_value).map_err(|_| AlchemistError::DeserializationFailed)?)))
            }
        }
    }
}