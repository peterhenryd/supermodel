use core::marker::PhantomData;
use crate::dialect::decode::{Decode, Decoder, Error};
use crate::dialect::Dialect;
use crate::field::data_type::DataType;
use crate::field::insertion::Insertion;
pub use utils::Utils;

pub mod data_type;
pub mod insertion;
pub mod read;
pub mod selection;
mod utils;

#[non_exhaustive]
pub struct Typed<T> {
    pub field: Field,
    pub _marker: PhantomData<T>
}

#[derive(Copy, Clone)]
#[non_exhaustive]
pub struct Field {
    pub name: &'static str,
    pub data_type: DataType,
}

pub trait Fields<D>
    where D: Dialect {
    const DEFINED_FIELDS: &'static [Field];
    const TOTAL_FIELDS: &'static [Field];

    type Insertion: Insertion<D>;
}

#[derive(Copy, Clone)]
pub struct Id<T: Copy + Clone = i64>(T);

impl<T: Decode + Copy + Clone> Decode for Id<T> {
    #[inline]
    fn decode(_decoder: &mut dyn Decoder) -> Result<Self, Error> {
        todo!()
    }
}