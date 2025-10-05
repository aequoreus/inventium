use crate::param::{GetDescParam, GetFilesParam, SearchModsParam};
use crate::serialize::{GetFileResponse, GetFilesResponse, GetModFileChangelog, GetModFileDownloadUrl, GetModResponse, ModDescription, SearchModsResponse};
use inventium_core::{Request, UrlBuilder};
use isahc::http::request::Builder;
use isahc::{Body, ReadResponseExt, Response};
use isahc::http::StatusCode;
use serde::Deserialize;

pub struct Curse {
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

    pub fn search_mods(&self, param: SearchModsParam) -> Result<SearchModsResponse, String> {
        let mut response = self.request.req(self.build_req(param.build(self.build_url("v1/mods/search"))));
        self.result(response.status(), serde_json::from_str::<SearchModsResponse>(&response.text().unwrap()).unwrap())
    }

    pub fn get_mod(&self, mod_id: i32) -> Result<GetModResponse, String> {
        let mut response = self.request.req(self.build_req(self.build_url(format!("v1/mods/{}", mod_id).as_str())));
        self.result(response.status(), serde_json::from_str::<GetModResponse>(&response.text().unwrap()).unwrap())
    }

    pub fn get_desc(&self, mod_id: i32, param: GetDescParam) -> Result<ModDescription, String> {
        let mut response = self.request.req(self.build_req(param.build(self.build_url(format!("v1/mods/{}/description", mod_id).as_str()))));
        self.result(response.status(), serde_json::from_str::<ModDescription>(&response.text().unwrap()).unwrap())
    }

    pub fn get_file(&self, mod_id: i32, file_id: i32) -> Result<GetFileResponse, String> {
        let mut response = self.request.req(self.build_req(self.build_url(format!("v1/mods/{}/files/{}", mod_id, file_id).as_str())));
        self.result(response.status(), serde_json::from_str::<GetFileResponse>(&response.text().unwrap()).unwrap())
    }

    pub fn get_files(&self, mod_id: i32, param: GetFilesParam) -> Result<GetFilesResponse, String> {
        let mut response = self.request.req(self.build_req(param.build(self.build_url(format!("v1/mods/{}/files", mod_id).as_str()))));
        self.result(response.status(), serde_json::from_str::<GetFilesResponse>(&response.text().unwrap()).unwrap())
    }

    pub fn changelog(&self, mod_id: i32, file_id: i32) -> Result<GetModFileChangelog, String> {
        let mut response = self.request.req(self.build_req(self.build_url(format!("v1/mods/{}/files/{}/changelog", mod_id, file_id).as_str())));
        self.result(response.status(), serde_json::from_str::<GetModFileChangelog>(&response.text().unwrap()).unwrap())
    }

    pub fn get_file_download_url(&self, mod_id: i32, file_id: i32) -> Result<GetModFileDownloadUrl, String> {
        let mut response = self.request.req(self.build_req(self.build_url(format!("v1/mods/{}/files/{}/changelog", mod_id, file_id).as_str())));
        self.result(response.status(), serde_json::from_str::<GetModFileDownloadUrl>(&response.text().unwrap()).unwrap())
    }

    fn result<T>(&self, status_code: StatusCode, result: T) -> Result<T, String>
    where
        T: Deserialize<'static>,
    {
        if status_code.is_success() {
            Ok(result)
        } else {
            Err("Invalid Request. Status Code: ".to_string() + &status_code.as_u16().to_string())
        }
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