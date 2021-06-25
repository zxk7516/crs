use crate::{
    errors::MyError,
    password::AuthenticateUtils,
    token::{Claims, ResToekn, TokenTool},
    PgPool,
};
use actix_web::{
    post,
    web::{self, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct GetUserLogin {
    id: i32,
    username: String,
    email: String,
    #[serde(skip)]
    password: String,
    #[sqlx(rename = "created_at")]
    created_at: DateTime<Local>,
}

#[post("/login")]
pub async fn login_action<'key>(
    login_request: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
    auth_utils: web::Data<AuthenticateUtils<'key>>,
    token_tool: web::Data<TokenTool<'_, '_>>,
) -> Result<Json<ResToekn>, MyError> {
    let user = sqlx::query_as::<_, GetUserLogin>(
        r#"select id,username,email,password,created_at from "users" where email = $1"#,
    )
    .bind(&login_request.email)
    .fetch_one(&**pool)
    .await
    .map_err(|_e| {
        // println!("error:{:?}", _e);
        MyError::InternalError
    })?;
    // println!("{:?}", user);
    if auth_utils.verify_password(&user.password, &login_request.password) {
        let duration = 7 * 24 * 3600;
        let exp = chrono::Local::now().naive_utc().timestamp() + duration;
        let token = token_tool.encode(&Claims { sub: user.id, exp })?;
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
    fn test_chrono() {
    }
}
