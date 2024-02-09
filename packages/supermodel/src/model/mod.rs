use crate::dialect::Dialect;
use crate::field::{Fields, Id};
use crate::response::From;
pub use utils::Utils;

mod utils;

pub trait Model<D: Dialect>: Sized + From<D> + Send + Sync {
    type Fields: Fields<D>;

    const NAME: &'static str;

    fn id(&self) -> Id;
}