use web_framework::{route, HttpRoute, HttpVerb::GET};

#[route(GET, "/")]
fn index() {}
