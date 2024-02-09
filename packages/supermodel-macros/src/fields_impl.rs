use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use crate::field::Field;
use crate::joined::{join, join_with_commas};
use crate::model::Model;
use crate::string_tokens::StringTokens;

pub struct FieldsImpl<'generator> {
    pub model: &'generator Model,
    pub dialect: &'generator syn::Ident
}

impl<'generator> ToTokens for FieldsImpl<'generator> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dialect = &self.dialect;
        let mut fields = vec![];

        for field in self.model.fields.iter() {
            let Field { name: StringTokens { lit_str, ident, .. }, rust_type } = field;
            let val = quote! {
                Typed {
                    field: Field {
                        name: #lit_str,
                        data_type: <#rust_type as DataValue>::DATA_TYPE
                    },
                    _marker: PhantomData,
                }
            };

            fields.push(quote! {
                pub const #ident: Typed<#rust_type> = #val;
            });
        }

        let fields = join(fields.iter(), &TokenStream::default());
        tokens.append_all(quote! {
            #fields
        });

        let mut field_names = vec![];
        for Field { name: StringTokens { ident, .. }, .. } in &self.model.fields.defined {
            field_names.push(quote!(#ident.field));
        }
        let field_names = join_with_commas(field_names.iter());

        tokens.append_all(quote! {
            impl supermodel::field::Fields<#dialect> for Fields<#dialect> {
                const DEFINED_FIELDS: &'static [Field] = &[#field_names];
                const TOTAL_FIELDS: &'static [Field] = &[id.field, #field_names];

                type Insertion = Insertion;
            }
        });
    }
}