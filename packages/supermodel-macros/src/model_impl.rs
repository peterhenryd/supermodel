use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use crate::field::Field;
use crate::joined::join_with_commas;
use crate::model::Model;
use crate::string_tokens::StringTokens;

pub struct ModelImpl<'generator> {
    pub model: &'generator Model,
    pub dialect: &'generator syn::Ident
}

impl<'generator> ToTokens for ModelImpl<'generator> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let dialect = self.dialect;
        let type_name = &self.model.type_name.ident;
        let module_name_ident = &self.model.module_name.ident;
        let module_name_lit_str = &self.model.module_name.lit_str;

        let fields = join_with_commas(self.model
            .fields
            .defined
            .iter()
            .map(|Field { name: StringTokens { ident, .. }, rust_type, .. }|
                quote! { pub #ident: #rust_type }
            )
        );

        tokens.append_all(quote! {
            pub struct #type_name {
                id: supermodel::field::Id,
                #fields,
            }
        });

        tokens.append_all(quote!{
            impl supermodel::model::Model<#dialect> for #type_name {
                type Fields = #module_name_ident::Fields<#dialect>;

                const NAME: &'static str = #module_name_lit_str;

                fn id(&self) -> supermodel::field::Id {
                    self.id
                }
            }
        });
    }
}