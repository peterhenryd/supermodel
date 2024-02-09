use core::marker::PhantomData;
use crate::dialect::Dialect;
use crate::field::Fields;
use crate::field::insertion::Insertion;
use crate::model::Model;
use crate::operation::Operation;
use crate::query::Fetch;

#[non_exhaustive]
pub struct InsertInto<M, I> {
    pub model: &'static str,
    pub insertion: I,
    pub _marker: PhantomData<M>,
}

impl<D: Dialect, M: Model<D>, I: Insertion<D>> Operation<D> for InsertInto<M, I> {
    type Output = M;
    type Query = Fetch;

    #[inline]
    fn as_query(&self) -> Self::Query {
        Fetch::Insert {
            model: M::NAME,
            fields: M::Fields::DEFINED_FIELDS,
            values: Box::new(self.insertion.encode_all()),
            returning: M::Fields::TOTAL_FIELDS,
        }
    }
}