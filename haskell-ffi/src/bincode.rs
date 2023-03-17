use std::{
    io::{Error, ErrorKind, Write},
    marker::PhantomData,
};

/// Implement `to_haskell` using `bincode`
///
/// The result will be length-prefixed ("bincode-in-Borsh").
pub fn bincode_to_haskell<Tag, T, W>(
    t: &T,
    writer: &mut W,
    _: PhantomData<Tag>,
) -> Result<(), Error>
where
    T: serde::ser::Serialize,
    W: Write,
{
    match bincode::serialize(t) {
        Ok(vec) => borsh::BorshSerialize::serialize(&vec, writer),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}

/// Implement `from_haskell` using `bincode`
///
/// See als `bincode_to_haskell`
pub fn bincode_from_haskell<Tag, T>(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let vec: Vec<u8> = borsh::BorshDeserialize::deserialize(buf)?;
    match bincode::deserialize(vec.as_ref()) {
        Ok(x) => Ok(x),
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    }
}
