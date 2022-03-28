use web_framework::{route, HttpRoute, HttpVerb::GET, ServiceFactory};

#[route(GET, "/")]
fn index() {
    println!("index")
}

#[route(GET, "/show")]
fn show() {
    println!("show")
}
