use crate::dialect::Dialect;
use crate::query::Query;
use crate::response::From;

pub trait Executor<D: Dialect>: Send + Sync {
    async fn execute_query<T: From<D>>(&mut self, query: impl Query) -> Result<T, D::Error>;
}