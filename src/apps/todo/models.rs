use crate::schema::todo_list;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "todo_list"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub user_id: i32,
}
