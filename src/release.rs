use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub id: i64,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInfo {
    pub url: String,
    pub browser_download_url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub state: String,
    pub content_type: String,
    pub size: i64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAssetInfo {
    name: Option<String>,
    label: Option<String>,
    state: Option<String>
}
