use supermodel::connector::Connector;
use supermodel::dialect::Dialect;
use supermodel::executor::Executor;
use supermodel::query::Query;
use supermodel::response::From;
use crate::postgres::{Postgres, Response};

pub struct Connection(pub sqlx::PgConnection);

impl Connector<Postgres> for Connection {}

impl Executor<Postgres> for Connection {
    async fn execute_query<'a, T: From<Postgres>>(&mut self, query: impl Query) -> Result<T, <Postgres as Dialect>::Error> {
        let sql = query.to_sql::<Postgres>();
        let query_result = sqlx::query(&sql).execute(&mut self.0).await?;
        let response = Response::QueryResult(query_result);

        Ok(T::from_response(response).unwrap()?)
    }
}