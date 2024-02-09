use core::marker::PhantomData;
use crate::dialect::Dialect;
use crate::response::From;

pub trait Bind<D: Dialect, T: From<D>>: Sized {
}

pub trait Utils<D: Dialect, T: From<D>>: Bind<D, T> {
    fn bind(self, values: T) -> Bound<D, Self, T>;
}

impl<D: Dialect, O: Bind<D, T>, T: From<D>> Utils<D, T> for O {
    #[inline]
    fn bind(self, values: T) -> Bound<D, Self, T> {
        Bound { operation: self, values, _marker: PhantomData }
    }
}

#[non_exhaustive]
pub struct Bound<D: Dialect, O: Bind<D, T>, T: From<D>> {
    pub operation: O,
    pub values: T,
    pub _marker: PhantomData<D>,
}