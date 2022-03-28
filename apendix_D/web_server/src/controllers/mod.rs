use web_framework::{route, HttpRoute, HttpVerb::GET, ServiceFactory};

#[route(GET, "/")]
fn index() {
    println!("index")
}
