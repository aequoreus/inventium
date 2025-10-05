use anyhow::Result;
use isahc::http::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};
use isahc::http::request::Builder;
use isahc::{Body, HttpClient, Request as IsaRequest, Response};

pub struct UrlBuilder {
    pub url: String,
}

impl UrlBuilder {
    pub fn new(base_url: &str) -> Self {
        Self {
            url: base_url.to_string() + "?",
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
    pub client: HttpClient,
}

impl Request {
    pub fn new(agent: &str) -> Result<Self> {
        let mut builder = isahc::HttpClient::builder();

        builder = builder.default_header(CONTENT_TYPE, "application/json");
        builder = builder.default_header(ACCEPT, "application/json");
        builder = builder.default_header(USER_AGENT, agent);

        Ok(Self {
            client: builder.build()?,
        })
    }

    pub fn get(&self, url: &str) -> Builder {
        IsaRequest::get(url)
    }

    pub fn req(&self, req: Builder) -> Result<Response<Body>> {
        Ok(self.client.send(req.body(Body::empty())?)?)
    }
}
