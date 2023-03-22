use std::{cmp::max, marker::PhantomData};

use crate::{derive_max_size_tuple_instance, fold_types, haskell_size::HaskellSize};

/*******************************************************************************
  Main class definition

  TODO: We do not currently support deriving 'HaskellMaxSize'.
*******************************************************************************/

pub trait HaskellMaxSize<Tag> {
    /// Statically known size (in bytes)
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize;
}

/*******************************************************************************
  Simple instances

  These all just piggy-back on the `HaskellSize` instances.
*******************************************************************************/

impl<Tag> HaskellMaxSize<Tag> for u8 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        u8::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for u16 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        u16::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for u32 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        u32::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for u64 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        u64::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for u128 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        u128::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for i8 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        i8::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for i16 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        i16::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for i32 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        i32::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for i64 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        i64::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for i128 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        i128::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for f32 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        f32::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for f64 {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        f64::haskell_size(tag)
    }
}

impl<Tag> HaskellMaxSize<Tag> for () {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        <()>::haskell_size(tag)
    }
}

/*******************************************************************************
  Composite instances

  See comments in `instances.rs` regarding `Result`
*******************************************************************************/

impl<Tag, T: HaskellMaxSize<Tag>, const N: usize> HaskellMaxSize<Tag> for [T; N] {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        T::haskell_max_size(tag) * N
    }
}

impl<Tag, T: HaskellMaxSize<Tag>> HaskellMaxSize<Tag> for Option<T> {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        1 + T::haskell_max_size(tag)
    }
}

impl<Tag, T: HaskellMaxSize<Tag>, E: HaskellMaxSize<Tag>> HaskellMaxSize<Tag> for Result<T, E> {
    fn haskell_max_size(tag: PhantomData<Tag>) -> usize {
        1 + max(T::haskell_max_size(tag), E::haskell_max_size(tag))
    }
}

/*******************************************************************************
  Tuples

  We support the same sizes of tuples as `borsh` does.
*******************************************************************************/

derive_max_size_tuple_instance!(T0, T1);
derive_max_size_tuple_instance!(T0, T1, T2);
derive_max_size_tuple_instance!(T0, T1, T2, T3);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
derive_max_size_tuple_instance!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
derive_max_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15
);
derive_max_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16
);
derive_max_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
);
derive_max_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
);
derive_max_size_tuple_instance!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
);
