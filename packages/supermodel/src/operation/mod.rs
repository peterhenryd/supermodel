use crate::dialect::Dialect;
use crate::query::Query;
pub use utils::Utils;
use crate::response;

pub mod bind;
pub mod insert_into;
pub mod register_model;
pub mod select;
mod utils;

pub trait Operation<D: Dialect>: Send + Sync {
    type Output: response::From<D>;
    type Query: Query;

    fn as_query(&self) -> Self::Query;
}