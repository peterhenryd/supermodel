use case::CaseExt;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use crate::field::Fields;
use crate::fields_impl::FieldsImpl;
use crate::insertion_impl::InsertionImpl;
use crate::joined::join;
use crate::model_impl::ModelImpl;
use crate::read_impl::ReadImpl;
use crate::string_tokens::StringTokens;

pub struct Model {
    pub fields: Fields,
    pub type_name: StringTokens,
    pub module_name: StringTokens,
}

pub fn generate<'generator>(item_struct: syn::ItemStruct, dialects: impl Iterator<Item = &'generator Ident>) -> TokenStream {
    let fields = match Fields::try_from(item_struct.fields) {
        Ok(x) => x,
        Err(message) => panic!("{}", message)
    };

    let module_name = item_struct.attrs.iter()
        .flat_map(|attr| attr.meta.require_name_value())
        .filter(|meta| !meta.path.segments.is_empty())
        .filter(|meta| &meta.path.segments.iter().next().unwrap().ident.to_string() == "name")
        .find_map(|meta| match &meta.value {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => Some(lit_str.value()),
            _ => None
        })
        .unwrap_or_else(|| item_struct.ident.to_string().to_snake());

    let model = Model {
        fields,
        type_name: item_struct.ident.into(),
        module_name: module_name.into(),
    };
    let module_name = &model.module_name.ident;

    let mut inner = vec![];
    let mut outer = vec![];

    for dialect in dialects {
        outer.push(ModelImpl { model: &model, dialect }.to_token_stream());

        inner.push(quote! { use super::#dialect; });
        inner.push(FieldsImpl { model: &model, dialect }.to_token_stream());
        inner.push(ReadImpl { model: &model, dialect }.to_token_stream());
        inner.push(InsertionImpl { model: &model, dialect }.to_token_stream());
    }

    let inner = join(inner.iter(), &TokenStream::default());
    let outer = join(outer.iter(), &TokenStream::default());

    let tokens = quote! {
        #[allow(non_upper_case_globals)]
        pub mod #module_name {
            use std::marker::PhantomData;
            use supermodel::dialect::Dialect;
            use supermodel::dialect::encode::Encode;
            use supermodel::dialect::encode::Encoder;
            use supermodel::dialect::decode::Decode;
            use supermodel::dialect::decode::Decoder;
            use supermodel::dialect::decode::Error as DecodeError;
            use supermodel::field::data_type::DataValue;
            use supermodel::field::Field;
            use supermodel::field::Typed;
            use supermodel::field::Id;
            use supermodel::response::Response;
            use supermodel::response;

            pub struct Fields<D>(PhantomData<D>);

            impl<D> Default for Fields<D> {
                fn default() -> Self {
                    Self(Default::default())
                }
            }

            #inner
        }

        #outer
    };

    println!("{}", &tokens);

    tokens
}