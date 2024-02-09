use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use crate::field::Field;
use crate::joined::join_with_commas;
use crate::model::Model;
use crate::string_tokens::StringTokens;

pub struct ReadImpl<'generator> {
    pub model: &'generator Model,
    pub dialect: &'generator syn::Ident
}

impl<'generator> ToTokens for ReadImpl<'generator> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dialect = self.dialect;
        let mut fields = self.model.fields.iter();

        let id_field = fields.next().unwrap();
        let id_ident = &id_field.name.ident;
        let id_type = &id_field.rust_type;

        let mut arguments = vec![quote! {
            #id_ident: match <#id_type as Decode>::decode(&mut decoder) {
                Ok(ok) => ok,
                Err(err) => return Some(Err(err)),
            }
        }];

        for field in fields {
            let Field { name: StringTokens { ident, .. }, rust_type, .. } = field;

            arguments.push(quote! {
                #ident: match <#rust_type as Decode>::decode(&mut decoder) {
                    Ok(ok) => ok,
                    Err(err) => return Some(Err(err)),
                }
            });
        }

        let arguments = join_with_commas(arguments.iter());
        let name = &self.model.type_name.ident;
        tokens.append_all(quote! {
            impl response::From<#dialect> for super::#name {
                fn from_response(response: <#dialect as Dialect>::Response) -> Option<Result<Self, DecodeError>> {
                    let mut decoder = response.into_decoder()?;
                    Some(Ok(Self { #arguments }))
                }
            }
        });
    }
}