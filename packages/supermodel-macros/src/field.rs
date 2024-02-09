use core::slice::Iter;
use quote::quote;
use crate::string_tokens::StringTokens;

#[derive(Debug)]
pub struct Field {
    pub name: StringTokens,
    pub rust_type: syn::Type,
}

#[derive(Debug)]
pub struct Fields {
    pub id: Field,
    pub defined: Vec<Field>,
}

pub struct FieldsIter<'field>(Iter<'field, Field>, Option<&'field Field>);

impl<'field> Iterator for FieldsIter<'field> {
    type Item = &'field Field;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1.is_some() {
            self.1.take()
        } else if let Some(field) = self.0.next() {
            Some(field)
        } else {
            None
        }
    }
}

impl Fields {
    pub fn iter(&self) -> FieldsIter {
        FieldsIter(self.defined.iter(), Some(&self.id))
    }
}

impl TryFrom<syn::Fields> for Fields {
    type Error = &'static str;

    fn try_from(fields: syn::Fields) -> Result<Self, Self::Error> {
        let named_fields = match fields {
            syn::Fields::Named(fields) => fields.named,
            _ => return Err("structs that derive Model must have named fields")
        };
        let mut defined = vec![];
        for field in named_fields {
            defined.push(Field {
                name: field.ident.clone().unwrap().into(),
                rust_type: field.ty,
            });
        }

        let id = Field {
            name: "id".to_string().into(),
            rust_type: syn::Type::Verbatim(quote!(Id)),
        };

        Ok(Self { id, defined })
    }
}