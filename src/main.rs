extern crate argon2;

use actix_web::{App, HttpServer};
use password::AuthenticateUtils;
use sqlx::{
    postgres::{PgPoolOptions, Postgres},
    Pool,
};
use token::TokenTool;

pub mod errors;
pub mod password;
pub mod routes;
pub mod token;

pub type PgPool = Pool<Postgres>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();

    let password_util = AuthenticateUtils::default();

    // let token_string = std::env::var("JWT_TOKEN").unwrap();
    // let jwt_token = &token_string;
    let jwt_token = "abc";
    let token_util = TokenTool::new(&jwt_token);

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&database_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data::<PgPool>(pool.clone())
            .data::<AuthenticateUtils>(password_util.clone())
            .data::<TokenTool>(token_util.clone())
            .configure(routes::auth_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
