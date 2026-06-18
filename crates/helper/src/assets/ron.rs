use std::io::{Error as IoError, ErrorKind};

use serde::de::DeserializeOwned;

pub fn from_bytes<T>(bytes: &[u8]) -> error::Result<T>
where
    T: DeserializeOwned,
{
    ron::de::from_bytes(bytes).map_err(|error| IoError::new(ErrorKind::InvalidData, error).into())
}
