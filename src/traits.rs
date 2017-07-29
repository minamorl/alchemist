use error::AlchemistError;
use objects::KeyValueHeader;

pub trait Serializable {
  fn serialize(&self) -> Result<Vec<u8>, AlchemistError>;
}

pub trait Deserializable<T> {
  fn deserialize(&self) -> Result<T, AlchemistError>;
}