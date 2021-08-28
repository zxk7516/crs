use crate::errors::MyResult;
use crate::token::JwtDecoder;
use actix_web::{get, web};

#[get("profile")]
pub async fn get_profile(jwt_decoder: web::Data<JwtDecoder<'_, '_>>) -> MyResult<String> {
    todo!("")
}
