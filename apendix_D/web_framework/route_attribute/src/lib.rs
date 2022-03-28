extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::parse_macro_input;
use syn::ItemFn;

/// The first argument attr, is for the contents of the
/// attribute: (GET, "/")
/// The second is the body of the item the attribute is attached to,
/// in this case, the function "index"
#[proc_macro_attribute]
pub fn route(attrs: TokenStream, item: TokenStream) -> TokenStream {
    println!("attrs.to_string(): {}", attrs.to_string());

    let attrs = parse_macro_input!(attrs as syn::AttributeArgs);
    let verb = attrs.get(0).unwrap();
    let uri = attrs.get(1).unwrap();

    let ast: ItemFn = syn::parse(item).unwrap();
    let method_name = ast.sig.ident;
    let method = method_name.to_string();
    println!("method: {}", method);

    // pub struct

    // impl ServiceFactory {
    //     fn register(&self, routes: &mut Vec<HttpRoute>);
    // }

    quote! {
        pub fn #method_name() -> HttpRoute {
           HttpRoute { verb: #verb,  uri: (#uri).to_string(), method: (#method).to_string() }
        }
    }
    .into()
}
