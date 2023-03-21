use borsh::{BorshDeserialize, BorshSerialize};
use ref_cast::RefCast;
use std::{
    cmp::Ordering,
    fmt::Debug,
    hash::{Hash, Hasher},
    io::Write,
    marker::PhantomData,
};

use crate::{from_haskell::FromHaskell, to_haskell::ToHaskell};

/*******************************************************************************
  Deriving-via support
*******************************************************************************/

#[derive(RefCast)]
#[repr(transparent)]
/// Newtype for "deriving-via" instances
///
/// The purpose of this newtype is best illustrated through its instances:
///
/// ```ignore
/// impl<Tag, T: ToHaskell<Tag>>   BorshSerialize   for Haskell<Tag, T>
/// impl<Tag, T: FromHaskell<Tag>> BorshDeserialize for Haskell<Tag, T>
/// ```
///
/// This is primarily used internally: when deriving `ToHaskell`/`FromHaskell`
/// instances for standard types, we want to re-use the logic from `borsh`,
/// rather than re-implement everything here. We do this by turning say a
/// `Vec<T>` into a `Vec<Haskell<Tag, T>>`, and then call functions from
/// `borsh`. The use of the newtype wrapper then ensures that the constraint
/// on `T` will be in terms of `ToHaskell`/`FromHaskell` again.
pub struct Haskell<Tag, T>(pub T, PhantomData<Tag>);

pub fn tag_val<Tag, T>(t: T) -> Haskell<Tag, T> {
    Haskell(t, PhantomData)
}

pub fn tag_ref<Tag, T>(t: &T) -> &Haskell<Tag, T> {
    RefCast::ref_cast(t)
}

pub fn untag_val<Tag, T>(tagged: Haskell<Tag, T>) -> T {
    tagged.0
}

pub fn untag_ref<Tag, T>(tagged: &Haskell<Tag, T>) -> &T {
    &tagged.0
}

/*******************************************************************************
  Standard instances
*******************************************************************************/

impl<Tag, T: Debug> Debug for Haskell<Tag, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<Tag, T: PartialEq> PartialEq for Haskell<Tag, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Tag, T: Eq> Eq for Haskell<Tag, T> {}

impl<Tag, T: PartialOrd> PartialOrd for Haskell<Tag, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<Tag, T: Hash> Hash for Haskell<Tag, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<Tag, T: Default> Default for Haskell<Tag, T> {
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<Tag, T: Clone> Clone for Haskell<Tag, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<Tag, T: Copy> Copy for Haskell<Tag, T> {}

/*******************************************************************************
  Forwarding instances

  NOTE: We do not expect _additional_ forwarding instances to be defined.
*******************************************************************************/

impl<Tag, T: ToHaskell<Tag>> BorshSerialize for Haskell<Tag, T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self.0.to_haskell(writer, PhantomData) {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

impl<Tag, T: FromHaskell<Tag>> BorshDeserialize for Haskell<Tag, T> {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let tag: PhantomData<Tag> = PhantomData;
        match T::from_haskell(buf, tag).map(tag_val) {
            Ok(x) => Ok(x),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}
