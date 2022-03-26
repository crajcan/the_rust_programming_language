
#[derive(Debug)]
pub struct HttpRoute {
    pub verb: HttpVerb,
    pub uri: String,
    pub method: String,
}

#[derive(Debug)]
pub enum HttpVerb {
    GET,
    POST,
    PUT,
}