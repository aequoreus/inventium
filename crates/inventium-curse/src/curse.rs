use inventium_core::Request;
use crate::serialize::SearchModsResponse;

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

    pub fn search_mods() {

    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }
}