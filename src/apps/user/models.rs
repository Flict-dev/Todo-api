use crate::schema::users;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

// id -> Int4,
// name -> Text,
// password -> Text,
// email -> Text,
// list_id -> Int4,

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
    pub todo_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}
