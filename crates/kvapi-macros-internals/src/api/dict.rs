use super::common::{file_types, Separator};
use super::node::Node;
use quote::quote;
use std::collections::HashMap;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    Expr, Ident, LitStr, Token, Type,
};

/// ```rust
/// dict: {
///   #[query("/append/this/string"), rename("rename_to_this")]
///   "my_endpoint": MyType,
///
///   "another/endpoint": AnotherType,
///   "a/third/endpoint": ThisType,
/// }
/// ```
#[derive(Debug)]
pub struct Dict {
    pub inner: HashMap<String, Node>,
}

impl Parse for Dict {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut inner: HashMap<String, Node> = HashMap::new();
        let content;
        braced!(content in input);
        while !content.is_empty() {
            let entry = content.parse::<Entry>()?;

            // >> rename attr
            // if rename == "some_value", then retain the original endpoint, but replace the naming convention.
            let fields = match entry.rename {
                Some(new_name) => new_name,
                None => entry.endpoint.clone(),
            };

            // split the name into segments, filtering out empties & file types (".json", ".csv", etc.).
            let fields: Vec<&str> = fields
                .split(&['/', '.'][..])
                .filter(|segment| !segment.is_empty() && !file_types().contains(segment))
                .collect();

            // iterate over each segment, establishing a node for each.
            let last = fields.len() - 1;
            for (i, field) in fields.iter().enumerate() {
                let node = inner.entry(field.to_string()).or_insert_with(Node::new);

                // >> is it a root node?
                // if so, it will need to be tagged now, in order to add it to the root fields later.
                if i == 0 {
                    node.root = true;
                }

                // >> is it a leaf node?
                // if so, it will need access to HTTP impls (`url()`, `client()` & `get()`) when building the TokenStreams.
                // if not, remember the child nodes for building this struct's fields.
                //
                // remember: each node will need a `new()` impl, so child nodes will be used in building those TokenStreams, also.
                if i == last {
                    let endpoint = entry.endpoint.clone();
                    let query = entry.query.clone();

                    // since we don't have the `base` url yet, take a segment of the eventual TokenStream (stringified for easy storage).
                    // if there is Some(query), then append it now, otherwise, just include the original.
                    //
                    // also, include the type.
                    if let Some(query) = query {
                        node.endpoint = Some(quote! {
                            let url = format!("{}{}", #endpoint, #query);
                        })
                    } else {
                        node.endpoint = Some(quote! {
                            let url = String::from(#endpoint);
                        })
                    };

                    let de_type = entry.de_type.clone();
                    node.de_type = Some(quote!( #de_type ));
                } else {
                    node.children.insert(fields[i + 1].to_string());
                }
            }
            content.parse::<Option<Token![,]>>()?;
        }

        // error if no dict entries
        if inner.is_empty() {
            return Err(syn::Error::new(
                content.span(),
                "Dictionary of `\"endpoints\": DataTypes` is required",
            ));
        }

        Ok(Self { inner })
    }
}

/// Parse a single Record of a Dict - this includes: endpoint, type, queries, and rename.
///
/// ```rust
/// #[query: "/append/this/string", rename: "rename_to_this"]
/// "my_endpoint": MyType,
/// ```
pub struct Entry {
    pub endpoint: String,
    pub de_type: Type,
    pub query: Option<Expr>,
    pub rename: Option<String>,
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut query: Option<Expr> = None;
        let mut rename: Option<String> = None;

        // parse any attributes: `#[ ... ]`
        // let lookahead = input.lookahead1();
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;

            let attrs;
            bracketed!(attrs in input);
            let _ = attrs
                .parse_terminated(Attr::parse, Token![,])?
                .into_iter()
                .map(|attr| match attr.fn_id.to_string().as_str() {
                    "query" => {
                        query = Some(attr.arg);
                        Ok(())
                    }

                    // `rename` attr actually requires LitStr input;
                    // enforcing LitStr inputs at this point is easier than defining an enum for attr inputs.
                    "rename" => {
                        let arg = match attr.arg {
                            Expr::Lit(expr_lit) => match expr_lit.lit {
                                syn::Lit::Str(lit_str) => lit_str.value(),
                                _ => {
                                    return Err(syn::Error::new(
                                        input.span(),
                                        "rename arg must be a string literal",
                                    ))
                                }
                            },
                            _ => {
                                return Err(syn::Error::new(
                                    input.span(),
                                    "rename arg must be a string literal",
                                ))
                            }
                        };
                        rename = Some(arg);
                        Ok(())
                    }
                    _ => {
                        return Err(syn::Error::new(
                            input.span(),
                            "dict macro input not recognised",
                        ))
                    }
                })
                .collect::<Vec<_>>();
        }

        // then, parse `"LitStr": Type`
        let endpoint = input.parse::<LitStr>()?.value();
        input.parse::<Separator>()?;
        let de_type = input.parse::<Type>()?;
        Ok(Self {
            endpoint,
            de_type,
            query,
            rename,
        })
    }
}

/// Attribute for a Dict entry;
///
/// ```rust
/// dict: {
///    #[#fn_id -> #arg] // <-- Attr
///    "my_endpoint": MyType,
/// }
/// ```
///
/// Possible attributes:
///     - query = format!("?api_key_in_the_url", API_KEY)
///     - rename = "new_name"
pub struct Attr {
    pub fn_id: Ident,
    pub arg: Expr,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fn_id = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "expected function identifier in dict attr")
        })?;
        input.parse::<Separator>()?;
        let arg = input.parse()?;
        Ok(Attr { fn_id, arg })
    }
}
