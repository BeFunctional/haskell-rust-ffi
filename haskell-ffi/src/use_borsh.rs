use borsh::{BorshDeserialize, BorshSerialize};
use std::{
    io::{Error, Write},
    marker::PhantomData,
};

use crate::{FromHaskell, ToHaskell};

/// Newtype wrapper for defaulting to `borsh` for `ToHaskell`/`FromHaskell`
///
/// `ToHaskell`/`FromHaskell` have instances for types such as `Vec<T>`, but
/// those instances depend on `ToHaskell`/`FromHaskell` for `T`. This
/// indirection is not always necessary, and may be expensive. The `UseBorsh`
/// newtype wrapper can be used to mark values where `ToHaskell`/`FromHaskell`
/// should just piggy-back on Borsh.
pub struct UseBorsh<T>(pub T);

pub fn unwrap_use_borsh<T>(use_borsh: UseBorsh<T>) -> T {
    let UseBorsh(t) = use_borsh;
    t
}

pub fn unwrap_use_borsh_ref<T>(use_borsh: &UseBorsh<T>) -> &T {
    let UseBorsh(t) = use_borsh;
    t
}

/*******************************************************************************
  Forwarding instances

  These instances _define_ the `UseBorsh` type
*******************************************************************************/

impl<Tag, T: BorshSerialize> ToHaskell<Tag> for UseBorsh<T> {
    fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<(), Error> {
        unwrap_use_borsh_ref(self).serialize(writer)
    }
}

impl<Tag, T: BorshDeserialize> FromHaskell<Tag> for UseBorsh<T> {
    fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self, Error> {
        T::deserialize(buf).map(UseBorsh)
    }
}

/*******************************************************************************
  Additional standard instances
*******************************************************************************/

impl<T: AsRef<T>> AsRef<T> for UseBorsh<T> {
    fn as_ref(&self) -> &T {
        unwrap_use_borsh_ref(self).as_ref()
    }
}
