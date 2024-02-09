use crate::dialect::Dialect;
use crate::dialect::encode::Encoder;
use crate::field::Field;

pub trait Query: Clone {
    fn to_sql<D: Dialect>(self) -> String;
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum Execute {
    RegisterModel {
        name: &'static str,
        fields: &'static [Field],
    }
}

impl Query for Execute {
    #[inline]
    fn to_sql<D: Dialect>(self) -> String {
        match self {
            Self::RegisterModel { name, fields, .. } => {
                format!("create table if not exists {}({});", name, join_fields::<D>(fields))
            }
        }
    }
}

#[non_exhaustive]
pub enum Fetch {
    Insert {
        model: &'static str,
        fields: &'static [Field],
        values: Box<dyn Encoder>,
        returning: &'static [Field],
    }
}

impl Clone for Fetch {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            &Self::Insert { model, fields, ref values, returning } => {
                Self::Insert { model, fields, values: values.as_ref().box_clone(), returning }
            }
        }
    }
}

impl Query for Fetch {
    #[inline]
    fn to_sql<D: Dialect>(self) -> String {
        match self {
            Self::Insert { model, fields, values, returning } => {
                let mut buf = format!("insert into {}({}) values ({})", model, join_fields::<D>(fields), values.as_str());
                if !returning.is_empty() {
                    let joint_fields = join_fields::<D>(returning);
                    buf.push_str("returning ");
                    buf.push_str(&joint_fields);
                }
                buf.push(';');
                buf
            }
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum FetchAll {
}

impl Query for FetchAll {
    #[inline]
    fn to_sql<D: Dialect>(self) -> String {
        todo!()
    }
}

fn join_fields<D: Dialect>(fields: &[Field]) -> String {
    fields.iter()
        .map(|field| format!("{} {}", field.name, D::get_data_type_sql(&field.data_type)))
        .collect::<Vec<_>>()
        .join(", ")
}