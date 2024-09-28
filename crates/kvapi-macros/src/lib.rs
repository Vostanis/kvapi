use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn api(input: TokenStream) -> TokenStream {
    use kvapi_macros_internals::api::{tokens::ApiComponents, Input};

    let input = parse_macro_input!(input as Input);
    let name = input.name.clone();

    let components = ApiComponents::build(input);
    let root_fields = components.root_fields;
    let other_nodes = components.other_nodes;

    quote! {
        pub struct #name {
            #( #root_fields, )*
        }
        impl #name {
            pub fn new() -> Self {
                Self {
                    #( #root_fields::new(), )*
                }
            }
        }
        #( #other_nodes )*
    }
    .into()
}
