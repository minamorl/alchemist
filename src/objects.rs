use utils::{u32tobytes, bytestou32};
use error::AlchemistError;
use traits::{Serializable, Deserializable};

const KEYVALUE_LENGTH_BYTE: usize = 4;

#[derive(Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl Serializable for KeyValue {
    fn serialize(&self) -> Result<Vec<u8>, AlchemistError> {
        let key_as_bytes = self.key.as_bytes().to_vec();
        let value_as_bytes = self.value.as_bytes().to_vec();
        let key_length_as_bytes = u32tobytes(key_as_bytes.len() as u32)?.to_vec();
        let value_length_as_bytes = u32tobytes(value_as_bytes.len() as u32)?.to_vec();

        Ok(vec![key_length_as_bytes,
                value_length_as_bytes,
                key_as_bytes,
                value_as_bytes]
                    .concat())
    }
}

impl Deserializable<KeyValue> for Vec<u8> {
    fn deserialize(&self) -> Result<KeyValue, AlchemistError> {
        let key_length = bytestou32(&self[0..KEYVALUE_LENGTH_BYTE])? as usize;
        let value_length = bytestou32(&self[KEYVALUE_LENGTH_BYTE..KEYVALUE_LENGTH_BYTE * 2])? as
                            usize;
        let key = String::from_utf8(self[KEYVALUE_LENGTH_BYTE * 2..
                                    KEYVALUE_LENGTH_BYTE * 2 + key_length]
                                            .to_vec())
                .map_err(|_| AlchemistError::DeserializationFailed)?;
        let value = String::from_utf8(self[KEYVALUE_LENGTH_BYTE * 2 + key_length..
                                        KEYVALUE_LENGTH_BYTE * 2 + key_length + value_length]
                                                .to_vec())
                .map_err(|_| AlchemistError::DeserializationFailed)?;
        Ok(KeyValue::new(&key, &value))
    }
}