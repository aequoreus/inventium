use isahc::{Body, HttpClient, HttpClientBuilder, Response};

pub trait Request {
    fn get(&self, url: &str) -> Response<Body> {
        isahc::get(url).unwrap()
    }
}