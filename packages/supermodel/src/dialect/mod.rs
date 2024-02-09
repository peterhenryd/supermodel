use crate::connector;
use crate::dialect::decode::Decoder;
use crate::dialect::encode::Encoder;
use crate::field::data_type::DataType;
use crate::response::Response;

pub mod encode;
pub mod decode;
pub mod utils;

pub trait Dialect: Sized + 'static {
    type Error: From<decode::Error> + Send + Sync;
    type Options<'options>;
    type Response: Response<Self>;
    type Connection: connector::Connector<Self>;
    type Pool: connector::Pool<Self>;
    type Decoder: Decoder;
    type Encoder: Encoder;

    async fn create_connection(options: Self::Options<'_>) -> Result<Self::Connection, Self::Error>;

    async fn create_pool(options: Self::Options<'_>) -> Result<Self::Pool, Self::Error>;

    fn get_data_type_sql(data_type: &DataType) -> String;
}