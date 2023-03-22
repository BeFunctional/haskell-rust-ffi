use std::{fmt::Display, io::Write, marker::PhantomData};

use crate::{error::Result, haskell_max_size::HaskellMaxSize, HaskellSize};

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
    fn to_haskell<W: Write>(&self, writer: &mut W, tag: PhantomData<Tag>) -> Result<()>;

    fn to_haskell_vec(&self, tag: PhantomData<Tag>) -> Result<Vec<u8>> {
        let mut result = Vec::with_capacity(DEFAULT_SERIALIZER_CAPACITY);
        self.to_haskell(&mut result, tag)?;
        Ok(result)
    }
}

impl<Tag, T: ToHaskell<Tag>> ToHaskell<Tag> for &T {
    fn to_haskell<W: Write>(&self, writer: &mut W, tag: PhantomData<Tag>) -> Result<()> {
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
        if out_len_copy != expected_len {
            panic!(
                "marshall_to_haskell_fixed: got buffer of expected size {}, but needed {}; bug in HaskellSize instance?",
                expected_len, out_len_copy
            );
        }
    }
}

/// Marshall value with encoding of known maximum size
///
/// The `out_len` parameter is only used to verify that the Haskell-side and
/// the Rust side agree on the length of the encoding.
pub fn marshall_to_haskell_max<Tag, T>(t: &T, out: *mut u8, out_len: usize, tag: PhantomData<Tag>)
where
    T: HaskellMaxSize<Tag> + ToHaskell<Tag>,
{
    let max_len: usize = T::haskell_max_size(tag);
    if out_len != max_len {
        panic!(
            "marshall_to_haskell_max: expected buffer of size {}, but got {}",
            max_len, out_len
        )
    } else {
        let mut out_len_copy = out_len;
        marshall_to_haskell_var(t, out, &mut out_len_copy, tag);
        if out_len_copy > max_len {
            panic!(
                "marshall_to_haskell_max: required size {} exceeds maximum {}; bug in HaskellMaxSize instance?",
                out_len_copy, max_len
            );
        }
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

/// Wrapper around `marshall_to_haskell_var` that calls `format` for errors
pub fn marshall_result_to_haskell_var<Tag, T, E>(
    res: &core::result::Result<T, E>,
    out: *mut u8,
    out_len: &mut usize,
    tag: PhantomData<Tag>,
) where
    T: ToHaskell<Tag>,
    E: Display,
{
    let res: core::result::Result<&T, String> = match res {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("{}", e)),
    };
    marshall_to_haskell_var(&res, out, out_len, tag);
}
