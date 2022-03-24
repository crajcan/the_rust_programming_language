use web_framework::{App};

mod controllers;

use controllers::*;

/// The way this is going to work is we're going to define "route" to be an attribute
/// macro that expands to
///     A. a struct definition for #name, where #name is the ident found in the DeriveInput
///        struct we get from calling syn::parse are the input TokenStream
///     B. an implementation of a trait for #name that has a function "register".
///
/// When we pass "index" to app.service(), service will call "register" on #name.
/// "register" will then add (#name, index, fn) to a app.paths vector.
fn main() {
    let app = App::new();

    app.service(index);

    println!("app: {:?}", app);
}
