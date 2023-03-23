use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateReleaseInfo {
    pub tag_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_category_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_release_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make_latest: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TagInfo {
    pub tag: String,
    pub message: String,
    pub object: String,
    #[serde(rename = "type")]
    pub type_tagged: String,
}

#[derive(Serialize, Deserialize)]
pub struct Release {
    pub node_id: String,
    pub sha: Option<String>,
    pub url: String,
    pub tag: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub node_id: String,
    pub tag: String,
    pub sha: String,
    pub url: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseInfo {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub discussion_url: Option<String>,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: Option<String>,
    pub author: Author,
    pub assets: Vec<AssetInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetInfo {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub uploader: Uploader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uploader {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAssetInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
