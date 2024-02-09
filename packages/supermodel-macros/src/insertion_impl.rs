use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use crate::field::Field;
use crate::joined::{join, join_with_commas};
use crate::model::Model;
use crate::string_tokens::StringTokens;

pub struct InsertionImpl<'generator> {
    pub model: &'generator Model,
    pub dialect: &'generator syn::Ident
}

impl<'generator> ToTokens for InsertionImpl<'generator> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dialect = self.dialect;

        let mut fields = vec![];
        let mut field_encoder = vec![];
        let mut boxed_fields = vec![];
        for Field { name: StringTokens { ident, .. }, rust_type, .. } in &self.model.fields.defined {
            fields.push(quote! {
                pub #ident: #rust_type
            });
            field_encoder.push(quote!( self.#ident.encode(&mut encoder); ));
            boxed_fields.push(quote!(Box::new(self.#ident)));
        }

        let fields = join_with_commas(fields.iter());
        let field_encoder = join(field_encoder.iter(), &TokenStream::new());
        let boxed_fields = join_with_commas(boxed_fields.iter());

        tokens.append_all(quote! {
            pub struct Insertion {
                #fields
            }

            impl supermodel::field::insertion::Insertion<#dialect> for Insertion {
                fn into_values(self) -> Vec<Box<dyn Encode>> {
                    vec![#boxed_fields]
                }

                fn encode_all(&self) -> <#dialect as Dialect>::Encoder {
                    let mut encoder = <#dialect as Dialect>::Encoder::new();
                    #field_encoder
                    encoder
                }
            }
        });
    }
}