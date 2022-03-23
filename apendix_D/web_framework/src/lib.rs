pub trait ServiceFactory<T> {
    fn register(self, app: &mut App<T>);
}

pub struct App<T> {
    services: Vec<T>,
    routes: Vec<Route>,
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

pub struct Route {
    verb: String,
    uri: String,
    method: String, //Ident?...fn? (function)?...Fn? (closure)?
}
