pub trait Encode {
    fn encode(&self, encoder: &mut dyn Encoder);
}

pub trait Encoder {
    fn new() -> Self where Self: Sized;

    // TODO: ideally, this would be a trait impl like ToOwned, however that causes Encoder<Owned=Box<dyn Encoder>>
    fn box_clone(&self) -> Box<dyn Encoder>;

    fn as_str(&self) -> &str;
}

impl Encode for u64 {
    #[inline]
    fn encode(&self, _encoder: &mut dyn Encoder) {
        todo!()
    }
}

impl Encode for String {
    #[inline]
    fn encode(&self, _encoder: &mut dyn Encoder) {
        todo!()
    }
}