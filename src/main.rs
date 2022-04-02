#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

mod db;
mod models;
mod schema;

#[derive(Serialize, Deserialize, Debug)]
struct AddSyncRequest {
    oncall_name: String,
    user_group: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithRequest {
    oncall_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SyncedWithResponse {
    oncall_name: String,
    user_groups: Vec<String>,
}

#[post("/add_sync")]
async fn add_sync(req: web::Json<AddSyncRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let sync_res = web::block(move || {
        db::add_sync(&conn, &req.oncall_name, &req.user_group).expect("This is an error")
    })
    .await
    .unwrap();
    Ok(HttpResponse::Ok().json(sync_res))
}

#[get("/synced_with")]
async fn synced_with(req: web::Json<SyncedWithRequest>) -> Result<impl Responder> {
    let conn = db::connection();
    let oncall_name = req.oncall_name.clone();
    let query = web::block(move || db::get_syncs(&conn, &req.oncall_name))
        .await
        .unwrap()
        .unwrap();
    Ok(HttpResponse::Ok().json(SyncedWithResponse {
        oncall_name,
        user_groups: query.iter().map(|sync| sync.user_group.clone()).collect(),
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(move || App::new().service(add_sync).service(synced_with))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
