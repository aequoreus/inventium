use serde_derive::{Deserialize, Serialize};
use strum::EnumString;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResults {
    pub hits: Vec<ProjectSearchResult>,
    pub offset: i64,
    pub limit: i64,
    pub total_hits: i64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSearchResult {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: SideStatus,
    pub server_side: SideStatus,
    pub project_type: ProjectType,
    pub downloads: i64,
    pub icon_url: Option<String>,
    pub color: Option<i64>,
    pub thread_id: String,
    pub project_id: String,
    pub author: String,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub follows: i64,
    #[serde(with = "time::serde::iso8601")]
    pub date_created: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub date_modified: OffsetDateTime,
    pub last_version: String,
    pub license: String,
    pub gallery: Vec<String>,
    pub featured_gallery: Option<String>
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mod {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: SideStatus,
    pub server_side: SideStatus,
    pub body: String,
    pub status: ProjectStatus,
    pub additional_categories: Vec<String>,
    pub issue_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub project_type: ProjectType,
    pub downloads: i64,
    pub icon_url: Option<String>,
    pub color: Option<i64>,
    pub thread_id: String,
    pub id: String,
    pub team: String,
    pub body_url: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub published: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated: OffsetDateTime,
    pub followers: i64,
    pub license: License,
    pub versions: Vec<String>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<Loader>,
    pub gallery: Vec<Gallery>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    pub dependencies: Vec<Depend>,
    pub game_versions: Vec<String>,
    pub version_type: VersionType,
    pub loaders: Vec<Loader>,
    pub featured: bool,
    pub status: ProjectStatus,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    #[serde(with = "time::serde::iso8601")]
    pub date_published: OffsetDateTime,
    pub downloads: i64,
    pub changelog_url: Option<String>,
    pub files: Vec<File>
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Depend {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: DependencyType,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct File {
    pub hashes: FileHash,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    pub file_type: Option<String>
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct FileHash {
    pub sha512: String,
    pub sha1: String
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum SideStatus {
    Required,
    Optional,
    Unsupported,
    Unknown
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum ProjectType {
    Mod,
    Modpack,
    Resourcepack,
    Shader
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ProjectStatus {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Withheld,
    Scheduled,
    Private,
    Unknown
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, EnumString)]
pub enum Loader {
    Fabric,
    Forge,
    Neoforge,
    Quilt,
    Babric,
    #[strum(serialize = "bta-babric")]
    BTABabric,
    #[strum(serialize = "java-agent")]
    JavaAgent,
    #[strum(serialize = "legacy-fabric")]
    LegacyFabric,
    Liteloader,
    Modloader,
    Nilloader,
    Ornithe,
    Rift
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Gallery {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    pub ordering: i64
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, EnumString)]
#[serde(rename_all = "camelCase")]
pub enum VersionType {
    Release,
    Beta,
    Alpha
}