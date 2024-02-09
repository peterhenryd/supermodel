use core::marker::PhantomData;
use crate::dialect::decode::Decode;
use crate::dialect::encode::Encode;

pub trait Selector {
    type Type: Encode + Decode;
}

#[non_exhaustive]
pub struct Selection<T>(pub PhantomData<T>);