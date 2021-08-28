use actix_web::{
    get, post,
    web::{self, scope, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
mod auth;
mod private;

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
use crate::errors::{MyError::LoginFailed, MyResult};
use crate::token::Claims;

pub fn authed_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        scope("/api/datas/")
            /*.wrap_fn(
                |req, srv| -> () {
                    let s = match req.headers().get("Authorization") {
                        Some(bear_token) => bear_token,
                        None => return Err(LoginFailed),
                    };
                    let token: String = s.into();
                    let token_split = token.split_whitespace();
                    match token_split.next() {
                        Some(b) => {
                            if b == "bearer" {
                                match token_split.next() {
                                    Some(t) => {
                                        let token_util = req.app_data::<TokenTool>().unwrap();
                                        let claim = token_util.decode(t)?;
                                        Err(LoginFailed)
                                    }
                                    None => Err(LoginFailed),
                                }
                            } else {
                                Err(LoginFailed)
                            }
                        }
                        None => Err(LoginFailed),
                    }
                },
            )*/
            .service(echo),
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
