use inventium_core::{Request, UrlBuilder};
use isahc::http::request::Builder;
use anyhow::Result;

pub struct Rinth {
    base_url: String,
    key: String,
    request: Request
}

impl Rinth {
    pub fn new(base_url: &str, key: &str, agent: &str) -> Result<Self> {
        Ok(Self {
            base_url: base_url.to_string(),
            key: key.to_string(),
            request: Request::new(agent)?
        })
    }

    pub fn default(key: &str, agent: &str) -> Result<Rinth> {
        Self::new("https://api.modrinth.com", key, agent)
    }

    fn build_req(&self, builder: UrlBuilder) -> Builder {
        let mut req = self.request.get(builder.as_str());
        req = req.header("Authorization", self.get_key());
        req
    }

    fn build_url(&self, postfix: &str) -> UrlBuilder {
        UrlBuilder::new(format!("{}/{}", self.get_base_url(), postfix).as_str())
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }
}