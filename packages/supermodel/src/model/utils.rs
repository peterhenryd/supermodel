use core::marker::PhantomData;
use crate::dialect::Dialect;
use crate::field::Fields;
use crate::model::Model;
use crate::operation::insert_into::InsertInto;
use crate::operation::register_model::RegisterModel;

pub trait Utils<D: Dialect>: Model<D> {
    type Insertion;

    fn create(insertion: Self::Insertion) -> InsertInto<Self, Self::Insertion>;

    fn register() -> RegisterModel<Self>;
}

impl<D: Dialect, M: Model<D>> Utils<D> for M {
    type Insertion = <Self::Fields as Fields<D>>::Insertion;

    #[inline]
    fn create(insertion: Self::Insertion) -> InsertInto<Self, Self::Insertion> {
        InsertInto { model: M::NAME, insertion, _marker: PhantomData }
    }

    #[inline]
    fn register() -> RegisterModel<Self> {
        RegisterModel::default()
    }
}