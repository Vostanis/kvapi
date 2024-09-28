use super::common::Separator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_str, Expr, LitStr, Token,
};

/// Collection of all headers for a HTTP client.
///
/// {
///    "Header Name": "Header Value"
///    "AnotherHeader": &std::env::var("ENV_HEADER")
/// }
#[derive(Debug)]
pub struct Headers {
    pub inner: Vec<TokenStream>,
}

impl Parse for Headers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut headers: Vec<Header> = vec![];
        let args;
        braced!(args in input);
        while !args.is_empty() {
            let header: Header = args.parse()?;
            if !headers.contains(&header) {
                headers.push(header);
            }
            args.parse::<Option<Token![,]>>()?;
        }

        // transformed directly to final TokenStream output (can just be expanded out easily)
        let headers = headers
            .iter()
            .map(|header| {
                let key = &header.key;
                let value = parse_str::<Expr>(&header.value).expect("expected Header");
                quote! {
                    let value = #value;
                    headers.insert(#key, reqwest::header::HeaderValue::from_str(value).unwrap());
                }
            })
            .collect::<Vec<TokenStream>>();

        Ok(Self { inner: headers })
    }
}

/// A single header entry for a HTTP client.
///
/// "Header Name": "Header Value"
#[derive(Hash, Eq, PartialEq)] // HashSet used for efficient uniqueness
pub struct Header {
    pub key: String,
    pub value: String, // this includes Exprs (function calls, etc.)
}

impl Parse for Header {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: String = input.parse::<LitStr>()?.value();
        input.parse::<Separator>()?;
        let value: Expr = input.parse()?;
        let value = quote!( #value ).to_string();

        Ok(Self { key, value })
    }
}
