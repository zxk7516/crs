use actix_web::{
    get, post,
    web::{self, scope, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
mod auth;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(test_json)
            .service(echo)
            .service(health)
            .service(auth::register_action)
            .service(auth::login_action),
    );
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TestJson {
    hello_world: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    "health"
}
#[get("/echo")]
async fn echo() -> impl Responder {
    "echo"
}
#[post("/test_json")]
async fn test_json(info: web::Json<TestJson>) -> impl Responder {
    Json(info.0)
}
