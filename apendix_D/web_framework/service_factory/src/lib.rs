use http_route::HttpRoute;

pub trait ServiceFactory {
    fn register(&self, routes: &mut Vec<HttpRoute>);
}
