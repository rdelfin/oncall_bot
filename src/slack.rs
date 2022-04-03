use reqwest::{header::AUTHORIZATION, StatusCode, Url};
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
    #[error("could not parse url")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserGroup {
    id: String,
    name: String,
    handle: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    name: String,
    real_name: Option<String>,
    is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListUsersResponse {
    members: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserResponse {
    user: User,
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

pub async fn list_users() -> Result<Vec<User>, Error> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let users_response = client
        .get("https://slack.com/api/users.list")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await
        .unwrap();

    match users_response.status() {
        reqwest::StatusCode::OK => Ok(users_response
            .json::<ListUsersResponse>()
            .await?
            .members
            .into_iter()
            .filter(|u| !u.is_bot)
            .collect()),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

pub async fn get_user(id: &str) -> Result<User, Error> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let users_response = client
        .get(Url::parse_with_params(
            "https://slack.com/api/users.info",
            &[("user", id)],
        )?)
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await
        .unwrap();

    match users_response.status() {
        reqwest::StatusCode::OK => Ok(users_response.json::<GetUserResponse>().await?.user),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

fn slack_oauth_token() -> String {
    env::var("SLACK_OAUTH_TOKEN").expect("SLACK_OAUTH_TOKEN must be set")
}
