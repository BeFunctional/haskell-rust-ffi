use std::{
    io::{Error, ErrorKind},
    marker::PhantomData,
};

use crate::HaskellSize;

/*******************************************************************************
  Main class definition
*******************************************************************************/

const ERROR_NOT_ALL_BYTES_READ: &str = "Not all bytes read";

pub trait FromHaskell<Tag>: Sized {
    /// Deserialize data sent from Haskell
    ///
    /// This is the analogue of `BorshDeserialize::deserialize`.
    //
    /// See `ToHaskell` for a detailed discussion of the `tag` argument.
    fn from_haskell(buf: &mut &[u8], tag: PhantomData<Tag>) -> Result<Self, Error>;

    fn from_haskell_slice(slice: &[u8], tag: PhantomData<Tag>) -> Result<Self, Error> {
        let mut slice_mut = slice;
        let result = Self::from_haskell(&mut slice_mut, tag)?;
        if !slice_mut.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, ERROR_NOT_ALL_BYTES_READ));
        }
        Ok(result)
    }
}

/*******************************************************************************
  Derived functionality

  See comments in `to_haskell` for why these functions do not live inside the
  trait.
*******************************************************************************/

/// Marshall value with variable-sized encoding
pub fn marshall_from_haskell_var<Tag, T>(inp: *const u8, len: usize, tag: PhantomData<Tag>) -> T
where
    T: FromHaskell<Tag>,
{
    let mut vec: Vec<u8> = vec![0; len];
    unsafe {
        std::ptr::copy(inp, vec.as_mut_ptr(), len);
    }
    match T::from_haskell_slice(vec.as_ref(), tag) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    }
}

/// Marshall value with fixed-size encoding
///
/// The `len` argument here is only to verify that the Haskell-side and
/// Rust-side agree on the size of the encoding.
pub fn marshall_from_haskell_fixed<Tag, T>(
    inp: *const u8,
    inp_len: usize,
    tag: PhantomData<Tag>,
) -> T
where
    T: FromHaskell<Tag> + HaskellSize<Tag>,
{
    let expected_len = T::haskell_size(tag);

    if inp_len != expected_len {
        panic!(
            "expected buffer of size {}, but got {}",
            expected_len, inp_len
        )
    } else {
        marshall_from_haskell_var(inp, inp_len, tag)
    }
}
