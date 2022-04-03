#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use futures_util::future::join_all;
use serde::{Deserialize, Serialize};

mod db;
mod models;
mod opsgenie;
mod schema;
mod slack;

#[derive(Serialize, Deserialize, Debug)]
struct AddSyncRequest {
    oncall_id: String,
    user_group_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithRequest {
    oncall_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithResponse {
    oncall_id: String,
    oncall_name: String,
    user_groups: Vec<slack::UserGroup>,
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
struct AddUserMapRequest {
    slack_id: String,
    opsgenie_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    error: String,
}

#[get("/list_slack_users")]
async fn list_slack_users() -> Result<impl Responder> {
    let users = match slack::list_users().await {
        Ok(users) => users,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(ErrorResponse {
                error: format!("{:?}", e),
            }));
        }
    };
    Ok(HttpResponse::Ok().json(ListSlackUsersResponse { users }))
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
async fn list_oncalls() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(ListOncallsResponse {
        oncalls: opsgenie::list_oncalls().await,
    }))
}

#[post("/add_user_map")]
async fn add_user_map(req: web::Json<AddUserMapRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let add_res = web::block(move || {
        db::add_user_mapping(&conn, &req.opsgenie_id, &req.slack_id).expect("This is an error")
    })
    .await
    .unwrap();
    Ok(HttpResponse::Ok().json(add_res))
}

#[post("/add_sync")]
async fn add_sync(req: web::Json<AddSyncRequest>) -> Result<impl Responder> {
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
        let sync_res = web::block(move || {
            db::add_sync(&conn, &req.oncall_id, &req.user_group_id).expect("This is an error")
        })
        .await
        .unwrap();
        Ok(HttpResponse::Ok().json(sync_res))
    }
}

#[get("/synced_with")]
async fn synced_with(req: web::Json<SyncedWithRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let oncall_id = req.oncall_id.clone();
    let oncall_name = opsgenie::get_oncall_name(&oncall_id).await.unwrap();

    let query = web::block(move || db::get_syncs(&conn, &req.oncall_id))
        .await
        .unwrap()
        .unwrap();
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

    Ok(HttpResponse::Ok().json(SyncedWithResponse {
        oncall_id,
        oncall_name,
        user_groups,
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .service(add_sync)
            .service(synced_with)
            .service(list_oncalls)
            .service(list_user_groups)
            .service(list_slack_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
