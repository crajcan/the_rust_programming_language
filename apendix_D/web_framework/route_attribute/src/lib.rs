extern crate proc_macro;

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
    let attrs = parse_macro_input!(attrs as syn::AttributeArgs);
    let verb = attrs.get(0).unwrap();

    quote! {
        fn route() {
            Route { verb: #verb,  uri: "/", method: "index" }
        }
    }
    .into()
}
