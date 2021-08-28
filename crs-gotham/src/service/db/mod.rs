use crate::service::dto::User;
use rbatis::core::db::db_adapter::DBExecResult;
use rbatis::executor::RbatisExecutor;
use rbatis::Error as RBError;

#[html_sql(rb, "sql_mappers/sql.html")]
pub async fn find_user_by_name_or_email(
    rb: &mut RbatisExecutor<'_>,
    email: &str,
    username: &str,
) -> Option<User> {
}

#[html_sql(rb, "sql_mappers/sql.html")]
pub async fn save_user(
    rb: &mut RbatisExecutor<'_>,
    email: &str,
    username: &str,
    password: &str,
) -> Result<DBExecResult, RBError> {
}

#[html_sql(rb, "sql_mappers/sql.html")]
pub async fn find_user_by_email(rb: &mut RbatisExecutor<'_>, email: &str) -> Option<User> {}
