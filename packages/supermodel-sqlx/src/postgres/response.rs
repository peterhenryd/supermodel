use crate::postgres::{Decoder, Postgres};

pub enum Response {
    QueryResult(sqlx::postgres::PgQueryResult),
    Rows(Vec<sqlx::postgres::PgRow>)
}

impl supermodel::response::Response<Postgres> for Response {
    fn is_decodable(&self) -> bool {
        match &self {
            Response::Rows(_) => true,
            Response::QueryResult(_) => false,
        }
    }
    
    fn into_rows_affected(self) -> Option<u64> {
        match self {
            Response::QueryResult(result) => Some(result.rows_affected()),
            Response::Rows(_) => None
        }
    }

    fn into_decoder(self) -> Option<Decoder> {
        match self {
            Response::QueryResult(_) => None,
            Response::Rows(rows) => Some(Decoder::new(rows))
        }
    }
}