use std::marker::PhantomData;

use crate::{derive_size_tuple_instance, fold_types};

pub use haskell_ffi_derive::HaskellSize;

/*******************************************************************************
  Main class definition
*******************************************************************************/

pub trait HaskellSize<Tag> {
    /// Statically known size (in bytes)
    fn haskell_size(tag: PhantomData<Tag>) -> usize;
}

/*******************************************************************************
  Simple instances

  Note: the following types in the Borsh spec do _not_ have statically known sizes:

  - Vec<T>
  - HashMap<K, V>
  - HashSet<T>
  - Option<T>
  - String
*******************************************************************************/

impl<Tag> HaskellSize<Tag> for u8 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        1
    }
}

impl<Tag> HaskellSize<Tag> for u16 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        2
    }
}

impl<Tag> HaskellSize<Tag> for u32 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        4
    }
}

impl<Tag> HaskellSize<Tag> for u64 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        8
    }
}

impl<Tag> HaskellSize<Tag> for u128 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        16
    }
}

impl<Tag> HaskellSize<Tag> for i8 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        1
    }
}

impl<Tag> HaskellSize<Tag> for i16 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        2
    }
}

impl<Tag> HaskellSize<Tag> for i32 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        4
    }
}

impl<Tag> HaskellSize<Tag> for i64 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        8
    }
}

impl<Tag> HaskellSize<Tag> for i128 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        16
    }
}

impl<Tag> HaskellSize<Tag> for f32 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        4
    }
}

impl<Tag> HaskellSize<Tag> for f64 {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        8
    }
}

impl<Tag> HaskellSize<Tag> for () {
    fn haskell_size(_tag: PhantomData<Tag>) -> usize {
        0
    }
}

impl<Tag, T: HaskellSize<Tag>, const N: usize> HaskellSize<Tag> for [T; N] {
    fn haskell_size(tag: PhantomData<Tag>) -> usize {
        T::haskell_size(tag) * N
    }
}

/*******************************************************************************
  Tuples

  We support the same sizes of tuples as `borsh` does.
*******************************************************************************/

derive_size_tuple_instance!(T0, T1);
derive_size_tuple_instance!(T0, T1, T2);
derive_size_tuple_instance!(T0, T1, T2, T3);
derive_size_tuple_instance!(T0, T1, T2, T3, T4);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
derive_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
derive_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
derive_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
derive_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
derive_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);

/*******************************************************************************
  Sanity checks
*******************************************************************************/

#[cfg(test)]
mod tests {
    use std::io::Error;

    use borsh::BorshSerialize;

    use super::*;

    enum ExampleTag {}

    #[derive(HaskellSize, BorshSerialize)]
    struct EmptyStruct;

    #[derive(HaskellSize, BorshSerialize)]
    struct UnnamedStruct(u16, (u8, u32));

    #[derive(HaskellSize, BorshSerialize)]
    struct NamedStruct {
        a: u8,
        b: u16,
        c: (u32, u64),
    }

    #[derive(HaskellSize, BorshSerialize)]
    struct ParamStruct<T> {
        a: u8,
        b: (T, T, T),
    }

    #[test]
    fn empty() -> Result<(), Error> {
        let tag: PhantomData<ExampleTag> = PhantomData;
        assert_eq!(EmptyStruct::haskell_size(tag), 0);
        let encoded = EmptyStruct.try_to_vec()?;
        assert_eq!(encoded.len(), EmptyStruct::haskell_size(tag));
        Ok(())
    }

    #[test]
    fn unnamed() -> Result<(), Error> {
        let tag: PhantomData<ExampleTag> = PhantomData;
        assert_eq!(UnnamedStruct::haskell_size(tag), 7);
        let encoded = UnnamedStruct(1, (2, 3)).try_to_vec()?;
        assert_eq!(encoded.len(), UnnamedStruct::haskell_size(tag));
        Ok(())
    }

    #[test]
    fn named() -> Result<(), Error> {
        let tag: PhantomData<ExampleTag> = PhantomData;
        assert_eq!(NamedStruct::haskell_size(tag), 15);
        let encoded = NamedStruct {
            a: 1,
            b: 2,
            c: (3, 4),
        }
        .try_to_vec()?;
        assert_eq!(encoded.len(), NamedStruct::haskell_size(tag));
        Ok(())
    }

    #[test]
    fn param() -> Result<(), Error> {
        let tag: PhantomData<ExampleTag> = PhantomData;
        assert_eq!(<ParamStruct<f64>>::haskell_size(tag), 25);
        let encoded = ParamStruct {
            a: 1,
            b: (1.0, 2.0, 3.0),
        }
        .try_to_vec()?;
        assert_eq!(encoded.len(), <ParamStruct<f64>>::haskell_size(tag));
        Ok(())
    }
}
