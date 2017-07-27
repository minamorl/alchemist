use std::fmt;

#[derive(Debug)]
pub enum AlchemistError {
    DeserializationFailed,
}

impl fmt::Display for AlchemistError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlchemistError::DeserializationFailed => write!(f, "deserialization was failed"),
        }
    }
}