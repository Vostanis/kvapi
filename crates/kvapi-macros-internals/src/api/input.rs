use super::{common::Separator, dict::Dict, headers::Headers};
use std::collections::HashMap;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Expr,
};

/// Input for the `api! { #input }` macro.
///
/// ```rust
/// name:       PascalStructName
/// dict:       { "endpoint": Type }
/// headers:    { "Header Name": "Header Value" }
/// query:      "?query_param"
/// ```
pub struct Input {
    pub name: TokenStream,
    pub dict: Dict,
    pub base: Option<TokenStream>,
    pub headers: Option<Headers>,
    pub query: Option<Expr>,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut api = Self {
            name: quote!(),
            dict: Dict { inner: HashMap::new(), },
            base: None,
            headers: None,
            query: None,
        };

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Separator>()?;
            match ident.to_string().as_str() {
                "name" => {
                    let name: Ident = input.parse()?;
                    api.name = quote!( #name );
                }
                "base" => {
                    let base: LitStr = input.parse()?;
                    let base = base.value();
                    api.base = Some(quote!(#base));
                }
                "dict" => {
                    let dict: Dict = input.parse()?;
                    api.dict = dict;
                }
                "headers" => {
                    let headers: Headers = input.parse()?;
                    api.headers = Some(headers);
                }
                "query" => {
                    let query: Expr = input.parse()?;
                    api.query = Some(query);
                }
                _ => return Err(syn::Error::new(ident.span(), "unknown field")),
            }
        }

        Ok(api)
    }
}
