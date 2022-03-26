pub use http_route::{HttpRoute, HttpVerb};
pub use route_attribute::route;

pub trait ServiceFactory<T> {
    fn register(self, app: &mut App<T>);
}

#[derive(Debug)]
pub struct App<T> {
    services: Vec<T>,
    routes: Vec<HttpRoute>,
}

impl<F> App<F> {
    pub fn new() -> Self {
        App {
            services: vec![],
            routes: vec![],
        }
    }

    pub fn service(&mut self, factory: F) -> &mut Self {
        self.services.push(factory);

        self
    }
}