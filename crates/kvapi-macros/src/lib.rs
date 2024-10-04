use kvapi_macros_internals::api::builder::ApiBuilder;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn api(input: TokenStream) -> TokenStream {
    let api_builder = parse_macro_input!(input as ApiBuilder);
    api_builder.build().into()
}
