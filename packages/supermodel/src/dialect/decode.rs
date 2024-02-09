pub trait Decode: Sized {
    fn decode(decoder: &mut dyn Decoder) -> Result<Self, Error>;
}

pub trait Decoder {
}

pub trait Row: PartialEq {
    fn index(&self) -> usize;

    fn read_next<T: Decode>(&mut self) -> Result<T, Error> where Self: Sized;
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Error;

impl Decode for i64 {
    #[inline]
    fn decode(_decoder: &mut dyn Decoder) -> Result<Self, Error> {
        todo!()
    }
}

impl Decode for String {
    #[inline]
    fn decode(_decoder: &mut dyn Decoder) -> Result<Self, Error> {
        todo!()
    }
}