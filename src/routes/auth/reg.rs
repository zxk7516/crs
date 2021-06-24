use actix_web::{
    post,
    web::{self, Data},
};
use serde::Deserialize;

use crate::errors;
use crate::password::AuthenticateUtils;
use crate::{errors::MyError, PgPool};

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(sqlx::FromRow, Debug)]
struct GetId {
    id: i32,
}

#[post("/register")]
async fn register_action<'k>(
    register_form: web::Json<RegisterRequest>,
    pool: Data<PgPool>,
    auth_utils: web::Data<AuthenticateUtils<'k>>,
) -> Result<String, MyError> {
    let result =
        sqlx::query_as::<_, GetId>(r#"select id from "users" where username = $1 or email = $2"#)
            .bind(&register_form.username)
            .bind(&register_form.email)
            .fetch_optional(&**pool)
            .await
            .map_err(|_e| errors::MyError::SqlError)?;
    if result.is_some() {
        return Err(errors::MyError::FieldAlreadyExist);
    }
    let hashed_password = &auth_utils.hash_password(&register_form.password)?;
    sqlx::query(
        r#"
INSERT INTO "users" (username, email, password) values ($1, $2, $3) RETURNING id
"#,
    )
    .bind(&register_form.username)
    .bind(&register_form.email)
    .bind(&hashed_password)
    .fetch_one(&**pool)
    .await
    .map_err(|e| {
        println!("{:?}", e);
        errors::MyError::SqlError
    })?;

    Ok("".to_string())
}