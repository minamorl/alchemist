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

#[derive(Debug)]
struct KeyValueSerializeOption {
    separator_key_value: u8,
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> KeyValue {
        KeyValue {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    fn serialize(&self, options: &KeyValueSerializeOption) -> Vec<u8> {
        let key_as_bytes = self.key.as_bytes().to_vec();
        let value_as_bytes = self.value.as_bytes().to_vec();
        vec![key_as_bytes, vec![options.separator_key_value], value_as_bytes].concat()
    }
    pub fn deserialize(bytes: Vec<u8>, options: &KeyValueSerializeOption) -> Result<Self, AlchemistError> {
        let mut after_separator = false;
        let mut key_bytes: Vec<u8> = vec![];
        let mut value_bytes: Vec<u8> = vec![];
        // Separate key and values by options.separator_key_value
        for x in bytes {
            if x == options.separator_key_value {
                after_separator = true;
                continue;
            }
            if after_separator {
                value_bytes.push(x)
            } else {
                key_bytes.push(x)
            }
        }
        let key = String::from_utf8(key_bytes);
        let value = String::from_utf8(value_bytes);
        if let Ok(key) = key {
            if let Ok(value) = value {
                return Ok(KeyValue {
                    key: key,
                    value: value
                })
            }
        };
        Err(AlchemistError::DeserializationFailed)
    }
}

fn main() {
    let kv = KeyValue::new("key", "value");
    let options = KeyValueSerializeOption {
        separator_key_value: 1,
    };
    let serialized = kv.serialize(&options);
    let deserialized = KeyValue::deserialize(serialized, &options).unwrap();
    assert_eq!(deserialized.key, "key");
    assert_eq!(deserialized.value, "value");
}
