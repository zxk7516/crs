#![cfg(debug_assertions)]
#![allow(unused_variables)]
#![cfg(debug_assertions)]
#![allow(unused_imports)]
#![cfg(debug_assertions)]
#![allow(dead_code)]

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;
extern crate argon2;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
    middleware::Logger,
};
use password::AuthenticateUtils;
use rbatis::rbatis::Rbatis;
use std::rc::Rc;
use token::{JwtDecoder, JwtEncoder};

pub mod errors;
pub mod middleware;
pub mod models;
pub mod password;
pub mod routes;
pub mod token;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref JWT_SECRET: String = {
        dotenv::dotenv().ok();
        std::env::var("JWT_SECRET").unwrap_or("abc".to_string())
    };
    static ref JWT_HEADER: jsonwebtoken::Header = jsonwebtoken::Header::default();
    static ref JWT_VALIDATION: jsonwebtoken::Validation = jsonwebtoken::Validation::default();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = std::env::var("DATABASE_URL").unwrap();

    RB.link(&database_url).await.unwrap();

    let password_util = Data::new(AuthenticateUtils::default());

    let jwt_encoder = Data::new(JwtEncoder::new(&JWT_SECRET, &*JWT_HEADER));
    let jwt_decoder = Data::new(JwtDecoder::new(&JWT_SECRET, &*JWT_VALIDATION));

    HttpServer::new(move || {
        App::new()
            .app_data::<Data<AuthenticateUtils>>(password_util.clone())
            .app_data::<Data<JwtEncoder>>(jwt_encoder.clone())
            .app_data::<Data<JwtDecoder>>(jwt_decoder.clone())
            .configure(routes::auth_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
