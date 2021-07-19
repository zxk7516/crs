use actix_web::{
    post,
    web::{self},
};
use rbatis::executor::RbatisExecutor;
use serde::Deserialize;

use crate::errors;
use crate::errors::MyError;
use crate::models::user::User;
use crate::password::AuthenticateUtils;
use rbatis::core::db::db_adapter::DBExecResult;
use rbatis::Error as RBError;

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug)]
struct GetId {
    id: i32,
}

#[html_sql(rb, "sql_mappers/sql.html")]
async fn find_user_by_name_or_email(
    rb: &mut RbatisExecutor<'_>,
    email: &str,
    username: &str,
) -> Option<User> {
}

#[html_sql(rb, "sql_mappers/sql.html")]
async fn save_user(
    rb: &mut RbatisExecutor<'_>,
    email: &str,
    username: &str,
    password: &str,
) -> Result<DBExecResult, RBError> {
}

#[post("/register")]
async fn register_action<'k>(
    register_form: web::Json<RegisterRequest>,
    auth_utils: web::Data<AuthenticateUtils<'k>>,
) -> Result<String, MyError> {
    let result = find_user_by_name_or_email(
        &mut (&*crate::RB).into(),
        &register_form.email,
        &register_form.username,
    )
    .await
    .map_err(|_e| MyError::InternalError)?;

    if result.is_some() {
        return Err(errors::MyError::FieldAlreadyExist);
    }
    let hashed_password = auth_utils.hash_password(&register_form.password)?;
    save_user(
        &mut (&*crate::RB).into(),
        &register_form.email,
        &register_form.username,
        &hashed_password,
    )
    .await
    .map_err(|e| {
        println!("{:?}", e);
        errors::MyError::SqlError
    })?;

    Ok("".to_string())
}
