#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate lazy_static;
extern crate argon2;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use password::AuthenticateUtils;
use rbatis::rbatis::Rbatis;
use token::TokenTool;

pub mod errors;
pub mod models;
pub mod password;
pub mod routes;
pub mod token;

lazy_static! {
    static ref RB:Rbatis = Rbatis::new();
    //static ref RB:Arc<Rbatis> = Arc::new(Rbatis::new());
    static ref JWT_SECRET: String = {
        dotenv::dotenv().ok();
        std::env::var("JWT_SECRET").unwrap_or("abc".to_string())
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    //let rb = Rbatis::new();
    RB.link(&database_url).await.unwrap();

    let password_util = AuthenticateUtils::default();

    // let token_string = std::env::var("JWT_TOKEN").unwrap();
    // let jwt_token = &token_string;
    // let jwt_token = "abc";
    let token_util = TokenTool::new(&JWT_SECRET);

    HttpServer::new(move || {
        App::new()
            .app_data::<Data<AuthenticateUtils>>(web::Data::new(password_util.clone()))
            .app_data::<Data<TokenTool>>(Data::new(token_util.clone()))
            .configure(routes::auth_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
