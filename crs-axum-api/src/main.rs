#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;

use crate::utils::PasswordUtil;
use axum::{
    handler::{get, post},
    Router,
};
use rbatis::rbatis::Rbatis;

mod errors;
mod routes;
mod service;
mod utils;

lazy_static! {
    static ref _ENV: () = {
        dotenv::dotenv().ok();
        ()
    };
    static ref RB: Rbatis = Rbatis::new();
}

#[tokio::main]
async fn main() {
    let _ = *_ENV;
    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("Init db connection(rbatis).[link]({})", database_url);
    RB.link(&database_url).await.unwrap();

    let _argon_key = [0, 0, 0, 0u8];
    let _pu = PasswordUtil::default();

    let app = Router::new().route("/", get(|| async { "Hello, world" }))
        .route("/foo", get(|| async { "Hello, foo" }))
        //.route("/api/register", post(service::actions::register_action))
        ;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
