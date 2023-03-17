pub mod borsh_instances;
mod macros;

use std::marker::PhantomData;

pub struct Tagged<Tag, T> {
    pub value: T,
    pub tag: PhantomData<Tag>,
}

pub fn tag<Tag, T>(t: T) -> Tagged<Tag, T> {
    Tagged {
        value: t,
        tag: PhantomData,
    }
}
