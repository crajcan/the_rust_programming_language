extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

/// The first argument attr, is for the contents of the
/// attribute: (GET, "/")
/// The second is the body of the item the attribute is attached to,
/// in this case, the function "index"
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse TokenStream(s)
    // let attr_ast: syn::DeriveInput = match syn::parse(attr) {
    //     Ok(ast) => ast,
    //     Err(e) => {
    //         println!("err: {:?}", e);
    //         panic!("{}", e);
    //     }
    // };
    // let item_ast: syn::DeriveInput = match syn::parse(item) {
    //     Ok(ast) => ast,
    //     Err(e) => {
    //         println!("err: {:?}", e);
    //         panic!("{}", e);
    //     }
    // };

    // let name = item_ast.ident;
    // let verb = &attr_ast.attrs[0];
    // let uri = &attr_ast.attrs[1];

    // define struct <route name>, impl ServiceFactory for <route name>
    quote! {
        pub struct Foobar<T>;//#name;

        impl<T> ServiceFactory for Foobar<T> {//#name {
            fn register(self, app: &mut App<T>) {
                //app.routes.push(Route { verb: #verb, uri: #uri, method: #name })
            }
        }
    }
    .into()
}
