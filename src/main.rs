#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

mod db;
mod models;
mod opsgenie;
mod schema;

#[derive(Serialize, Deserialize, Debug)]
struct AddSyncRequest {
    oncall_id: String,
    user_group: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithRequest {
    oncall_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithResponse {
    oncall_id: String,
    oncall_name: String,
    user_groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListOncallsResponse {
    oncalls: Vec<opsgenie::Oncall>,
}

#[get("/list_oncalls")]
async fn list_oncalls() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(ListOncallsResponse {
        oncalls: opsgenie::list_oncalls().await,
    }))
}

#[post("/add_sync")]
async fn add_sync(req: web::Json<AddSyncRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let sync_res = web::block(move || {
        db::add_sync(&conn, &req.oncall_id, &req.user_group).expect("This is an error")
    })
    .await
    .unwrap();
    Ok(HttpResponse::Ok().json(sync_res))
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

    Ok(HttpResponse::Ok().json(SyncedWithResponse {
        oncall_id,
        oncall_name,
        user_groups: query.iter().map(|sync| sync.user_group.clone()).collect(),
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
