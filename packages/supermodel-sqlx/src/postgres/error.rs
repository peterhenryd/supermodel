use supermodel::dialect::decode;

#[derive(Debug)]
pub enum Error {
    Sqlx(sqlx::Error),
    Decode(decode::Error)
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<decode::Error> for Error {
    fn from(value: decode::Error) -> Self {
        Self::Decode(value)
    }
}