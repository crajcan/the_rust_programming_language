extern crate proc_macro;

use http_route::HttpRoute;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;
use syn::DeriveInput;

/// The first argument attr, is for the contents of the
/// attribute: (GET, "/")
/// The second is the body of the item the attribute is attached to,
/// in this case, the function "index"
#[proc_macro_attribute]
pub fn route(attrs: TokenStream, item: TokenStream) -> TokenStream {
    println!("**** Parsing TokenStreams in route attribute ****");
    println!("attrs.to_string(): {}", attrs.to_string());

    let attrs = parse_macro_input!(attrs as syn::AttributeArgs);
    let verb = attrs.get(0).unwrap();
    let uri = attrs.get(1).unwrap();

    quote! {
        pub fn index() -> HttpRoute {
            HttpRoute { verb: #verb,  uri: (#uri).to_string(), method: "index".to_string() }
        }
    }
    .into()
}
