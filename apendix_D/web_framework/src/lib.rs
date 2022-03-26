pub use http_route::{HttpRoute, HttpVerb};
pub use route_attribute::route;

/// This was going to be used if we were going to turn every route into a type
/// then implement register so each route type could push a path object onto App.routes
// pub trait ServiceFactory<T> {
//     fn register(self, app: &mut App);
// }
pub struct App<T>
where
    T: FnOnce() -> HttpRoute,
{
    services: Vec<T>,
    routes: Vec<HttpRoute>,
}

impl<T> App<T>
where
    T: FnOnce() -> HttpRoute,
{
    pub fn new() -> Self {
        App {
            services: vec![],
            routes: vec![],
        }
    }

    pub fn service(&mut self, factory: T) -> &mut Self {
        self.services.push(factory);

        self
    }

    pub fn bind(&mut self) {
        while let Some(service) = self.services.pop() {
            let route = service();
            self.routes.push(route);
        }
    }

    pub fn routes(&self) -> &Vec<HttpRoute> {
        &self.routes
    }
}
