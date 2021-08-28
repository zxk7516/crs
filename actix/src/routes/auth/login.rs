use crate::models::user::User;
use crate::{
    errors::MyError,
    password::AuthenticateUtils,
    token::{Claims, JwtDecoder, JwtEncoder, ResToekn},
};
use actix_web::{
    post,
    web::{self, Data, Json},
};
use rbatis::executor::RbatisExecutor;
use serde::Deserialize;
use std::rc::Rc;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[html_sql(rb, "sql_mappers/sql.html")]
async fn find_user_by_email(rb: &mut RbatisExecutor<'_>, email: &str) -> Option<User> {}

#[post("/login")]
pub async fn login_action<'key>(
    login_request: web::Json<LoginRequest>,
    auth_utils: Data<AuthenticateUtils<'key>>,
    jwt_encoder: Data<JwtEncoder<'_>>,
) -> Result<Json<ResToekn>, MyError> {
    let user = find_user_by_email(&mut (&*crate::RB).into(), &login_request.email)
        .await
        .map_err(|_e| MyError::InternalError)? // escape -> db error
        .ok_or(MyError::InternalError)?; //  some -> result

    if auth_utils.verify_password(&user.password, &login_request.password) {
        let duration = 7 * 24 * 3600;
        let exp = chrono::Local::now().naive_utc().timestamp() + duration;
        let token = jwt_encoder.encode(&Claims { sub: user.id, exp })?;
        Ok(web::Json(ResToekn {
            token,
            expires: duration,
        }))
    } else {
        return Err(MyError::LoginFailed);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_chrono() {}
}
