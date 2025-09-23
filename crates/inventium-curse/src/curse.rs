struct Curse {
    base_url: String,
    key: String
}

impl Curse {
    pub fn new(base_url: &str, key: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            key: key.to_string()
        }
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }
}