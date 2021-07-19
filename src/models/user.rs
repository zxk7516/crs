use chrono::{DateTime, Local};

#[crud_table(table_name:users)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Local>,
}
