#[derive(Clone)]
pub struct Encoder(String);

impl supermodel::dialect::encode::Encoder for Encoder {
    fn new() -> Self {
        Self(String::new())
    }

    fn box_clone(&self) -> Box<dyn supermodel::dialect::encode::Encoder> {
        Box::new(Encoder(self.0.clone()))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}