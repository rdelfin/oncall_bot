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

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Oncall {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Schedule {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(rename = "fullName")]
    pub full_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OncallParticipant {
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentOncall {
    #[serde(rename = "onCallParticipants")]
    pub on_call_participants: Vec<OncallParticipant>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScheduleListResponse {
    pub data: Vec<Schedule>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetScheduleResponse {
    pub data: Schedule,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListUsersResponse {
    pub data: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetUserResponse {
    pub data: User,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentOncallResponse {
    pub data: CurrentOncall,
}

pub async fn list_oncalls() -> Result<Vec<Oncall>> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let schedules_response = client
        .get("https://api.opsgenie.com/v2/schedules")
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await?;

    Ok(match schedules_response.status() {
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
    })
}

pub async fn get_oncall_name(id: &str) -> Result<String> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let schedule_response = client
        .get(format!("https://api.opsgenie.com/v2/schedules/{}", id))
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await?;

    match schedule_response.status() {
        reqwest::StatusCode::OK => Ok(schedule_response
            .json::<GetScheduleResponse>()
            .await?
            .data
            .name),
        code => Err(Error::HttpErrorCode(code)),
    }
}

pub async fn list_users() -> Result<Vec<User>> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let users_response = client
        .get("https://api.opsgenie.com/v2/users")
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await?;

    match users_response.status() {
        reqwest::StatusCode::OK => Ok(users_response.json::<ListUsersResponse>().await?.data),
        code => Err(Error::HttpErrorCode(code)),
    }
}

pub async fn get_user(id: &str) -> Result<User> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let user_response = client
        .get(format!("https://api.opsgenie.com/v2/users/{}", id))
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await?;

    match user_response.status() {
        reqwest::StatusCode::OK => Ok(user_response.json::<GetUserResponse>().await?.data),
        code => Err(Error::HttpErrorCode(code)),
    }
}

pub async fn get_current_oncalls(oncall_id: &str) -> Result<Vec<String>> {
    let opsgenie_key = opsgenie_key();
    let client = reqwest::Client::new();
    let oncall_response = client
        .get(format!(
            "https://api.opsgenie.com/v2/schedules/{}/on-calls",
            oncall_id
        ))
        .header(AUTHORIZATION, format!("GenieKey {}", opsgenie_key))
        .send()
        .await?;

    match oncall_response.status() {
        reqwest::StatusCode::OK => Ok(oncall_response
            .json::<CurrentOncallResponse>()
            .await?
            .data
            .on_call_participants
            .into_iter()
            .filter_map(|participant| {
                if participant.typ == "user" {
                    Some(participant.id)
                } else {
                    None
                }
            })
            .collect()),
        code => Err(Error::HttpErrorCode(code)),
    }
}

fn opsgenie_key() -> String {
    env::var("OPSGENIE_API_KEY").expect("OPSGENIE_API_KEY must be set")
}
