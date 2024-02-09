use proc_macro2::Span;

pub struct StringTokens {
    pub string: String,
    pub lit_str: syn::LitStr,
    pub ident: syn::Ident
}

impl From<syn::Ident> for StringTokens {
    fn from(ident: syn::Ident) -> Self {
        let string = ident.to_string();
        let lit_str = syn::LitStr::new(&string, Span::call_site());
        Self { string, lit_str, ident }
    }
}

impl From<String> for StringTokens {
    fn from(string: String) -> Self {
        let lit_str = syn::LitStr::new(&string, Span::call_site());
        let ident = syn::Ident::new(&string, Span::call_site());
        Self { string, lit_str, ident }
    }
}