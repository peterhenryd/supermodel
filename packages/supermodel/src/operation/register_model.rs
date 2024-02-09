use core::marker::PhantomData;
use crate::dialect::Dialect;
use crate::field::Fields;
use crate::model::Model;
use crate::operation::Operation;
use crate::query::Execute;

pub struct RegisterModel<M>(PhantomData<M>);

impl<M> Default for RegisterModel<M> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}



impl<D: Dialect, M: Model<D>> Operation<D> for RegisterModel<M> {
    type Output = ();
    type Query = Execute;

    #[inline]
    fn as_query(&self) -> Execute {
        Execute::RegisterModel {
            name: M::NAME,
            fields: M::Fields::TOTAL_FIELDS,
        }
    }
}