use std::{
    io::{Error, Write},
    marker::PhantomData,
};

use crate::HaskellSize;

/*******************************************************************************
  Main class definition
*******************************************************************************/

// Copied from `borsh`
const DEFAULT_SERIALIZER_CAPACITY: usize = 1024;

pub trait ToHaskell<Tag> {
    /// Serialize data to be sent to Haskell
    ///
    /// This is the analogue of `BorshSerialize::serialize`.
    ///
    /// The `tag` argument allows client libraries to define additional
    /// instances of `ToHaskell` for foreign (non-local) types. For example, the
    /// `solana-sdk-haskell` library can define a `ToHaskell` instance for
    /// `Keypair`, defined in `solana-sdk`, as long as it uses a tag `Solana`
    /// defined locally in the `solana-haskell-sdk` package.
    fn to_haskell<W: Write>(&self, writer: &mut W, tag: PhantomData<Tag>) -> Result<(), Error>;

    fn to_haskell_vec(&self, tag: PhantomData<Tag>) -> Result<Vec<u8>, Error> {
        let mut result = Vec::with_capacity(DEFAULT_SERIALIZER_CAPACITY);
        self.to_haskell(&mut result, tag)?;
        Ok(result)
    }
}

impl<Tag, T: ToHaskell<Tag>> ToHaskell<Tag> for &T {
    fn to_haskell<W: Write>(&self, writer: &mut W, tag: PhantomData<Tag>) -> Result<(), Error> {
        (*self).to_haskell(writer, tag)
    }
}

/*******************************************************************************
  Derived functionality

  These functions are not defined in the trait itself, to make it clear that
  they only exist at top-level calls, and will not be recursively called
  in various `ToHaskell` instances. This is important, because the `len`
  parameter that gives the length of the buffer only applies to the _overall_
  buffer.
*******************************************************************************/

/// Marshall value with fixed-sized encoding
///
/// The `out_len` parameter is only used to verify that the Haskell-side and
/// the Rust side agree on the length of the encoding.
pub fn marshall_to_haskell_fixed<Tag, T>(t: &T, out: *mut u8, out_len: usize, tag: PhantomData<Tag>)
where
    T: HaskellSize<Tag> + ToHaskell<Tag>,
{
    let expected_len: usize = T::haskell_size(tag);
    if out_len != expected_len {
        panic!(
            "marshall_to_haskell_fixed: expected buffer of size {}, but got {}",
            expected_len, out_len
        )
    } else {
        let mut out_len_copy = out_len;
        marshall_to_haskell_var(t, out, &mut out_len_copy, tag);
    }
}

/// Marshall value with variable-sized encoding
pub fn marshall_to_haskell_var<Tag, T>(
    t: &T,
    out: *mut u8,
    out_len: &mut usize,
    tag: PhantomData<Tag>,
) where
    T: ToHaskell<Tag>,
{
    match t.to_haskell_vec(tag) {
        Ok(vec) => {
            let slice: &[u8] = vec.as_ref();

            if slice.len() <= *out_len {
                unsafe {
                    std::ptr::copy(slice.as_ptr(), out, slice.len());
                }
            }

            *out_len = slice.len();
        }
        Err(e) => panic!("{}", e),
    }
}
