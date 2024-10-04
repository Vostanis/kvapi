use super::{common::Separator, dict::Dict, headers::Headers};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Ident, LitStr,
};

/// Input for the `api! { #input }` macro.
///
/// ```rust
/// name:       PascalStructName
/// dict:       { "endpoint": Type }
/// headers:    { "Header Name": "Header Value" }
/// query:      "?query_param"
/// ```
///
/// The Director will generate the API with an ApiBuilder.
pub struct ApiBuilder {
    // required
    pub name: Option<Ident>,
    pub dict: Option<Dict>,

    // optional
    pub base: Option<TokenStream>,
    pub headers: Option<Headers>,
    pub query: Option<Expr>,
}

impl ApiBuilder {
    pub fn build(self) -> TokenStream {
        let mut fields: Vec<TokenStream> = vec![];
        let mut nodes: Vec<TokenStream> = vec![];

        let dict = self.dict.expect("dict entry is required").inner;
        let api_name = self.name.expect("name entry is required");

        // build all fields from nodes
        for (name, node) in dict {
            // (snake_name, PascalName) == (field_name, StructName)
            let (snake, pascal) = (
                format_ident!("{}", name.to_case(Case::Snake)),
                format_ident!("{}{}", api_name, name.to_case(Case::Pascal)),
            );

            // api.fields
            if node.is_root() {
                fields.push(quote! { #snake: #pascal });
            }

            // api.nodes
            let fields = node.children_fields(api_name.clone());
            if node.is_http() {
                let url = node.build_url(self.base.clone());
                let http = node.build_http(url, self.headers.clone());

                // http node
                let node = quote! {
                    pub struct #pascal {
                        client: reqwest::Client,
                        url: String,
                        #( #fields, )*
                    }
                    impl #pascal {
                        pub fn new() -> Self {
                            Self {
                                client: Self::build_client().unwrap(),
                                url: Self::build_url(),
                                #( #fields::new(), )*
                            }
                        }
                        #http
                    }
                };

                nodes.push(node)
            } else {
                // non-http node
                let node = quote! {
                    pub struct #pascal {
                        #( #fields, )*
                    }

                    impl #pascal {
                        pub fn new() -> Self {
                            Self {
                                #( #fields::new(), )*
                            }
                        }
                    }
                };
                nodes.push(node);
            }
        }

        // return the final TokenStream
        quote! {
            pub struct #api_name {
                #( #fields, )*
            }
            impl #api_name {
                pub fn new() -> Self {
                    Self {
                        #( #fields::new(), )*
                    }
                }
            }
            #( #nodes )*
        }
    }
}

impl Parse for ApiBuilder {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut api = Self {
            // required
            name: None,
            dict: None,

            // optional
            base: None,
            headers: None,
            query: None,
        };

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Separator>()?;
            match ident.to_string().as_str() {
                "name" | "N" => {
                    let name: Ident = input.parse()?;
                    api.name = Some(name);
                }
                "base" | "B" => {
                    let base: LitStr = input.parse()?;
                    let base = base.value();
                    api.base = Some(quote!(#base));
                }
                "dict" | "D" => {
                    let dict: Dict = input.parse()?;
                    api.dict = Some(dict);
                }
                "headers" | "head" | "hdrs" | "H" => {
                    let headers: Headers = input.parse()?;
                    api.headers = Some(headers);
                }
                "query" | "Q" => {
                    let query: Expr = input.parse()?;
                    api.query = Some(query);
                }
                _ => return Err(syn::Error::new(ident.span(), "unknown input to `api!`")),
            }
        }

        // guarantee `name` & `dict`
        if api.name.is_none() {
            panic!("`name` is required; the name of the struct identity")
        }

        if api.dict.is_none() {
            panic!("`dict` is required; a list of endpoints and output types")
        }

        Ok(api)
    }
}
