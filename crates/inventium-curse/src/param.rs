use inventium_core::UrlBuilder;

pub struct SearchModsParam {
    pub class_id: Option<i32>,
    pub category_id: Option<Category>,
    pub category_ids: Option<Vec<Category>>,
    pub game_version: Option<String>,
    pub game_versions: Option<String>,
    pub search_filter: Option<String>,
    pub sort_field: Option<ModSearchSortField>,
    pub sort_order: Option<String>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub mod_loader_types: Option<Vec<ModLoaderType>>,
    pub game_version_type_id: Option<i32>,
    pub author_id: Option<i32>,
    pub primary_author_id: Option<i32>,
    pub slug: Option<String>,
    pub index: Option<i32>,
    pub page_size: Option<i32>
}

impl SearchModsParam {
    pub fn new() -> Self {
        Self {
            class_id: None,
            category_id: None,
            category_ids: None,
            game_version: None,
            game_versions: None,
            search_filter: None,
            sort_field: None,
            sort_order: None,
            mod_loader_type: None,
            mod_loader_types: None,
            game_version_type_id: None,
            author_id: None,
            primary_author_id: None,
            slug: None,
            index: None,
            page_size: None
        }
    }

    pub fn build(&self, builder: UrlBuilder) -> UrlBuilder {
        let mut b = builder;
        b = b.add_param("gameId", "432");
        if let Some(class_id) = self.class_id {
            b = b.add_param("classId", &class_id.to_string());
        }
        if let Some(category_id) = self.category_id {
            b = b.add_param("categoryId", &category_id.id().to_string());
        }
        if let Some(category_ids) = &self.category_ids {
            let ids: Vec<String> = category_ids.iter().map(|c| c.id().to_string()).collect();
            b = b.add_param("categoryIds", &ids.join(","));
        }
        if let Some(game_version) = &self.game_version {
            b = b.add_param("gameVersion", game_version);
        }
        if let Some(game_versions) = &self.game_versions {
            b = b.add_param("gameVersions", game_versions);
        }
        if let Some(search_filter) = &self.search_filter {
            b = b.add_param("searchFilter", search_filter);
        }
        if let Some(sort_field) = self.sort_field {
            b = b.add_param("sortField", &sort_field.id().to_string());
        }
        if let Some(sort_order) = &self.sort_order {
            b = b.add_param("sortOrder", sort_order);
        }
        if let Some(mod_loader_type) = self.mod_loader_type {
            b = b.add_param("modLoaderType", &mod_loader_type.id().to_string());
        }
        if let Some(mod_loader_types) = &self.mod_loader_types {
            let ids: Vec<String> = mod_loader_types.iter().map(|m| m.id().to_string()).collect();
            b = b.add_param("modLoaderTypes", &ids.join(","));
        }
        if let Some(game_version_type_id) = self.game_version_type_id {
            b = b.add_param("gameVersionTypeId", &game_version_type_id.to_string());
        }
        if let Some(author_id) = self.author_id {
            b = b.add_param("authorId", &author_id.to_string());
        }
        if let Some(primary_author_id) = self.primary_author_id {
            b = b.add_param("primaryAuthorId", &primary_author_id.to_string());
        }
        if let Some(slug) = &self.slug {
            b = b.add_param("slug", slug);
        }
        if let Some(index) = self.index {
            b = b.add_param("index", &index.to_string());
        }
        if let Some(page_size) = self.page_size {
            b = b.add_param("pageSize", &page_size.to_string());
        }
        b
    }
}

pub struct GetDescParam {
    pub raw: Option<bool>,
    pub stripped: Option<bool>,
    pub markup: Option<bool>
}

impl GetDescParam {
    pub fn new() -> Self {
        Self {
            raw: None,
            stripped: None,
            markup: None
        }
    }

    pub fn build(&self, builder: UrlBuilder) -> UrlBuilder {
        let mut b = builder;
        if let Some(raw) = self.raw {
            b = b.add_param("raw", &raw.to_string());
        }
        if let Some(stripped) = self.stripped {
            b = b.add_param("stripped", &stripped.to_string());
        }
        if let Some(markup) = self.markup {
            b = b.add_param("markup", &markup.to_string());
        }
        b
    }
}

pub struct GetFilesParam {
    pub game_version: Option<String>,
    pub mod_loader_type: Option<ModLoaderType>,
    pub game_version_type_id: Option<i32>,
    pub index: Option<i32>,
    pub page_size: Option<i32>
}

impl GetFilesParam {
    pub fn new() -> Self {
        Self {
            game_version: None,
            mod_loader_type: None,
            game_version_type_id: None,
            index: None,
            page_size: None
        }
    }

    pub fn build(&self, builder: UrlBuilder) -> UrlBuilder {
        let mut b = builder;
        if let Some(game_version) = &self.game_version {
            b = b.add_param("gameVersion", game_version);
        }
        if let Some(mod_loader_type) = self.mod_loader_type {
            b = b.add_param("modLoaderType", &mod_loader_type.id().to_string());
        }
        if let Some(game_version_type_id) = self.game_version_type_id {
            b = b.add_param("gameVersionTypeId", &game_version_type_id.to_string());
        }
        if let Some(index) = self.index {
            b = b.add_param("index", &index.to_string());
        }
        if let Some(page_size) = self.page_size {
            b = b.add_param("pageSize", &page_size.to_string());
        }
        b
    }

}

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Worlds = 17,
    Mods = 6,
    Modpacks = 4471,
    ResourcePacks = 12,
    Shaders = 6552
}

impl Category {
    pub fn id(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ModSearchSortField {
    Featured = 1,
    Popularity = 2,
    LastUpdated = 3,
    Name = 4,
    Author = 5,
    TotalDownloads = 6,
    Category = 7,
    GameVersion = 8,
    EarlyAccess = 9,
    FeaturedReleased = 10,
    ReleasedDate = 11,
    Rating = 12
}

impl ModSearchSortField {
    pub fn id(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6
}

impl ModLoaderType {
    pub fn id(self) -> i32 {
        self as i32
    }
}