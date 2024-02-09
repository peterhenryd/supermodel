#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
)]
#![allow(clippy::missing_trait_methods)]
#![allow(clippy::ref_patterns)]
#![allow(clippy::todo)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::pub_use)]
#![allow(clippy::mod_module_files)]
#![allow(async_fn_in_trait)]

use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{ItemStruct, parse_macro_input, Token};

mod field;
mod fields_impl;
mod insertion_impl;
mod joined;
mod model;
mod model_impl;
mod read_impl;
mod string_tokens;

#[proc_macro_attribute]
pub fn model(attr: TokenStream, input: TokenStream) -> TokenStream {
    let dialects = Punctuated::<Ident, Token![,]>::parse_terminated
        .parse(attr)
        .expect("you must select a dialect for your model, e.g. #[model(Postgres)]");
    let item_struct = parse_macro_input!(input as ItemStruct);
    model::generate(item_struct, dialects.iter()).into()
}