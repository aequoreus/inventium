use crate::param::SearchModsParam;
use crate::serialize::SearchModsResponse;
use inventium_core::{Request, UrlBuilder};
use isahc::http::request::Builder;
use isahc::ReadResponseExt;

struct Curse {
    base_url: String,
    key: String,
    request: Request
}

impl Curse {
    pub fn new(base_url: &str, key: &str, agent: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            key: key.to_string(),
            request: Request::new(agent)
        }
    }

    pub fn default(key: &str, agent: &str) -> Self {
        Self::new("https://api.curseforge.com", key, agent)
    }

    pub fn search_mods(&self, param: SearchModsParam) -> SearchModsResponse {
        let response = self.request.req(self.build_req(param.build(self.build_url("v1/mods/search")))).text().unwrap();
        serde_json::from_str(&response).unwrap()
    }

    fn build_req(&self, builder: UrlBuilder) -> Builder {
        let mut req = self.request.get(builder.as_str());
        req = req.header("x-api-key", self.get_key());
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