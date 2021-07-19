//#[crud_table(table_name:user)]
//#[derive(Clone, Debug)]
//pub struct User {
//    pub id: Option<i32>,
//    pub name: Option<String>,
//    pub email: Option<String>,
//    pub password: Option<String>,
//}

use chrono::{DateTime, Local};

#[crud_table(table_name:users)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    // #[serde(skip)]
    pub password: String,
    pub created_at: DateTime<Local>,
}
