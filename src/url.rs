use crate::RepoInfo;

const API_URL: &str = "https://api.github.com";
const UPLOAD_URL: &str = "https://uploads.github.com";

fn release_url(repo_info: &RepoInfo) -> String {
    format!(
        "{}/repos/{}/{}/releases",
        API_URL, repo_info.owner, repo_info.repo_name
    )
}

pub fn upload_asset_url(repo_info: &RepoInfo, asset_id: u64) -> String {
    format!(
        "{}/repos/{}/{}/releases/{}/assets",
        UPLOAD_URL, repo_info.owner, repo_info.repo_name, asset_id
    )
}

pub fn release_assets(repo_info: &RepoInfo, release_id: u64) -> String {
    format!("{}/{}/assets", release_url(repo_info), release_id)
}

pub fn asset_url(repo_info: &RepoInfo, asset_id: u64) -> String {
    format!("{}/assets/{}", release_url(repo_info), asset_id)
}

pub fn tag_url(repo_info: &RepoInfo, tag: &str) -> String {
    format!("{}/tags/{}", release_url(repo_info), tag)
}

pub fn latest_url(repo_info: &RepoInfo) -> String {
    format!("{}/latest", release_url(repo_info))
}
