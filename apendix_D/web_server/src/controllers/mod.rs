use route_attribute::route;
use web_framework::{App, ServiceFactory};

#[route(GET, "/")]
fn index() {}
