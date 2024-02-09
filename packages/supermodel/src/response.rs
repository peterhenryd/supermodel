use crate::dialect::{decode, Dialect};

pub trait Response<D: Dialect> {
    fn is_decodable(&self) -> bool;

    fn into_rows_affected(self) -> Option<u64>;

    fn into_decoder(self) -> Option<D::Decoder>;
}

pub trait From<D: Dialect>: Sized + Send + Sync {
    fn from_response(response: D::Response) -> Option<Result<Self, decode::Error>>;
}

impl<D: Dialect> From<D> for () {
    #[inline]
    fn from_response(_response: D::Response) -> Option<Result<Self, decode::Error>> {
        Some(Ok(()))
    }
}