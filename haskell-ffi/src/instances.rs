//! ToHaskell and FromHaskell instances for the various standard types mandated
//! by the [Borsh spec](https://borsh.io/), piggy-backing on the implementation
//! in the `borsh` crate. The only spec-described types _not_ provided are
//! user-defined structs and enums.

use borsh::{BorshDeserialize, BorshSerialize};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{Error, ErrorKind, Write},
    marker::PhantomData,
};

use crate::{
    derive_array_instances, derive_simple_instances, derive_tuple_instances,
    deriving_via::{tag_ref, untag_val, Haskell},
    from_haskell::FromHaskell,
    map_tuple, map_tuple_ref,
    to_haskell::ToHaskell,
    HaskellSize,
};

/*******************************************************************************
  Simple (non-composite) instances
*******************************************************************************/

derive_simple_instances!(u8);
derive_simple_instances!(u16);
derive_simple_instances!(u32);
derive_simple_instances!(u64);
derive_simple_instances!(u128);
derive_simple_instances!(i8);
derive_simple_instances!(i16);
derive_simple_instances!(i32);
derive_simple_instances!(i64);
derive_simple_instances!(i128);
derive_simple_instances!(f32);
derive_simple_instances!(f64);
derive_simple_instances!(());
derive_simple_instances!(String);

/*******************************************************************************
  Array instances

  This is the same set of sizes as supported by borsh.
*******************************************************************************/

derive_array_instances!(0);
derive_array_instances!(1);
derive_array_instances!(2);
derive_array_instances!(3);
derive_array_instances!(4);
derive_array_instances!(5);
derive_array_instances!(6);
derive_array_instances!(7);
derive_array_instances!(8);
derive_array_instances!(9);
derive_array_instances!(10);
derive_array_instances!(11);
derive_array_instances!(12);
derive_array_instances!(13);
derive_array_instances!(14);
derive_array_instances!(15);
derive_array_instances!(16);
derive_array_instances!(17);
derive_array_instances!(18);
derive_array_instances!(19);
derive_array_instances!(20);
derive_array_instances!(21);
derive_array_instances!(22);
derive_array_instances!(23);
derive_array_instances!(24);
derive_array_instances!(25);
derive_array_instances!(26);
derive_array_instances!(27);
derive_array_instances!(28);
derive_array_instances!(29);
derive_array_instances!(30);
derive_array_instances!(31);
derive_array_instances!(32);

derive_array_instances!(64);
derive_array_instances!(65);

derive_array_instances!(128);
derive_array_instances!(256);
derive_array_instances!(512);
derive_array_instances!(1024);
derive_array_instances!(2048);

/*******************************************************************************
  Composite instances

  This is the same set of tuple sizes as supported by `borsh.`
*******************************************************************************/

derive_tuple_instances!(T0, T1);
derive_tuple_instances!(T0, T1, T2);
derive_tuple_instances!(T0, T1, T2, T3);
derive_tuple_instances!(T0, T1, T2, T3, T4);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
derive_tuple_instances!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
derive_tuple_instances!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
derive_tuple_instances!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
derive_tuple_instances!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);

/*******************************************************************************
  Vec
*******************************************************************************/

impl<Tag, T: ToHaskell<Tag>> ToHaskell<Tag> for Vec<T> {
    fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<(), Error> {
        let tagged: Vec<&Haskell<Tag, T>> = self.iter().map(tag_ref).collect();
        tagged.serialize(writer)
    }
}

impl<Tag, T: FromHaskell<Tag>> FromHaskell<Tag> for Vec<T> {
    fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self, Error> {
        let tagged: Vec<Haskell<Tag, T>> = BorshDeserialize::deserialize(buf)?;
        Ok(tagged.into_iter().map(untag_val).collect())
    }
}

/*******************************************************************************
  HashMap
*******************************************************************************/

impl<Tag, K, V> ToHaskell<Tag> for HashMap<K, V>
where
    K: Eq + PartialOrd + Hash + ToHaskell<Tag>,
    V: ToHaskell<Tag>,
{
    fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<(), Error> {
        let tagged: HashMap<&Haskell<Tag, K>, &Haskell<Tag, V>> =
            self.iter().map(|(k, v)| (tag_ref(k), tag_ref(v))).collect();
        tagged.serialize(writer)
    }
}

impl<Tag, K, V> FromHaskell<Tag> for HashMap<K, V>
where
    K: Eq + Hash + FromHaskell<Tag>,
    V: FromHaskell<Tag>,
{
    fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self, Error> {
        let tagged: HashMap<Haskell<Tag, K>, Haskell<Tag, V>> = BorshDeserialize::deserialize(buf)?;
        Ok(tagged
            .into_iter()
            .map(|(k, v)| (untag_val(k), untag_val(v)))
            .collect())
    }
}

/*******************************************************************************
  HashSet
*******************************************************************************/

impl<Tag, T> ToHaskell<Tag> for HashSet<T>
where
    T: Eq + PartialOrd + Hash + ToHaskell<Tag>,
{
    fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<(), Error> {
        let tagged: HashSet<&Haskell<Tag, T>> = self.iter().map(tag_ref).collect();
        tagged.serialize(writer)
    }
}

impl<Tag, T> FromHaskell<Tag> for HashSet<T>
where
    T: Eq + Hash + FromHaskell<Tag>,
{
    fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self, Error> {
        let tagged: HashSet<Haskell<Tag, T>> = BorshDeserialize::deserialize(buf)?;
        Ok(tagged.into_iter().map(untag_val).collect())
    }
}

/*******************************************************************************
  Option
*******************************************************************************/

impl<Tag, T: ToHaskell<Tag>> ToHaskell<Tag> for Option<T> {
    fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<(), Error> {
        let tagged: Option<&Haskell<Tag, T>> = self.as_ref().map(tag_ref);
        tagged.serialize(writer)
    }
}

impl<Tag, T: FromHaskell<Tag>> FromHaskell<Tag> for Option<T> {
    fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self, Error> {
        let tagged: Option<Haskell<Tag, T>> = BorshDeserialize::deserialize(buf)?;
        Ok(tagged.map(untag_val))
    }
}

/*******************************************************************************
  Result

  `Result` is not explicitly mentioned by the Borsh spec, but it's ubiquitous
  and so we provide an instance for it, following the standard rule for enum.

  There is no need for an instance of `FromHaskell`, since this is indicating
  the result of some Rust-side operation.
*******************************************************************************/

impl<Tag, T: ToHaskell<Tag>, E: ToHaskell<Tag>> ToHaskell<Tag> for Result<T, E> {
    fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<(), Error> {
        let tagged: Result<&Haskell<Tag, T>, &Haskell<Tag, E>> = match self {
            Ok(t) => Ok(tag_ref(t)),
            Err(e) => Err(tag_ref(e)),
        };
        tagged.serialize(writer)
    }
}

/*******************************************************************************
  Bool

  The Borsh spec does not mention Bool; we encode `true` as 1 and `false` as 0;
  this matches what the Haskell `borsh` library does.
*******************************************************************************/

impl<Tag> HaskellSize<Tag> for bool {
    fn haskell_size(tag: PhantomData<Tag>) -> usize {
        u8::haskell_size(tag)
    }
}

impl<Tag> ToHaskell<Tag> for bool {
    fn to_haskell<W: Write>(&self, writer: &mut W, tag: PhantomData<Tag>) -> Result<(), Error> {
        let as_u8: u8 = if *self { 1 } else { 0 };
        as_u8.to_haskell(writer, tag)
    }
}

impl<Tag> FromHaskell<Tag> for bool {
    fn from_haskell(buf: &mut &[u8], tag: PhantomData<Tag>) -> Result<Self, Error> {
        let as_u8 = u8::from_haskell(buf, tag)?;
        match as_u8 {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid bool")),
        }
    }
}
