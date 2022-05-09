use crate::schema::users;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}
