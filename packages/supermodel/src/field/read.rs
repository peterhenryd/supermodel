/*
use crate::dialect::decode::Decode;
use crate::dialect::Dialect;

pub trait Reader {
    fn read_value<T: Decode>(&mut self) -> T;
}

pub trait Read {
    type Error;

    fn read<D: Dialect>(reader: &mut D::Reader) -> Result<Self, Self::Error> where Self: Sized;
}

impl Read for () {
    type Error = ();

    fn read<D: Dialect>(_reader: &mut D::Reader) -> Result<Self, Self::Error> where Self: Sized {
        Ok(())
    }
}

macro_rules! tuple_read {
    ($($types:ident, )*) => {
        impl<$($types, )*> Read for ($($types,)*)
        where $($types: Read, )* {
            type Error = ();

            fn read<Di: Dialect>(_reader: &mut Di::Reader) -> Result<Self, Self::Error> where Self: Sized {
                todo!()
            }
        }
    };
}

tuple_read! { A, B, }
tuple_read! { A, B, C, }
tuple_read! { A, B, C, D, }
tuple_read! { A, B, C, D, E, }
tuple_read! { A, B, C, D, E, F, }
tuple_read! { A, B, C, D, E, F, G, }
tuple_read! { A, B, C, D, E, F, G, H, }
tuple_read! { A, B, C, D, E, F, G, H, I, }

 */