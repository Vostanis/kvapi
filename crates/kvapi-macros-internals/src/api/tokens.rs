use super::{headers::Headers, Input};
use convert_case::{
    Case::{Pascal, Snake},
    Casing,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct ApiComponents {
    pub root_fields: Vec<TokenStream>,
    pub other_nodes: Vec<TokenStream>,
}

impl ApiComponents {
    pub fn build(input: Input) -> Self {
        let mut api = Self {
            root_fields: vec![],
            other_nodes: vec![],
        };

        let headers = input.headers.unwrap_or(Headers { inner: vec![] }).inner;

        // loop all the nodes from the input and build the TokenStreams
        for (field, node) in input.dict.inner {
            let (snake, pascal) = (
                format_ident!("{}", field.to_case(Snake)),
                format_ident!("__{}", field.to_case(Pascal)),
            );

            // >> is it a root field?
            // if so, save `field: Field` for the TokenStreams.
            if node.root {
                let field_def = quote! { #snake: #pascal };
                api.root_fields.push(field_def);
            }

            // >> are there child nodes?
            // if so, save `child: Child` for the definition & `new()`
            let children: Vec<TokenStream> = node
                .children
                .into_iter()
                .map(|child| {
                    let (c_snake, c_pascal) = (
                        format_ident!("{}", child.to_case(Snake)),
                        format_ident!("__{}", child.to_case(Pascal)),
                    );
                    quote! { #c_snake: #c_pascal, }
                })
                .collect();

            // all nodes will need a `new()` impl, using the children above.
            // if `children` is empty, then it's just an empty function,
            // but we make it, regardless.
            // api.other_nodes.push(quote! {
            //     pub struct #pascal {
            //         #( #children, )*
            //     }

            //     impl #pascal {
            //         fn new() -> Self {
            //             Self {
            //                 #( #children::new(), )*
            //             }
            //         }
            //     }
            // });

            // >> is it a leaf node?
            // it will have a Type if it is.
            //
            // if so, it needs a collection of HTTP-related functions, like `get()`.
            // if not, we can just ignore it.
            match node.de_type {
                // if there is no deserialize type, then it is not an endpoint
                None => {
                    api.other_nodes.push(quote! {
                        pub struct #pascal {
                            #( #children, )*
                        }

                        impl #pascal {
                            fn new() -> Self {
                                Self {
                                    #( #children::new(), )*
                                }
                            }
                        }
                    });
                }

                // if there is deserialize type, it's an endpoint and will need a Client, HttpMethods, etc.
                Some(de_type) => {
                    // unpack the url, dependent on the `base` variable.
                    let endpoint = node.endpoint.unwrap();
                    let url = if let Some(ref base) = input.base {
                        quote! {
                            let base = #base;
                            #endpoint
                            let url = format!("{}{}", base, url);
                        }
                    } else {
                        quote! {
                            #endpoint
                        }
                    };

                    let url = if let Some(ref query) = input.query {
                        quote! {
                            #url
                            let url = format!("{}{}", url, #query);
                        }
                    } else {
                        url
                    };

                    let url = quote! {
                        #url
                        url
                    };

                    api.other_nodes.push(quote! {
                        pub struct #pascal {
                            client: reqwest::Client,
                            url: String,
                            #( #children, )*
                        }

                        impl #pascal {
                            fn new() -> Self {
                                Self {
                                    client: Self::build_client(),
                                    url: Self::build_url(),
                                    #( #children::new(), )*
                                }
                            }

                            fn build_client() -> reqwest::Client {
                                let mut headers = reqwest::header::HeaderMap::new();
                                #( #headers )*
                                reqwest::ClientBuilder::new()
                                    .default_headers(headers)
                                    .build()
                                    .expect("failed to buy a Client")
                            }

                            fn build_url() -> String {
                                #url
                            }

                            fn url(&self) -> &String {
                                &self.url
                            }

                            fn client(&self) -> &reqwest::Client {
                                &self.client
                            }

                            // optional
                            pub fn dbg_url(&self) {
                                println!("{}", self.url());
                            }

                            pub fn dbg_client(&self) {
                                println!("{:#?}", self.client());
                            }

                            pub async fn get(&self) -> reqwest::Result<#de_type> {
                                let value = self
                                    .client()
                                    .get(self.url())
                                    .send()
                                    .await?
                                    .json::<#de_type>()
                                    .await?;
                                Ok(value)
                            }

                            pub async fn post(&self, json: serde_json::Value) -> reqwest::Result<#de_type> {
                                let value = self
                                    .client()
                                    .post(self.url())
                                    .json(&json)
                                    .send()
                                    .await?
                                    .json::<#de_type>()
                                    .await?;
                                Ok(value)
                            }
                        }
                    });

                    // // if #[debug] included, add DebugHttpClient trait
                    // if debug {
                    //     api.other_nodes.push(quote! {
                    //         impl kvapi::DebugHttpClient for #pascal {}
                    //     });
                    // }
                }
            }

            // >> is it a leaf node?
            // it will have a Type if it is.
            //
            // if so, it needs a collection of HTTP-related functions, like `get()`.
            // if not, we can just ignore it.
            // if let Some(de_type) = node.de_type {
            //     // unpack the url, dependent on the `base` variable.
            //     let endpoint = node.endpoint.unwrap();
            //     let url = if let Some(ref base) = input.base {
            //         quote! {
            //             let base = #base;
            //             #endpoint
            //             let url = format!("{}{}", base, url);
            //         }
            //     } else {
            //         quote! {
            //             #endpoint
            //         }
            //     };

            //     let url = if let Some(ref query) = input.query {
            //         quote! {
            //             #url
            //             let url = format!("{}{}", url, #query);
            //         }
            //     } else {
            //         url
            //     };

            //     let url = quote! {
            //         #url
            //         url
            //     };

            //     // push the HTTP impl collection
            //     api.other_nodes.push(quote! {
            //         impl #pascal {
            //             pub fn url() -> String {
            //                 #url
            //             }
            //             pub fn dbg_url(&self) -> () {
            //                 println!("{}", Self::url())
            //             }
            //             pub fn client() -> Result<reqwest::Client, reqwest::Error> {
            //                 let mut headers = reqwest::header::HeaderMap::new();
            //                 #( #headers )*
            //                 reqwest::ClientBuilder::new()
            //                     .default_headers(headers)
            //                     .build()
            //             }
            //             pub fn dbg_client(&self) -> () {
            //                 println!("{:#?}", Self::client())
            //             }
            //             pub async fn get(&self) -> Result<#de_type, reqwest::Error> {
            //                 let response = Self::client()?
            //                     .get(Self::url())
            //                     .send()
            //                     .await?
            //                     .json::<#de_type>()
            //                     .await?;
            //                 Ok(response)
            //             }
            //         }
            //     });
            // }
        }

        api
    }
}
