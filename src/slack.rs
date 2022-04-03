use reqwest::{header::AUTHORIZATION, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making request")]
    RequestError(#[from] reqwest::Error),
    #[error("got HTTP {0}")]
    HttpErrorCode(StatusCode),
    #[error("could not find user group")]
    UserGroupNotFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    id: String,
    name: String,
    handle: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroupsListResponse {
    ok: bool,
    usergroups: Vec<UserGroup>,
}

pub async fn list_user_groups() -> Result<Vec<UserGroup>, Error> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let usergroups_response = client
        .get("https://slack.com/api/usergroups.list")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await
        .unwrap();

    match usergroups_response.status() {
        reqwest::StatusCode::OK => Ok(usergroups_response
            .json::<UserGroupsListResponse>()
            .await?
            .usergroups),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

pub async fn get_user_group(id: &str) -> Result<UserGroup, Error> {
    list_user_groups()
        .await?
        .into_iter()
        .find(|user_group| user_group.id == id)
        .ok_or_else(|| Error::UserGroupNotFound)
}

fn slack_oauth_token() -> String {
    env::var("SLACK_OAUTH_TOKEN").expect("SLACK_OAUTH_TOKEN must be set")
}
