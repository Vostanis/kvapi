use super::common::Separator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_str, Expr, Ident, LitStr, Token,
};

/// Collection of all headers for a HTTP client.
///
/// ```rust
/// {
///    "Header Name": "Header Value"
///    "AnotherHeader": &std::env::var("ENV_HEADER")?
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Headers {
    pub client: Vec<TokenStream>,
    pub query: Vec<TokenStream>,
}

impl Parse for Headers {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut headers: Vec<Header> = vec![];
        let mut client_headers: Vec<TokenStream> = vec![];
        let mut query_headers: Vec<TokenStream> = vec![];

        // parse
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
        let _ = headers
            .iter()
            .map(|header| {
                let key = &header.key;
                let value = parse_str::<Expr>(&header.value).expect("expected Header");

                // match the header to the correct TokenStream
                if header.is_query == true {
                    query_headers.push(quote! {
                        // url query
                        .header(#key, #value)
                    });
                } else {
                    client_headers.push(quote! {
                        // client query
                        let value = #value;
                        headers.insert(#key, kvapi::HeaderValue::from_str(value).unwrap());
                    });
                }
            })
            .collect::<Vec<_>>();

        Ok(Self {
            client: client_headers,
            query: query_headers,
        })
    }
}

/// A single header entry for a HTTP client.
///
/// "Header Name": "Header Value"
#[derive(Hash, Eq, PartialEq)] // HashSet used for efficient uniqueness
pub struct Header {
    pub key: String,
    pub value: String, // this includes Exprs (function calls, etc.)
    pub is_query: bool,
}

impl Parse for Header {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut is_query = false;

        // attr
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            let attrs;
            bracketed!(attrs in input);
            let _ = attrs
                .parse_terminated(Attr::parse, Token![,])?
                .into_iter()
                .map(|attr| match attr.key.to_string().as_str() {
                    "query" => is_query = true,
                    "client" => {}
                    _ => panic!("unexpected header attribute"),
                })
                .collect::<Vec<()>>();
        }

        // header entry
        let key: String = input.parse::<LitStr>()?.value();
        input.parse::<Separator>()?;
        let value: Expr = input.parse()?;
        let value = quote!( #value ).to_string();

        Ok(Self {
            key,
            value,
            is_query,
        })
    }
}

/// #[per_query]
/// "Custom-Header": my_function(self.url)
///
/// This would signify a header that is dependent on the query, and
/// so is added at the time of the query being built/sent, rather then
/// when the client is built.
pub struct Attr {
    pub key: Ident,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Ident>()?;
        Ok(Self { key })
    }
}
