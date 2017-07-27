use error::{AlchemistError};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub fn u32tobytes(v: u32) -> Result<Vec<u8>, AlchemistError> {
    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(v)
        .map_err(|_| AlchemistError::DeserializationFailed)?;
    Ok(wtr)
}

pub fn bytestou32(v: &[u8]) -> Result<u32, AlchemistError> {
    let mut rdr = Cursor::new(v);
    rdr.read_u32::<BigEndian>()
        .map_err(|_| AlchemistError::DeserializationFailed)
}
