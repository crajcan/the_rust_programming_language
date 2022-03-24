extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();

    if input_string.contains("FROM") {
        return input;
    } else {
        panic!("****************** NO FROM CLAUSE FOUND!! ******************");
    }
}
