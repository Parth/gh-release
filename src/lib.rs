use std::fs::File;
use crate::error::Error;
use reqwest::blocking::{Client, RequestBuilder, Response};
use crate::release::{ReleaseInfo, UpdateAssetInfo};

mod error;
mod release;

const GITHUB_URL: &str = "https://github.com";

fn release_url(repo_info: &RepoInfo) -> String {
    format!("{}/repos/{}/{}/releases", GITHUB_URL, repo_info.owner, repo_info.repo_name)
}

fn release_assets(repo_info: &RepoInfo, release_id: &str) -> String {
    format!("{}/{}/assets", release_id, release_url(repo_info))
}

fn asset_url(repo_info: &RepoInfo, asset_id: u64) -> String {
    format!("{}/assets/{}", release_url(repo_info), asset_id)
}

fn tag_url(repo_info: &RepoInfo, tag: &str) -> String {
    format!("{}/tags/{}", release_url(repo_info), tag)
}

fn latest_url(repo_info: &RepoInfo) -> String {
    format!("{}/latest", release_url(repo_info))
}

trait GithubAuthRequest {
    fn send_and_successful(self, auth_token: &str) -> Result<Response, Error>;
}

impl GithubAuthRequest for RequestBuilder {
    fn send_and_successful(self, auth_token: &str) -> Result<Response, Error> {
        let response = self
            .header("ACCEPT", "application/vnd.github+json")
            .header("Authorization",  auth_token)
            .send()?;

        match response.status().as_u16() {
            200 => Ok(response),
            201 => Ok(response),
            302 => Ok(response),
            _ => Err(Error::Unexpected(format!("Unknown status code: {}", response.status())))
        }
    }
}

pub struct AuthenticatedUser {
    auth_token: String,
    client: Client,
}

pub struct RepoInfo {
    owner: String,
    repo_name: String,
}

impl AuthenticatedUser {
    pub fn new(maybe_auth_token: Option<String>) -> Option<AuthenticatedUser> {
        Some(AuthenticatedUser {
            auth_token: format!("token {}", maybe_auth_token
                .and(option_env!("GITHUB_TOKEN"))?),
            client: Client::new(),
        })
    }

    pub fn get_release_asset(
        &self,
        repo_info: &RepoInfo,
        asset_id: u64,
    ) -> Result<ReleaseInfo, Error> {
        Ok(self
            .client
            .get(asset_url(&repo_info, asset_id))
            .send_and_successful(&self.auth_token)?
            .json()?)
    }

    pub fn update_release_asset(
        &self,
        repo_info: &RepoInfo,
        asset_id: u64,
        maybe_asset_info: Option<UpdateAssetInfo>
    ) -> Result<ReleaseInfo, Error> {
        Ok(self
            .client
            .patch(asset_url(&repo_info, asset_id))
            .body(serde_json::to_string(&maybe_asset_info)?)
            .send_and_successful(&self.auth_token)?
            .json()?)
    }

    pub fn delete_release_asset(&self, repo_info: &RepoInfo, asset_id: u64) -> Result<(), Error> {
        self
            .client
            .delete(asset_url(&repo_info, asset_id))
            .send_and_successful(&self.auth_token)?;

        Ok(())
    }

    pub fn list_release_assets(
        &self,
        repo_info: &RepoInfo,
        release_id: &str,
        per_page: Option<u64>,
        page: Option<u64>,
    ) -> Result<Vec<ReleaseInfo>, Error> {
        Ok(self
            .client
            .patch(release_assets(&repo_info, release_id))
            .query(&[("per_page", per_page.unwrap_or(30)), ("page", page.unwrap_or(1))])
            .send_and_successful(&self.auth_token)?
            .json()?)
    }

    pub fn upload_release_asset(
        &self,
        repo_info: &RepoInfo,
        release_id: &str,
        asset_name: &str,
        content_type: &str,
        file: File,
        label: Option<&str>,
    ) -> Result<ReleaseInfo, Error> {
        let request = self
            .client
            .patch(release_assets(&repo_info, release_id))
            .query(&[("name", asset_name), ("label", label.unwrap_or(""))])
            .header("Content-Type", content_type)
            .body(file);

        Ok(request
            .send_and_successful(&self.auth_token)?
            .json()?)

    }

    pub fn get_release_by_tag_name(
        &self,
        repo_info: &RepoInfo,
        tag: &str,
    ) -> Result<ReleaseInfo, Error> {
        Ok(self
            .client
            .get(tag_url(&repo_info, tag))
            .send_and_successful(&self.auth_token)?
            .json()?)
    }

    pub fn get_latest_release(&self, repo_info: &RepoInfo) -> Result<ReleaseInfo, Error> {
        Ok(self
            .client
            .get(latest_url(&repo_info))
            .send_and_successful(&self.auth_token)?
            .json()?)
    }
}
