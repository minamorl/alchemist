use error::AlchemistError;

pub trait Serializable {
  fn serialize(&self) -> Result<Vec<u8>, AlchemistError>;
}

pub trait Deserializable<T> {
  fn deserialize(&self) -> Result<T, AlchemistError>;
}