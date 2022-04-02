use actix_web::{
    post,
    web::{self, Data},
    App, HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

mod db;

#[derive(Serialize, Deserialize)]
struct SyncRequest {
    oncall_name: String,
    user_group: String,
}

#[derive(Serialize, Deserialize)]
struct SyncResponse {
    ok: bool,
}

#[post("/add_sync")]
async fn add_sync(
    _info: web::Json<SyncRequest>,
    _data: Data<SqlitePool>,
) -> Result<impl Responder> {
    Ok(web::Json(SyncResponse { ok: true }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = db::initialise().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(add_sync)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
