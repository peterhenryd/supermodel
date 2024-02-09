use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::token::Comma;

pub fn join(
    mut iter: impl Iterator<Item=impl ToTokens>,
    separator: impl ToTokens + Copy
) -> TokenStream {
    let mut tokens = TokenStream::new();

    if let Some(value) = iter.next() { value.to_tokens(&mut tokens); }
    for value in iter {
        separator.to_tokens(&mut tokens);
        value.to_tokens(&mut tokens);
    }

    tokens
}

pub fn join_with_commas(iter: impl Iterator<Item=impl ToTokens>) -> TokenStream {
    join(iter, Comma::default())
}