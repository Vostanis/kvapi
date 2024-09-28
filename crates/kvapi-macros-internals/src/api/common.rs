use std::collections::HashSet;
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

/// Seperator token; one of: `:`, `=`, `->`, or `=>`.
#[derive(Debug, PartialEq)]
pub enum Separator {
    Colon,
    Equals,
    Arrow,
}

impl Parse for Separator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Ok(Self::Colon)
        } else if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Ok(Self::Equals)
        } else if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            Ok(Self::Arrow)
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected one of ':', '=', or '->'",
            ))
        }
    }
}


/// Remember the file types to ignore when parsing the endpoint.
pub fn file_types() -> HashSet<&'static str> {
    let mut set: HashSet<&str> = HashSet::new();
    vec!["json", "csv", "xml", "toml", "yaml", "html", "htm"]
        .iter()
        .for_each(|file_type| {
            set.insert(file_type);
        });
    set
}
