use crate::dialect::Dialect;
use crate::dialect::encode::Encode;

pub trait Insertion<D: Dialect>: Send + Sync {
    fn into_values(self) -> Vec<Box<dyn Encode>>;

    fn encode_all(&self) -> D::Encoder;
}

pub trait Utils<D: Dialect> {

}

impl<D: Dialect, I: Insertion<D>> Utils<D> for I {

}