use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReleaseInfo {
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
    pub uploader: Uploader,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Uploader {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
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
    pub r#type: String,
    pub site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAssetInfo {
    name: Option<String>,
    label: Option<String>,
    state: Option<String>
}
