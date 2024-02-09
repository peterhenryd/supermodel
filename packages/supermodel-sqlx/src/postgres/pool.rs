use sqlx::Acquire;
use supermodel::connector::Connector;
use supermodel::dialect::Dialect;
use supermodel::executor::Executor;
use supermodel::query::Query;
use supermodel::response::From;
use crate::postgres::{Postgres, Response};

#[derive(Clone)]
pub struct Pool(pub sqlx::PgPool);

impl Connector<Postgres> for Pool {}

impl supermodel::connector::Pool<Postgres> for Pool {
    type Executor = PoolExecutor;

    async fn acquire(&self) -> Result<Self::Executor, <Postgres as Dialect>::Error> {
        Ok(PoolExecutor(self.0.acquire().await?))
    }
}

pub struct PoolExecutor(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

impl Executor<Postgres> for PoolExecutor {
    async fn execute_query<'a, T: From<Postgres>>(&mut self, query: impl Query) -> Result<T, <Postgres as Dialect>::Error> {
        let sql = query.to_sql::<Postgres>();
        let query_result = sqlx::query(&sql).execute(self.0.acquire().await?).await?;
        let response = Response::QueryResult(query_result);

        Ok(T::from_response(response).unwrap()?)
    }
}