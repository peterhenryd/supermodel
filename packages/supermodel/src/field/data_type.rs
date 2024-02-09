use crate::field::Id;

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum DataType {
    String,
    Id
}

impl DataType {
    #[inline]
    #[must_use]
    pub fn to_postgresql(&self) -> String {
        match *self {
            Self::String => "varchar(30)".to_owned(),
            Self::Id => "serial8 primary key".to_owned(),
        }
    }
}

pub trait DataValue {
    const DATA_TYPE: DataType;
}

impl DataValue for String {
    const DATA_TYPE: DataType = DataType::String;
}


impl DataValue for Id {
    const DATA_TYPE: DataType = DataType::Id;
}