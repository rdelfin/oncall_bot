use actix_web::{get, post, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SyncRequest {
    oncall_name: String,
    user_group: String,
}

#[derive(Serialize, Deserialize)]
struct SyncResponse {
    ok: bool,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/add_sync")]
async fn add_sync(info: web::Json<SyncRequest>) -> Result<impl Responder> {
    Ok(web::Json(SyncResponse { ok: true }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(add_sync))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
