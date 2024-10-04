use super::headers::Headers;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;
use syn::Ident;

/// Nodes are the segments of the key in `"key": EndpointType` in a Dict Entry
///
/// e.g.,
///     "path/to/endpoint": MyType,
///
/// path: Path                              <- root
///         |_ to: To                       <- child of `path`
///                 |_ endpoint: Endpoint   <- leaf & child of `to`
///
///                    impl Endpoint {
///                         async fn get() -> Result<MyType> { ... }
///                    }
///
/// The struct build of this would result in:
/// `SomeApiName.path.to.endpoint.get()`
///
/// note 1:  `endpoint`, being a leaf node, gets access to the `get()` function.
/// note 2:  each struct will have a `new()` impl.
#[derive(Clone, Debug)]
pub struct Node {
    pub root: bool,                   // add to the Root fields
    pub de_type: Option<TokenStream>, // type of the `get()` result; if none, no `get()` needed
    pub children: HashSet<String>,    // determines `new()` tokens
    pub endpoint: Option<TokenStream>, // if leaf node, remember the original endpoint for `url()`
                                      // (and any additional query)
}

impl Node {
    // used in in building the dictionary
    pub(crate) fn new() -> Self {
        Self {
            root: false,
            de_type: None,
            children: HashSet::new(),
            endpoint: None,
        }
    }

    // remake children nodes in field TokenStream
    pub(crate) fn children_fields(&self, api_name: Ident) -> Vec<TokenStream> {
        self.children
            .iter()
            .map(|child| {
                let (snake, pascal) = (
                    format_ident!("{}", child.to_case(Case::Snake)),
                    format_ident!("{}{}", api_name, child.to_case(Case::Pascal)),
                );
                quote! { #snake: #pascal }
            })
            .collect()
    }

    // check if the node is an HTTP node
    pub(crate) fn is_http(&self) -> bool {
        if self.de_type.is_some() {
            true
        } else {
            false
        }
    }

    // build the url, combining with a base URl, if there is one
    pub(crate) fn build_url(&self, base: Option<TokenStream>) -> TokenStream {
        let url = self.endpoint.as_ref().unwrap();
        if let Some(base) = base {
            quote! {
                #url
                let base = #base;
                let url = format!("{}{}", base, url);
            }
        } else {
            url.clone()
        }
    }

    // build the HTTP functions
    pub(crate) fn build_http(&self, url: TokenStream, headers: Option<Headers>) -> TokenStream {
        let de_type = self.de_type.clone().unwrap();
        let headers = headers.unwrap_or(Headers { inner: vec![] }).inner;

        let http_methods = quote! {
            fn build_client() -> anyhow::Result<reqwest::Client> {
                let mut headers = reqwest::header::HeaderMap::new();
                #( #headers )*
                let client = reqwest::ClientBuilder::new()
                    .default_headers(headers)
                    .build()?;
                Ok(client)
            }

            fn build_url() -> String {
                #url
                url
            }

            fn url(&self) -> &String {
                &self.url
            }

            fn client(&self) -> &reqwest::Client {
                &self.client
            }

            pub async fn get(&self) -> reqwest::Result<#de_type> {
                let response: #de_type = self
                    .client()
                    .get(self.url())
                    .send()
                    .await?
                    .json()
                    .await?;
                Ok(response)
            }

            pub async fn post(&self, json: serde_json::Value) -> reqwest::Result<#de_type> {
                let response: #de_type = self
                    .client()
                    .post(self.url())
                    .json(&json)
                    .send()
                    .await?
                    .json()
                    .await?;
                Ok(response)
            }
        };

        http_methods
    }

    // check if the node is a root node
    pub(crate) fn is_root(&self) -> bool {
        self.root
    }
}
