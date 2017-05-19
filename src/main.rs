const SEPARATOR_KEY_VALUE: u8 = 1;

#[derive(Debug)]
struct KeyValue {
    key: String,
    value: String,
}

impl KeyValue {
    pub fn new(key: &str, value: &str) -> KeyValue {
        KeyValue {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    fn serialize(self) -> Vec<u8> {
        let key_as_bytes = self.key.as_bytes().to_vec();
        let value_as_bytes = self.value.as_bytes().to_vec();
        vec![key_as_bytes, vec![SEPARATOR_KEY_VALUE], value_as_bytes].concat()
    }
    pub fn deserialize(bytes: Vec<u8>) -> Self {
        let mut after_separator = false;
        let mut key_bytes: Vec<u8> = vec![];
        let mut value_bytes: Vec<u8> = vec![];
        // Separate key and values by SEPARATOR_KEY_VALUE
        for x in bytes {
            if x == SEPARATOR_KEY_VALUE {
                after_separator = true;
                continue;
            }
            if after_separator {
                value_bytes.push(x)
            } else {
                key_bytes.push(x)
            }
        }
        KeyValue {
            key: String::from_utf8(key_bytes).unwrap(),
            value: String::from_utf8(value_bytes).unwrap(),
        }
    }
}

fn main() {
    let kv = KeyValue::new("key", "value");
    let serialized = kv.serialize();
    let deserialized = KeyValue::deserialize(serialized);
    assert_eq!(deserialized.key, "key");
    assert_eq!(deserialized.value, "value");
}
