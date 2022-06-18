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

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGroup {
    pub id: String,
    pub name: String,
    pub handle: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub real_name: Option<String>,
    pub is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelTopic {
    pub value: String,
    pub creator: String,
    pub last_set: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub topic: ChannelTopic,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserGroupUpdateRequest<'a> {
    usergroup: &'a str,
    users: &'a [String],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationSetTopicRequest<'a> {
    channel: &'a str,
    topic: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostMessageRequest<'a> {
    channel: &'a str,
    text: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListUsersResponse {
    pub members: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserResponse {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGroupsListResponse {
    pub ok: bool,
    pub usergroups: Vec<UserGroup>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationListResponseMetadata {
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationsListResponse {
    pub ok: bool,
    pub channels: Vec<Channel>,
    pub response_metadata: Option<ConversationListResponseMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationInfoResponse {
    pub ok: bool,
    pub channel: Channel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConversationSetTopicResponse {
    pub ok: bool,
    pub channel: Channel,
}

pub async fn list_user_groups() -> Result<Vec<UserGroup>> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let usergroups_response = client
        .get("https://slack.com/api/usergroups.list")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await?;

    match usergroups_response.status() {
        reqwest::StatusCode::OK => Ok(usergroups_response
            .json::<UserGroupsListResponse>()
            .await?
            .usergroups),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

pub async fn get_user_group(id: &str) -> Result<UserGroup> {
    list_user_groups()
        .await?
        .into_iter()
        .find(|user_group| user_group.id == id)
        .ok_or_else(|| Error::UserGroupNotFound)
}

pub async fn set_user_group(id: &str, users: &[String]) -> Result {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let usergroups_response = client
        .post("https://slack.com/api/usergroups.users.update")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .json(&UserGroupUpdateRequest {
            usergroup: &id,
            users,
        })
        .send()
        .await?;

    match usergroups_response.status() {
        reqwest::StatusCode::OK => Ok(()),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

pub async fn list_users() -> Result<Vec<User>> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let users_response = client
        .get("https://slack.com/api/users.list")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await?;

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

pub async fn get_user(id: &str) -> Result<User> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let users_response = client
        .get(Url::parse_with_params(
            "https://slack.com/api/users.info",
            &[("user", id)],
        )?)
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await?;

    match users_response.status() {
        reqwest::StatusCode::OK => Ok(users_response.json::<GetUserResponse>().await?.user),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

pub async fn list_channels() -> Result<Vec<Channel>> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let mut channel_list = vec![];
    let mut cursor: Option<String> = None;

    loop {
        let params: Vec<_> = match cursor {
            Some(ref cursor) => vec![("cursor", &cursor[..]), ("limit", "1000")],
            None => vec![("types", "public_channel"), ("limit", "1000")],
        };
        let conversations_response = client
            .get(Url::parse_with_params(
                "https://slack.com/api/conversations.list",
                &params,
            )?)
            .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
            .send()
            .await?;

        let mut conversations = match conversations_response.status() {
            reqwest::StatusCode::OK => {
                conversations_response
                    .json::<ConversationsListResponse>()
                    .await?
            }
            error_code => return Err(Error::HttpErrorCode(error_code)),
        };

        channel_list.append(&mut conversations.channels);

        match conversations
            .response_metadata
            .map(|metadata| metadata.next_cursor)
        {
            Some(Some(next_cursor)) => {
                if next_cursor == "" {
                    break;
                }
                cursor = Some(next_cursor);
            }
            _ => {
                break;
            }
        }
    }

    Ok(channel_list)
}

pub async fn get_channel(id: &str) -> Result<Channel> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();
    let conversations_response = client
        .get(Url::parse_with_params(
            "https://slack.com/api/conversations.info",
            &[("channel", id)],
        )?)
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .send()
        .await?;

    let conversation = match conversations_response.status() {
        reqwest::StatusCode::OK => {
            conversations_response
                .json::<ConversationInfoResponse>()
                .await?
        }
        error_code => return Err(Error::HttpErrorCode(error_code)),
    };

    Ok(conversation.channel)
}

pub async fn set_channel_topic(channel_id: &str, topic: &str) -> Result<Channel> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();

    let set_topic_response = client
        .post("https://slack.com/api/conversations.setTopic")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .json(&ConversationSetTopicRequest {
            channel: channel_id,
            topic,
        })
        .send()
        .await?;

    match set_topic_response.status() {
        reqwest::StatusCode::OK => Ok(set_topic_response
            .json::<ConversationSetTopicResponse>()
            .await?
            .channel),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

pub async fn post_message(channel_id: &str, message: &str) -> Result<()> {
    let slack_oauth_token = slack_oauth_token();
    let client = reqwest::Client::new();

    let set_topic_response = client
        .post("https://slack.com/api/chat.postMessage")
        .header(AUTHORIZATION, format!("Bearer {}", slack_oauth_token))
        .json(&PostMessageRequest {
            channel: channel_id,
            text: message,
        })
        .send()
        .await?;

    match set_topic_response.status() {
        reqwest::StatusCode::OK => Ok(()),
        error_code => Err(Error::HttpErrorCode(error_code)),
    }
}

fn slack_oauth_token() -> String {
    env::var("SLACK_OAUTH_TOKEN").expect("SLACK_OAUTH_TOKEN must be set")
}
