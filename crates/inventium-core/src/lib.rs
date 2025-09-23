use isahc::{Body, HttpClient, HttpClientBuilder, Response, Request as IsaRequest};
use isahc::http::request::Builder;

pub struct UrlBuilder {
    pub url: String
}

impl UrlBuilder {
    pub fn new(base_url: &str) -> Self {
        Self {
            url: base_url.to_string() + "?"
        }
    }

    pub fn add_param(mut self, key: &str, value: &str) -> Self {
        self.url.push_str(&format!("{}={}&", key, value));
        self
    }

    pub fn as_str(&self) -> &str {
        &self.url
    }
}

pub struct Request {
    pub client: HttpClient
}

impl Request {
    pub fn new(agent: &str) -> Self {
        let mut builder = isahc::HttpClient::builder();

        builder = builder.default_header("Content-Type", "application/json");
        builder = builder.default_header("Accept", "application/json");
        builder = builder.default_header("User-Agent", agent);

        Self {
            client: builder.build().unwrap()
        }
    }

    pub fn get(&self, url: &str) -> Builder {
        IsaRequest::get(url)
    }

    pub fn req(&self, req: Builder) -> Response<Body> {
        self.client.send(req.body(Body::empty()).unwrap()).unwrap()
    }
}