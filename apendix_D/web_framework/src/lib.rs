pub use http_route::{HttpRoute, HttpVerb};
pub use route_attribute::route;

/// This was going to be used if we were going to turn every route into a type
/// then implement register so each route type could push a path object onto App.routes
pub trait ServiceFactory {
    fn register(&self, routes: &mut Vec<HttpRoute>);
}

pub struct App {
    services: Vec<Box<dyn ServiceFactory>>,
    routes: Vec<HttpRoute>,
}

impl App {
    pub fn new() -> Self {
        App {
            services: vec![],
            routes: vec![],
        }
    }

    pub fn service<F>(&mut self, factory: F) -> &mut Self
    where
        F: ServiceFactory + 'static,
    {
        self.services.push(Box::new(factory));

        self
    }

    pub fn bind(&mut self) {
        while let Some(service) = self.services.pop() {
            service.register(&mut self.routes);
        }
    }

    pub fn routes(&self) -> &Vec<HttpRoute> {
        &self.routes
    }
}
