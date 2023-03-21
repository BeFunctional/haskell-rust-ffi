/*******************************************************************************
  Auxiliary general-purpose macros

  The `map_tuple` macro is adapted from
  https://stackoverflow.com/questions/66396814/generating-tuple-indices-based-on-macro-rules-repetition-expansion .
*******************************************************************************/

/// Map function across all elements of a tuple
///
/// ```ignore
/// map_tuple( [T0, T1], tuple, f )
/// ```
///
/// will become
///
/// ```ignore
/// ( f(tuple.0) , f(tuple.1) )
/// ```
///
/// See also `map_tuple_ref`.
#[macro_export]
macro_rules! map_tuple {
    // Base-case: we are done. Return the accumulator
    //
    // We explicitly allow the list of indices to be non-empty (not all indices might be used)
    ( @, $tuple:ident, $fn:ident, [], [ $($ixs:tt)* ], [ $($acc:tt)* ] ) => {
        ( $($acc),* )
    };

    // Recursive-case: add entry to accumulator
    ( @, $tuple:ident, $fn:ident, [ $t:ident $(,$ts:ident)* ], [ ($ix:tt) $($ixs:tt)* ], [ $($acc:tt)* ] ) => {
        map_tuple!(@, $tuple, $fn, [ $($ts),* ], [ $($ixs)* ], [ $($acc)* ($fn($tuple . $ix)) ])
    };

    // Entry-point into the macro
    ( [ $($ts:ident),* ], $tuple:ident, $fn:ident ) => {
      map_tuple!(@, $tuple, $fn,
          // Pass original list of identifiers (only used to determine tuple length)
          [ $($ts),* ]

          // Pre-defined list of tuple indices
        , [(0) (1) (2) (3) (4) (5) (6) (7) (8) (9) (10) (11) (12) (13) (14) (15) (16) (17) (18) (19)]

          // Empty accumulator
        , []
        )
    }
}

/// Variation on `map_tuple` that uses a _reference_ to a tuple
///
/// TODO: It seems I cannot unify these two macros, because `&self.0` and `(&self).0` are not
/// equivalent expressions. Is that true..?
#[macro_export]
macro_rules! map_tuple_ref {
    // Base-case: we are done. Return the accumulator
    //
    // We explicitly allow the list of indices to be non-empty (not all indices might be used)
    ( @, $tuple:ident, $fn:ident, [], [ $($ixs:tt)* ], [ $($acc:tt)* ] ) => {
        ( $($acc),* )
    };

    // Recursive-case: add entry to accumulator
    ( @, $tuple:ident, $fn:ident, [ $t:ident $(,$ts:ident)* ], [ ($ix:tt) $($ixs:tt)* ], [ $($acc:tt)* ] ) => {
        map_tuple_ref!(@, $tuple, $fn, [ $($ts),* ], [ $($ixs)* ], [ $($acc)* ($fn(&$tuple . $ix)) ])
    };

    // Entry-point into the macro
    ( [ $($ts:ident),* ], $tuple:ident, $fn:ident ) => {
      map_tuple_ref!(@, $tuple, $fn,
          // Pass original list of identifiers (only used to determine tuple length)
          [ $($ts),* ]

          // Pre-defined list of tuple indices
        , [(0) (1) (2) (3) (4) (5) (6) (7) (8) (9) (10) (11) (12) (13) (14) (15) (16) (17) (18) (19)]

          // Empty accumulator
        , []
        )
    }
}

/// Fold a list of types
///
/// ```ignore
/// fold_types!( [T0, T1], haskell_size, tag, +, 0);
/// ```
///
/// expands to
///
/// ```ignore
/// 0 + <T0>::haskell_size(tag) + <T1>::haskell_size(tag)
/// ```
#[macro_export]
macro_rules! fold_types {
    // Base-case: we are done. Return the accumulator
    ( @, $f:ident, $arg:ident, $op:tt, [], $acc:tt ) => {
        $acc
    };

    // Recursive-case: add entry to accumulator
    ( @, $f:ident, $arg:ident, $op:tt, [ $t:ty $(,$ts:ty)* ], $acc:tt ) => {
        fold_types!(@, $f, $arg, $op, [ $($ts),* ], ( $acc $op (<$t> :: $f($arg)) ))
    };

    // Entry-point into the macro
    ( [ $($ts:ty),* ], $f:ident, $arg:ident, $op:tt, $e:tt ) => {
        fold_types!(@, $f, $arg, $op, [ $($ts),* ], $e)
    };
}

/*******************************************************************************
  Macros for deriving specific kinds of instances
*******************************************************************************/

/// Derive `ToHaskell` and `FromHaskell` instances for simple types: types with
/// no type arguments.
#[macro_export]
macro_rules! derive_simple_instances {
    ($t:ty) => {
        impl<Tag> ToHaskell<Tag> for $t {
            fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<()> {
                self.serialize(writer)?;
                Ok(())
            }
        }

        impl<Tag> FromHaskell<Tag> for $t {
            fn from_haskell(buf: &mut &[u8], _tag: PhantomData<Tag>) -> Result<Self> {
                let x = <$t>::deserialize(buf)?;
                Ok(x)
            }
        }
    };
}

/// Derive `ToHaskell` and `FromHaskell` instances for arrays of the specified size.
#[macro_export]
macro_rules! derive_array_instances {
    ($sz : literal) => {
        impl<Tag, T: ToHaskell<Tag>> ToHaskell<Tag> for [T; $sz] {
            fn to_haskell<W: Write>(&self, writer: &mut W, _: PhantomData<Tag>) -> Result<()> {
                let tagged: [&Haskell<Tag, T>; $sz] = self.each_ref().map(tag_ref);
                tagged.serialize(writer)?;
                Ok(())
            }
        }

        impl<Tag, T: FromHaskell<Tag> + Default + Copy> FromHaskell<Tag> for [T; $sz] {
            fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self> {
                let tagged: [Haskell<Tag, T>; $sz] = BorshDeserialize::deserialize(buf)?;
                Ok(tagged.map(untag_val))
            }
        }
    };
}

/// Derive `ToHaskell` and `FromHaskell` for tuples with the specified number of type arguments
/// (i.e., for tuples of the specified size).
#[macro_export]
macro_rules! derive_tuple_instances {
    ($($ts:ident),*) => {
        impl<Tag, $($ts: ToHaskell<Tag> ),* > ToHaskell<Tag> for ( $($ts ),* ) {
            fn to_haskell<W: Write>(&self, writer: &mut W,_: PhantomData<Tag>) -> Result<()> {
                let tagged: ( $(&Haskell<Tag, $ts> ),* ) = map_tuple_ref!( [ $($ts),* ], self, tag_ref );
                tagged.serialize(writer)?;
                Ok(())
            }
        }

        impl<Tag, $($ts: FromHaskell<Tag> ),* > FromHaskell<Tag> for ( $($ts ),* ) {
            fn from_haskell(buf: &mut &[u8], _: PhantomData<Tag>) -> Result<Self> {
                let tagged: ( $(Haskell<Tag, $ts> ),* ) = BorshDeserialize::deserialize(buf)?;
                Ok( map_tuple!( [ $($ts),* ], tagged, untag_val ) )
            }
        }
    };
}

/// Derive `HaskellSize` instance for tuple with the specified type arguments.
#[macro_export]
macro_rules! derive_size_tuple_instance {
    ($($ts:ident),*) => {
        impl<Tag, $($ts: HaskellSize<Tag> ),* > HaskellSize<Tag> for ( $($ts),* ) {
            fn haskell_size(tag: PhantomData<Tag>) -> usize {
                fold_types!( [ $($ts),* ], haskell_size, tag, +, 0)
            }
        }
    };
}
