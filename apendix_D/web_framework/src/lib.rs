pub use http_route::HttpRoute;
pub use route_attribute::route;

pub trait ServiceFactory<T> {
    fn register(self, app: &mut App<T>);
}

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

    fn service(&mut self, factory: F) -> &mut App<F> {
        self.services.push(factory);

        self
    }
}
