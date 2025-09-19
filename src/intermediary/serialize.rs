#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub version: Vec<String>,
    pub author: Vec<String>,
    pub platforms: Vec<Platform>
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Platform {}