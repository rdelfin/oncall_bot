use reqwest::{header::AUTHORIZATION, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error making request")]
    RequestError(#[from] reqwest::Error),
    #[error("got HTTP {0}")]
    HttpErrorCode(StatusCode),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Oncall {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schedule {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    username: String,
    #[serde(rename = "fullName")]
    full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScheduleListResponse {
    data: Vec<Schedule>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetScheduleResponse {
    data: Schedule,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListUsersResponse {
    data: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetUserResponse {
    data: User,
}

pub async fn list_oncalls() -> Vec<Oncall> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let schedules_response = client
        .get("https://api.opsgenie.com/v2/schedules")
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await
        .unwrap();

    match schedules_response.status() {
        reqwest::StatusCode::OK => match schedules_response.json::<ScheduleListResponse>().await {
            Ok(parsed) => parsed
                .data
                .into_iter()
                .map(|schedule| Oncall {
                    id: schedule.id,
                    name: schedule.name,
                })
                .collect(),
            Err(_) => vec![],
        },
        _ => vec![],
    }
}

pub async fn get_oncall_name(id: &str) -> Result<String, Error> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let schedule_response = client
        .get(format!("https://api.opsgenie.com/v2/schedules/{}", id))
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await
        .unwrap();

    match schedule_response.status() {
        reqwest::StatusCode::OK => Ok(schedule_response
            .json::<GetScheduleResponse>()
            .await?
            .data
            .name),
        code => Err(Error::HttpErrorCode(code)),
    }
}

pub async fn list_users() -> Result<Vec<User>, Error> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let users_response = client
        .get("https://api.opsgenie.com/v2/users")
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await
        .unwrap();

    match users_response.status() {
        reqwest::StatusCode::OK => Ok(users_response.json::<ListUsersResponse>().await?.data),
        code => Err(Error::HttpErrorCode(code)),
    }
}

pub async fn get_user(id: &str) -> Result<User, Error> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let user_response = client
        .get(format!("https://api.opsgenie.com/v2/users/{}", id))
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await
        .unwrap();

    match user_response.status() {
        reqwest::StatusCode::OK => Ok(user_response.json::<GetUserResponse>().await?.data),
        code => Err(Error::HttpErrorCode(code)),
    }
}

fn opsgenie_key() -> String {
    env::var("OPSGENIE_API_KEY").expect("OPSGENIE_API_KEY must be set")
}
