use sqlx::Connection as _;
use supermodel::dialect::Dialect;
use supermodel::field::data_type::DataType;
pub use crate::postgres::connection::Connection;
pub use crate::postgres::decoder::Decoder;
pub use crate::postgres::encoder::Encoder;
pub use crate::postgres::error::Error;
pub use crate::postgres::pool::Pool;
pub use crate::postgres::response::Response;

mod connection;
mod decoder;
mod encoder;
mod error;
mod reader;
mod pool;
mod response;

pub struct Postgres;

impl Dialect for Postgres {
    type Error = Error;
    type Options<'a> = &'a str;
    type Response = Response;
    type Connection = Connection;
    type Pool = Pool;
    type Decoder = Decoder;
    type Encoder = Encoder;

    async fn create_connection<'a>(url: Self::Options<'a>) -> Result<Self::Connection, Self::Error> {
        Ok(Connection(sqlx::PgConnection::connect(url).await?))
    }

    async fn create_pool<'a>(url: Self::Options<'a>) -> Result<Self::Pool, Self::Error> {
        Ok(Pool(sqlx::PgPool::connect(url).await?))
    }

    fn get_data_type_sql(data_type: &DataType) -> String {
        data_type.to_postgresql()
    }
}