use crate::error::Error;
use crate::release::{AssetInfo, ReleaseInfo, UpdateAssetInfo};
use reqwest::blocking::{Client, RequestBuilder, Response};
use std::fs::File;

mod error;
pub mod release;
mod url;

pub struct ReleaseClient {
    auth_token: String,
    client: Client,
}

pub struct RepoInfo<'a> {
    pub owner: &'a str,
    pub repo_name: &'a str,
}

impl ReleaseClient {
    pub fn new(auth_token: String) -> Result<ReleaseClient, Error> {
        let valid_auth_token = format!("token {}", auth_token);
        let client = Client::builder().timeout(None).use_rustls_tls().build()?;

        Ok(ReleaseClient {
            auth_token: valid_auth_token,
            client,
        })
    }

    pub fn get_release_asset(
        &self,
        repo_info: &RepoInfo,
        asset_id: u64,
    ) -> Result<AssetInfo, Error> {
        Ok(self
            .client
            .get(url::asset_url(repo_info, asset_id))
            .do_it(&self.auth_token)?
            .json()?)
    }

    pub fn update_release_asset(
        &self,
        repo_info: &RepoInfo,
        asset_id: u64,
        asset_info: Option<UpdateAssetInfo>,
    ) -> Result<AssetInfo, Error> {
        let mut request = self.client.patch(url::asset_url(repo_info, asset_id));

        if let Some(asset_info) = asset_info {
            request = request.body(serde_json::to_string(&asset_info)?);
        }

        Ok(request.do_it(&self.auth_token)?.json()?)
    }

    pub fn delete_release_asset(&self, repo_info: &RepoInfo, asset_id: u64) -> Result<(), Error> {
        self.client
            .delete(url::asset_url(repo_info, asset_id))
            .do_it(&self.auth_token)?;

        Ok(())
    }

    pub fn list_release_assets(
        &self,
        repo_info: &RepoInfo,
        release_id: u64,
        per_page: Option<u64>,
        page: Option<u64>,
    ) -> Result<Vec<AssetInfo>, Error> {
        Ok(self
            .client
            .get(url::release_assets(repo_info, release_id))
            .query(&[
                ("per_page", per_page.unwrap_or(30)),
                ("page", page.unwrap_or(1)),
            ])
            .do_it(&self.auth_token)?
            .json()?)
    }

    pub fn upload_release_asset(
        &self,
        repo_info: &RepoInfo,
        release_id: u64,
        asset_name: &str,
        content_type: &str,
        file: File,
        label: Option<&str>,
    ) -> Result<AssetInfo, Error> {
        let request = self
            .client
            .post(url::upload_asset_url(repo_info, release_id))
            .query(&[("name", asset_name), ("label", label.unwrap_or(""))])
            .header("Content-Type", content_type)
            .body(file);

        Ok(request.do_it(&self.auth_token)?.json()?)
    }

    pub fn get_release_by_tag_name(
        &self,
        repo_info: &RepoInfo,
        tag: &str,
    ) -> Result<ReleaseInfo, Error> {
        Ok(self
            .client
            .get(url::tag_url(repo_info, tag))
            .do_it(&self.auth_token)?
            .json()?)
    }

    pub fn get_latest_release(&self, repo_info: &RepoInfo) -> Result<ReleaseInfo, Error> {
        Ok(self
            .client
            .get(url::latest_url(repo_info))
            .do_it(&self.auth_token)?
            .json()?)
    }
}

trait GithubAuthRequest {
    fn do_it(self, auth_token: &str) -> Result<Response, Error>;
}

impl GithubAuthRequest for RequestBuilder {
    fn do_it(self, auth_token: &str) -> Result<Response, Error> {
        let response = self
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "Github-Release-rs")
            .header("Authorization", auth_token)
            .send()?;
        let status = response.status().as_u16();

        if (200..299).contains(&status) || status == 302 {
            Ok(response)
        } else {
            Err(Error::HttpError(
                status,
                Some(String::from_utf8_lossy(response.bytes()?.as_ref()).to_string()),
            ))
        }
    }
}
