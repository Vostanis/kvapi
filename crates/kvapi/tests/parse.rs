use quote::{format_ident, quote};

// Test all custom parsing builds, from `kvapi_macros_internals` crate.
//
// Note: many `syn` structs can't equate, e.g. `Expr`.

// api/common.rs
// =============
//
// Separator; parse `:`, `=`, or `->`
#[test]
fn parse_separator() {
    use kvapi_macros_internals::api::common::Separator;

    let input = quote! { : };
    let parsed = syn::parse2::<Separator>(input).expect("parse `:`");
    assert_eq!(parsed, Separator::Colon);

    let input = quote! { = };
    let parsed = syn::parse2::<Separator>(input).expect("parse `=`");
    assert_eq!(parsed, Separator::Equals);

    let input = quote! { -> };
    let parsed = syn::parse2::<Separator>(input).expect("parse `->`");
    assert_eq!(parsed, Separator::Arrow);
}

// api/dict.rs
// ===========
//
// Dict, Record, Attr
#[test]
fn parse_dict() {
    use kvapi_macros_internals::api::dict::Dict;

    // 1. no attrs
    let input = quote! {
        {
            "my_endpoint": MyType,
            "another_endpoint": AnotherType,
            "a_third_endpoint": ThirdType,
        }
    };
    syn::parse2::<Dict>(input).expect("parse Dict [no attrs]");

    // 2. rename attr
    let input = quote! {
        {
            #[rename="it_should_be_called_this"]
            "my_endpoint"=MyType,
            "another_endpoint": AnotherType,

            #[rename="call_it_this"]
            "a_third_endpoint" -> ThirdType,
        }
    };
    syn::parse2::<Dict>(input).expect("parse Dict [with rename attrs]");

    // 3. query attr
    let input = quote! {
        {
            #[query="?add_this_on_the_end"]
            "my_endpoint": MyType
            #[query="?add_this_on_the_end"]
            "another_endpoint": AnotherType,
            #[query="?add_this_on_the_end"]
            "a_third_endpoint": ThirdType
        }
    };
    syn::parse2::<Dict>(input).expect("parse Dict [with query attrs]");

    // 4. rename & query attrs
    let input = quote! {
        {
            #[rename="it_should_be_called_this", query="?add_this_on_the_end"]
            "my_endpoint": MyType
            "another_endpoint"=AnotherType,

            #[query: "?add_this", rename -> "call_it_this",]
            "a_third_endpoint" -> ThirdType,
        }
    };
    syn::parse2::<Dict>(input).expect("parse Dict [with rename & query attrs]");
}

#[test]
fn parse_dict_entry() {
    use kvapi_macros_internals::api::dict::Entry;

    // 1. no attr
    let input = quote! {
        "my_endpoint": MyType
    };
    let parsed = syn::parse2::<Entry>(input).expect("parse Record; no macro");
    let parsed_type = parsed.de_type;
    let parsed_type = quote!( #parsed_type );
    assert_eq!(parsed.endpoint, "my_endpoint".to_string());
    assert_eq!(quote!( #parsed_type ).to_string(), "MyType");

    // 2. with attr (rename)
    let input = quote! {
        #[rename="rename_to_this"]
        "/this/is/an/endpoint.json": std::collections::HashMap<String, i32>
    };
    let parsed = syn::parse2::<Entry>(input).expect("parse Record; with attr (rename)");
    let parsed_type = parsed.de_type;
    let parsed_type = quote!( #parsed_type );
    let parsed_rename = parsed.rename;
    let parsed_rename = quote!( #parsed_rename );
    assert_eq!(parsed.endpoint, "/this/is/an/endpoint.json".to_string());
    assert_eq!(
        quote!( #parsed_type ).to_string(),
        "std :: collections :: HashMap < String , i32 >" // TokenStream strings are spaced out
    );
    assert_eq!(
        Some(quote!( #parsed_rename ).to_string()),
        Some("\"rename_to_this\"".to_string())
    );

    // 3. with attr (query)
    let input = quote! {
        #[query="?add_this_on_the_end"]
        "/some_url": Map<String, Map<String, f32>>
    };
    let parsed = syn::parse2::<Entry>(input).expect("parse Record; with attr (query)");
    let parsed_type = parsed.de_type;
    let parsed_type = quote!( #parsed_type );
    let parsed_query = parsed.query;
    let parsed_query = quote!( #parsed_query );
    assert_eq!(parsed.endpoint, "/some_url".to_string());
    assert_eq!(
        quote!( #parsed_type ).to_string(),
        "Map < String , Map < String , f32 > >" // TokenStream strings are spaced out
    );
    assert_eq!(
        Some(quote!( #parsed_query ).to_string()),
        Some("\"?add_this_on_the_end\"".to_string())
    );

    // 4. with attrs (query & rename)
    let input = quote! {
        #[query -> "?add_this_on_the_end", rename: "renamed"]
        "/some_url": Map<String, Map<String, f32>>
    };
    let parsed = syn::parse2::<Entry>(input).expect("parse Record; with attrs (query & rename)");
    let parsed_type = parsed.de_type;
    let parsed_type = quote!( #parsed_type );
    let parsed_rename = parsed.rename;
    let parsed_rename = quote!( #parsed_rename );
    let parsed_query = parsed.query;
    let parsed_query = quote!( #parsed_query );
    assert_eq!(parsed.endpoint, "/some_url".to_string());
    assert_eq!(
        quote!( #parsed_type ).to_string(),
        "Map < String , Map < String , f32 > >" // TokenStream strings are spaced out
    );
    assert_eq!(
        Some(quote!( #parsed_rename ).to_string()),
        Some("\"renamed\"".to_string())
    );
    assert_eq!(
        Some(quote!( #parsed_query ).to_string()),
        Some("\"?add_this_on_the_end\"".to_string())
    );
}

#[test]
fn parse_dict_attr() {
    use kvapi_macros_internals::api::dict::Attr;

    // 1. parse `query("/append/this/string")`
    let input = quote! { query: "/append/this/string" };
    let parsed = syn::parse2::<Attr>(input).expect("parse `query(\"append/this/string\")`");

    let expected = Attr {
        fn_id: format_ident!("query"),
        arg: syn::parse_quote!("/append/this/string"),
    };
    assert_eq!(parsed.fn_id, expected.fn_id);

    let (exp_arg, par_arg) = (expected.arg, parsed.arg);
    assert_eq!(quote!(#exp_arg).to_string(), quote!(#par_arg).to_string(),);

    // 2. parse `rename("rename_to_this")`
    let input = quote! { rename="rename_to_this" };
    let parsed = syn::parse2::<Attr>(input).expect("parse `rename(\"rename_to_this\")`");

    let expected = Attr {
        fn_id: format_ident!("rename"),
        arg: syn::parse_quote!("rename_to_this"),
    };
    assert_eq!(parsed.fn_id, expected.fn_id);

    let (exp_arg, par_arg) = (expected.arg, parsed.arg);
    assert_eq!(quote!(#exp_arg).to_string(), quote!(#par_arg).to_string(),);
}

// api/header.rs
// =============
//
// Headers, Header
#[test]
fn parse_headers() {
    use kvapi_macros_internals::api::headers::Headers;

    let input = quote! {
        {
            "User-Agent"="example@example_domain.com"
            "Useragent": &std::env::var("USER_AGENT"),
        }
    };
    let parsed = syn::parse2::<Headers>(input).expect("parse Headers");
    let inner = parsed.inner;
    let requoted = quote!( #( #inner  )*).to_string();
    assert_eq!(requoted, "let value = \"example@example_domain.com\" ; headers . insert (\"User-Agent\" , reqwest :: header :: HeaderValue :: from_str (value) . unwrap ()) ; let value = & std :: env :: var (\"USER_AGENT\") ; headers . insert (\"Useragent\" , reqwest :: header :: HeaderValue :: from_str (value) . unwrap ()) ;");
}

#[test]
fn parse_header() {
    use kvapi_macros_internals::api::headers::Header;

    // 1. parse `LitStr: LitStr`
    let input = quote! { "User-Agent": "example@domain.com" };
    let parsed = syn::parse2::<Header>(input).expect("parse `LitStr: LitStr`");
    let parsed_key = parsed.key;
    let parsed_val = parsed.value;
    assert_eq!(parsed_key, "User-Agent".to_string());
    assert_eq!(parsed_val, "\"example@domain.com\"".to_string());

    // 2. parse `LitStr -> Expr`
    let input = quote! { "UserAgent" -> &std::env::var("USER_AGENT") };
    let parsed = syn::parse2::<Header>(input).expect("parse `LitStr -> Expr`");
    let parsed_key = parsed.key;
    let parsed_val = parsed.value;
    assert_eq!(parsed_key, "UserAgent".to_string());
    assert_eq!(
        parsed_val,
        "& std :: env :: var (\"USER_AGENT\")".to_string()
    );
}
