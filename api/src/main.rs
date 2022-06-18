#[macro_use]
extern crate diesel;

use crate::{cache::Cache, notifier::SlackNotifier, user_group_sync::UserGroupSyncer};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv;
use futures_util::future::join_all;
use log::{warn, Level};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, sync::Arc, time::Duration};
use tokio::{join, sync::Mutex};

mod cache;
mod db;
mod models;
mod notifier;
mod opsgenie;
mod schema;
mod slack;
mod user_group_sync;

#[derive(Serialize, Deserialize, Debug)]
struct OncallSync {
    id: i32,
    oncall_id: String,
    oncall_name: String,
    user_group_id: String,
    user_group_name: String,
    user_group_handle: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserMapping {
    id: i32,
    opsgenie_user_id: String,
    slack_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Notification {
    pub id: i32,
    pub oncall_id: String,
    pub oncall_name: String,
    pub slack_channel_id: String,
    pub slack_channel_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddSyncRequest {
    oncall_id: String,
    user_group_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveSyncRequest {
    oncall_sync_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithRequest {
    oncall_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetSlackUserMappingRequest {
    slack_user_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithResponse {
    syncs: Vec<OncallSync>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListSyncsResponse {
    syncs: Vec<OncallSync>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListOncallsResponse {
    oncalls: Vec<opsgenie::Oncall>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListUserGroupsResponse {
    user_groups: Vec<slack::UserGroup>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListSlackUsersResponse {
    users: Vec<slack::User>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListSlackChannelsResponse {
    channels: Vec<slack::Channel>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListOpsgenieUsersResponse {
    users: Vec<opsgenie::User>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddUserMapRequest {
    slack_id: String,
    opsgenie_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveUserMapRequest {
    user_mapping_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetNotificationForSlackChannelRequest {
    slack_channel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetNotificationForOncallRequest {
    oncall_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddNotificationRequest {
    oncall_id: String,
    slack_channel_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveNotificationRequest {
    notification_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListUserMappingsResponse {
    user_mappings: Vec<UserMapping>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetSlackUserMappingResponse {
    user_mapping: Option<UserMapping>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListNotificationsResponse {
    notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetNotificationForSlackChannelResponse {
    notification: Option<Notification>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetNotificationForOncallResponse {
    notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddNotificationResponse {
    notification: Notification,
}

#[derive(Serialize, Deserialize, Debug)]
struct RemoveNotificationResponse {
    notification: Notification,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct UserGroupSyncerKey {
    oncall_id: String,
    user_group_id: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct SlackNotifierKey {
    oncall_id: String,
    slack_channel_id: String,
}

//
// Cache Functions
//

async fn slack_users_update() -> slack::Result<HashMap<String, slack::User>> {
    Ok(slack::list_users()
        .await?
        .into_iter()
        .map(|user| (user.id.clone(), user))
        .collect())
}

async fn oncall_update() -> opsgenie::Result<HashMap<String, opsgenie::Oncall>> {
    Ok(opsgenie::list_oncalls()
        .await?
        .into_iter()
        .map(|oncall| (oncall.id.clone(), oncall))
        .collect())
}

async fn slack_channel_update() -> slack::Result<HashMap<String, slack::Channel>> {
    Ok(slack::list_channels()
        .await?
        .into_iter()
        .map(|channel| (channel.id.clone(), channel))
        .collect())
}

//
// Helper functions
//

async fn db_notification_to_response(
    notification: models::NotifiedSlackChannel,
    data: &Arc<AppState>,
) -> anyhow::Result<Notification> {
    let slack_channel = data
        .slack_channel_cache
        .get(&notification.slack_channel_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("slack channel not found"))?;
    let oncall = data
        .oncall_cache
        .get(&notification.oncall_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("oncall not found"))?;
    Ok(Notification {
        id: notification.id,
        oncall_id: notification.oncall_id,
        oncall_name: oncall.name.clone(),
        slack_channel_id: notification.slack_channel_id,
        slack_channel_name: slack_channel.name.clone(),
    })
}

//
// App State
//

struct AppState {
    // Map of oncall ID to syncers
    syncers: Mutex<HashMap<UserGroupSyncerKey, UserGroupSyncer>>,
    notifiers: Mutex<HashMap<SlackNotifierKey, SlackNotifier>>,
    slack_user_cache: Cache<String, slack::User, slack::Error>, // Key is the user ID
    oncall_cache: Cache<String, opsgenie::Oncall, opsgenie::Error>, // Key is the oncall ID
    slack_channel_cache: Cache<String, slack::Channel, slack::Error>, // Key is the slack channel ID
}

impl AppState {
    async fn new() -> anyhow::Result<AppState> {
        let (syncs, notifiers) = join!(
            web::block(move || {
                let conn = db::connection();
                db::list_oncall_syncs(&conn)
            }),
            web::block(move || {
                let conn = db::connection();
                db::list_notified_slack_channels(&conn)
            })
        );
        let syncs = syncs??;
        let notifiers = notifiers??;

        let syncers = syncs
            .into_iter()
            .map(|s| {
                (
                    UserGroupSyncerKey {
                        oncall_id: s.oncall_id.clone(),
                        user_group_id: s.user_group_id.clone(),
                    },
                    UserGroupSyncer::new(s.oncall_id, s.user_group_id),
                )
            })
            .collect();
        let notifiers = notifiers
            .into_iter()
            .map(|n| {
                (
                    SlackNotifierKey {
                        oncall_id: n.oncall_id.clone(),
                        slack_channel_id: n.slack_channel_id.clone(),
                    },
                    SlackNotifier::new(n.oncall_id, n.slack_channel_id),
                )
            })
            .collect();

        Ok(AppState {
            syncers: Mutex::new(syncers),
            notifiers: Mutex::new(notifiers),
            slack_user_cache: Cache::new(Duration::from_secs(60), slack_users_update),
            oncall_cache: Cache::new(Duration::from_secs(60), oncall_update),
            slack_channel_cache: Cache::new(Duration::from_secs(60), slack_channel_update),
        })
    }
}

//
// Endpoints
//

#[get("/list_slack_users")]
async fn list_slack_users(data: web::Data<Arc<AppState>>) -> Result<impl Responder> {
    let users = match data.slack_user_cache.get_all().await {
        Ok(users) => users,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(ListSlackUsersResponse {
        users: users.into_values().collect(),
    }))
}

#[get("/list_opsgenie_users")]
async fn list_opsgenie_users() -> Result<impl Responder> {
    let users = match opsgenie::list_users().await {
        Ok(users) => users,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(ListOpsgenieUsersResponse { users }))
}

#[get("/list_user_groups")]
async fn list_user_groups() -> Result<impl Responder> {
    let user_groups = match slack::list_user_groups().await {
        Ok(user_groups) => user_groups,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(ListUserGroupsResponse { user_groups }))
}

#[get("/list_oncalls")]
async fn list_oncalls(data: web::Data<Arc<AppState>>) -> Result<impl Responder> {
    let oncalls = match data.oncall_cache.get_all().await {
        Ok(oncalls) => oncalls,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(ListOncallsResponse {
        oncalls: oncalls.into_values().collect(),
    }))
}

#[get("/list_slack_channels")]
async fn list_slack_channels(data: web::Data<Arc<AppState>>) -> Result<impl Responder> {
    let channels = match data.slack_channel_cache.get_all().await {
        Ok(channels) => channels,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(ListSlackChannelsResponse {
        channels: channels.into_values().collect(),
    }))
}

#[post("/add_user_map")]
async fn add_user_map(req: web::Json<AddUserMapRequest>) -> Result<impl Responder> {
    // Confirm users exist
    let (slack_user, opsgenie_user) = join!(
        slack::get_user(&req.slack_id),
        opsgenie::get_user(&req.opsgenie_id)
    );
    if let Err(e) = slack_user {
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("{:?}", e),
        }));
    }
    if let Err(e) = opsgenie_user {
        return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
            error: format!("{:?}", e),
        }));
    }

    let conn = db::connection();
    let add_res = match web::block(move || {
        db::add_user_mapping(&conn, &req.opsgenie_id, &req.slack_id)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };
    Ok(HttpResponse::Ok().json(add_res))
}

#[post("/remove_user_map")]
async fn remove_user_map(req: web::Json<RemoveUserMapRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let user_mapping_id = req.user_mapping_id;

    // Remove user mapping from DB first.
    let removed_user_mapping =
        match web::block(move || db::remove_user_mapping(&conn, user_mapping_id)).await {
            Err(blocking_error) => {
                return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                    error: format!("{:?}", blocking_error),
                }));
            }
            Ok(Err(db_error)) => {
                return Ok(db_error.into());
            }
            Ok(Ok(res)) => res,
        };

    Ok(HttpResponse::Ok().json(removed_user_mapping))
}

#[post("/add_sync")]
async fn add_sync(
    req: web::Json<AddSyncRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder> {
    let conn = db::connection();
    // Verify oncall existence
    if let Err(opsgenie::Error::HttpErrorCode(code)) =
        opsgenie::get_oncall_name(&req.oncall_id).await
    {
        if code == reqwest::StatusCode::NOT_FOUND {
            Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: format!("Oncall with ID {} does not exist", req.oncall_id),
            }))
        } else {
            Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Error fetching oncalls from opsgenie".into(),
            }))
        }
    } else if let Err(e) = slack::get_user_group(&req.user_group_id).await {
        match e {
            slack::Error::UserGroupNotFound => Ok(HttpResponse::NotFound().json(ErrorResponse {
                error: format!("User group with ID {} does not exist", req.user_group_id),
            })),
            e => Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", e),
            })),
        }
    } else {
        let oncall_id = req.oncall_id.clone();
        let user_group_id = req.user_group_id.clone();
        let sync_res =
            match web::block(move || db::add_sync(&conn, &oncall_id, &user_group_id)).await {
                Err(blocking_error) => {
                    return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                        error: format!("{:?}", blocking_error),
                    }));
                }
                Ok(Err(db_error)) => {
                    return Ok(db_error.into());
                }
                Ok(Ok(res)) => res,
            };

        // Add a syncer if not already there
        {
            let key = UserGroupSyncerKey {
                oncall_id: req.oncall_id.clone(),
                user_group_id: req.user_group_id.clone(),
            };
            let mut syncers = data.syncers.lock().await;
            if !syncers.contains_key(&key) {
                syncers.insert(
                    key,
                    UserGroupSyncer::new(req.oncall_id.clone(), req.user_group_id.clone()),
                );
            }
        }
        Ok(HttpResponse::Ok().json(sync_res))
    }
}

#[post("/remove_sync")]
async fn remove_sync(
    req: web::Json<RemoveSyncRequest>,
    data: web::Data<Arc<AppState>>,
) -> Result<impl Responder> {
    let conn = db::connection();
    let oncall_sync_id = req.oncall_sync_id;

    // First, remove the element from the DB
    let deleted_sync = match web::block(move || db::remove_sync(&conn, oncall_sync_id)).await {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    // Delete syncer if present
    {
        let key = UserGroupSyncerKey {
            oncall_id: deleted_sync.oncall_id.clone(),
            user_group_id: deleted_sync.user_group_id.clone(),
        };
        let mut syncers = data.syncers.lock().await;
        if !syncers.contains_key(&key) {
            warn!("Syncer key {:?} not found in cache after delete", key);
        } else {
            syncers.remove(&key);
        }
    }

    return Ok(HttpResponse::Ok().json(deleted_sync));
}

#[get("/synced_with")]
async fn synced_with(info: web::Query<SyncedWithRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let oncall_id = info.oncall_id.clone();
    let oncall_name = opsgenie::get_oncall_name(&oncall_id).await.unwrap();

    let query = match web::block(move || db::get_syncs(&conn, &info.oncall_id)).await {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };
    let user_groups = join_all(
        query
            .iter()
            .map(|sync| slack::get_user_group(&sync.user_group_id)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>();

    let user_groups = match user_groups {
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
        Ok(ug) => ug,
    };

    let syncs = user_groups
        .into_iter()
        .zip(query.into_iter().map(|sync| sync.id))
        .map(|(user_group, id)| OncallSync {
            id,
            oncall_id: oncall_id.clone(),
            oncall_name: oncall_name.clone(),
            user_group_id: user_group.id,
            user_group_name: user_group.name,
            user_group_handle: user_group.handle,
        })
        .collect();

    Ok(HttpResponse::Ok().json(SyncedWithResponse { syncs }))
}

#[get("/list_syncs")]
async fn list_syncs() -> Result<impl Responder> {
    let conn = db::connection();

    let query = match web::block(move || db::list_oncall_syncs(&conn)).await {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };
    let user_groups = join_all(
        query
            .iter()
            .map(|sync| slack::get_user_group(&sync.user_group_id)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>();
    let oncalls = join_all(
        query
            .iter()
            .map(|sync| opsgenie::get_oncall_name(&sync.oncall_id)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>, _>>();

    let user_groups = match user_groups {
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
        Ok(ug) => ug,
    };
    let oncalls = match oncalls {
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
        Ok(oncalls) => oncalls,
    };

    let syncs = query
        .into_iter()
        .zip(user_groups.into_iter())
        .zip(oncalls.into_iter())
        .map(|((sync, user_group), oncall_name)| OncallSync {
            id: sync.id,
            oncall_id: sync.oncall_id,
            oncall_name,
            user_group_id: sync.user_group_id,
            user_group_name: user_group.name,
            user_group_handle: user_group.handle,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ListSyncsResponse { syncs }))
}

#[get("/list_user_mappings")]
async fn list_user_mappings() -> Result<impl Responder> {
    let user_mappings = match web::block(|| {
        let conn = db::connection();
        db::list_user_mappings(&conn)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    let user_mappings = user_mappings
        .into_iter()
        .map(|user_mapping| UserMapping {
            id: user_mapping.id,
            opsgenie_user_id: user_mapping.opsgenie_id,
            slack_user_id: user_mapping.slack_id,
        })
        .collect();

    Ok(HttpResponse::Ok().json(ListUserMappingsResponse { user_mappings }))
}

#[get("/get_slack_user_mapping")]
async fn get_slack_user_mapping(
    info: web::Query<GetSlackUserMappingRequest>,
) -> Result<impl Responder> {
    let slack_user_id = info.into_inner().slack_user_id;
    let user_mapping = match web::block(move || {
        let conn = db::connection();
        db::get_slack_user_mapping(&conn, &slack_user_id)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    }
    .map(|um| UserMapping {
        id: um.id,
        opsgenie_user_id: um.opsgenie_id,
        slack_user_id: um.slack_id,
    });

    Ok(HttpResponse::Ok().json(GetSlackUserMappingResponse { user_mapping }))
}

#[get("/notification/list")]
async fn list_notifications(data: web::Data<Arc<AppState>>) -> Result<impl Responder> {
    // Fetch notifications from DB
    let notifications = match web::block(move || {
        let conn = db::connection();
        db::list_notified_slack_channels(&conn)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    let notifications: Vec<_> = match join_all(notifications.into_iter().map(|notification| {
        let data = data.clone();
        async move { db_notification_to_response(notification, &data).await }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, anyhow::Error>>()
    {
        Ok(notifications) => notifications,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", e),
            }));
        }
    };

    Ok(HttpResponse::Ok().json(ListNotificationsResponse { notifications }))
}

#[get("/notifications/slack")]
async fn get_notification_for_slack_channel(
    data: web::Data<Arc<AppState>>,
    info: web::Query<GetNotificationForSlackChannelRequest>,
) -> Result<impl Responder> {
    let notification = match web::block(move || {
        let conn = db::connection();
        db::get_oncall_notified_in_channel(&conn, &info.slack_channel_id)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    let notification = match join_all(notification.into_iter().map(|notification| {
        let data = data.clone();
        async move { db_notification_to_response(notification, &data).await }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, anyhow::Error>>()
    {
        Ok(notification) => notification,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", e),
            }));
        }
    };

    let notification = notification.into_iter().nth(0);

    Ok(HttpResponse::Ok().json(GetNotificationForSlackChannelResponse { notification }))
}

#[get("/notifications/oncall")]
async fn get_notification_for_oncall(
    data: web::Data<Arc<AppState>>,
    info: web::Query<GetNotificationForOncallRequest>,
) -> Result<impl Responder> {
    let notifications = match web::block(move || {
        let conn = db::connection();
        db::get_channels_notified_for_oncall(&conn, &info.oncall_id)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    let notifications = match join_all(notifications.into_iter().map(|notification| {
        let data = data.clone();
        async move { db_notification_to_response(notification, &data).await }
    }))
    .await
    .into_iter()
    .collect::<Result<Vec<_>, anyhow::Error>>()
    {
        Ok(notifications) => notifications,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", e),
            }));
        }
    };

    Ok(HttpResponse::Ok().json(GetNotificationForOncallResponse { notifications }))
}

#[post("/notifications/add")]
async fn add_notification(
    data: web::Data<Arc<AppState>>,
    req: web::Json<AddNotificationRequest>,
) -> Result<impl Responder> {
    let notification = match web::block(move || {
        let conn = db::connection();
        db::add_channel_oncall_notification(&conn, &req.slack_channel_id, &req.oncall_id)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    let notification = match db_notification_to_response(notification, &data).await {
        Ok(notification) => notification,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", e),
            }));
        }
    };

    // Add notifier if not already there
    {
        let key = SlackNotifierKey {
            oncall_id: notification.oncall_id.clone(),
            slack_channel_id: notification.slack_channel_id.clone(),
        };
        let mut notifiers = data.notifiers.lock().await;
        if !notifiers.contains_key(&key) {
            notifiers.insert(
                key,
                SlackNotifier::new(
                    notification.oncall_id.clone(),
                    notification.slack_channel_id.clone(),
                ),
            );
        }
    }

    Ok(HttpResponse::Ok().json(AddNotificationResponse { notification }))
}

#[post("/notifications/remove")]
async fn remove_notification(
    data: web::Data<Arc<AppState>>,
    req: web::Json<RemoveNotificationRequest>,
) -> Result<impl Responder> {
    let notification = match web::block(move || {
        let conn = db::connection();
        db::remove_channel_oncall_notification(&conn, req.notification_id)
    })
    .await
    {
        Err(blocking_error) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", blocking_error),
            }));
        }
        Ok(Err(db_error)) => {
            return Ok(db_error.into());
        }
        Ok(Ok(res)) => res,
    };

    let notification = match db_notification_to_response(notification, &data).await {
        Ok(notification) => notification,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{}", e),
            }));
        }
    };

    // Delete notifier if present
    {
        let key = SlackNotifierKey {
            oncall_id: notification.oncall_id.clone(),
            slack_channel_id: notification.slack_channel_id.clone(),
        };
        let mut notifiers = data.notifiers.lock().await;
        if !notifiers.contains_key(&key) {
            warn!("Syncer key {:?} not found in cache after delete", key);
        } else {
            notifiers.remove(&key);
        }
    }

    Ok(HttpResponse::Ok().json(RemoveNotificationResponse { notification }))
}

async fn not_found() -> Result<impl Responder> {
    Ok(HttpResponse::NotFound().json(ErrorResponse {
        error: "the requested page does not exist".into(),
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();
    dotenv::from_filename(env::var("DOTENV_FILE").unwrap_or(".env".into()))
        .expect("error loading environment");

    let app_state = Arc::new(AppState::new().await?);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(add_sync)
            .service(remove_sync)
            .service(synced_with)
            .service(list_oncalls)
            .service(list_user_groups)
            .service(list_slack_users)
            .service(list_slack_channels)
            .service(list_opsgenie_users)
            .service(add_user_map)
            .service(remove_user_map)
            .service(list_syncs)
            .service(list_user_mappings)
            .service(get_slack_user_mapping)
            .service(list_notifications)
            .service(get_notification_for_slack_channel)
            .service(get_notification_for_oncall)
            .service(add_notification)
            .service(remove_notification)
            .default_service(web::route().to(not_found))
    })
    .bind((
        "0.0.0.0",
        env::var("PORT")
            .unwrap_or("80".into())
            .parse()
            .expect("unable to parse provided port to a number"),
    ))?
    .run()
    .await?;

    Ok(())
}
