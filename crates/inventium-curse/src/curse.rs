use crate::param::{GetDescParam, GetFilesParam, SearchModsParam};
use crate::serialize::{
    GetFileResponse, GetFilesResponse, GetModFileChangelog, GetModFileDownloadUrl, GetModResponse,
    ModDescription, SearchModsResponse,
};
use anyhow::Result;
use inventium_core::{Request, UrlBuilder};
use isahc::http::request::Builder;
use isahc::ReadResponseExt;

pub struct Curse {
    base_url: String,
    key: String,
    request: Request,
}

impl Curse {
    pub fn new(base_url: &str, key: &str, agent: &str) -> Result<Self> {
        Ok(Self {
            base_url: base_url.to_string(),
            key: key.to_string(),
            request: Request::new(agent)?,
        })
    }

    pub fn default(key: &str, agent: &str) -> Result<Self> {
        Self::new("https://api.curseforge.com", key, agent)
    }

    pub fn search_mods(&self, param: SearchModsParam) -> Result<SearchModsResponse> {
        let mut response = self
            .request
            .req(self.build_req(param.build(self.build_url("v1/mods/search"))))?;
        Ok(serde_json::from_str::<SearchModsResponse>(
            &response.text()?,
        )?)
    }

    pub fn get_mod(&self, mod_id: i32) -> Result<GetModResponse> {
        let mut response = self
            .request
            .req(self.build_req(self.build_url(format!("v1/mods/{}", mod_id).as_str())))?;
        Ok(serde_json::from_str::<GetModResponse>(&response.text()?)?)
    }

    pub fn get_desc(&self, mod_id: i32, param: GetDescParam) -> Result<ModDescription> {
        let mut response = self.request.req(self.build_req(
            param.build(self.build_url(format!("v1/mods/{}/description", mod_id).as_str())),
        ))?;
        Ok(serde_json::from_str::<ModDescription>(&response.text()?)?)
    }

    pub fn get_file(&self, mod_id: i32, file_id: i32) -> Result<GetFileResponse> {
        let mut response =
            self.request.req(self.build_req(
                self.build_url(format!("v1/mods/{}/files/{}", mod_id, file_id).as_str()),
            ))?;
        Ok(serde_json::from_str::<GetFileResponse>(&response.text()?)?)
    }

    pub fn get_files(&self, mod_id: i32, param: GetFilesParam) -> Result<GetFilesResponse> {
        let mut response = self.request.req(self.build_req(
            param.build(self.build_url(format!("v1/mods/{}/files", mod_id).as_str())),
        ))?;
        Ok(serde_json::from_str::<GetFilesResponse>(&response.text()?)?)
    }

    pub fn changelog(&self, mod_id: i32, file_id: i32) -> Result<GetModFileChangelog> {
        let mut response = self.request.req(self.build_req(
            self.build_url(format!("v1/mods/{}/files/{}/changelog", mod_id, file_id).as_str()),
        ))?;
        Ok(serde_json::from_str::<GetModFileChangelog>(
            &response.text()?,
        )?)
    }

    pub fn get_file_download_url(
        &self,
        mod_id: i32,
        file_id: i32,
    ) -> Result<GetModFileDownloadUrl> {
        let mut response = self.request.req(self.build_req(
            self.build_url(format!("v1/mods/{}/files/{}/changelog", mod_id, file_id).as_str()),
        ))?;
        Ok(serde_json::from_str::<GetModFileDownloadUrl>(
            &response.text()?,
        )?)
    }

    fn build_req(&self, builder: UrlBuilder) -> Builder {
        let mut req = self.request.get(builder.as_str());
        req = req.header("x-api-key", self.get_key());
        req
    }

    fn build_url(&self, endpoint: &str) -> UrlBuilder {
        UrlBuilder::new(format!("{}/{}", self.get_base_url(), endpoint).as_str())
    }

    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }
}
