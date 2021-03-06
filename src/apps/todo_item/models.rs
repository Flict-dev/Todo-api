use crate::schema::todo_item;
use chrono::NaiveDateTime;
use diesel::{Associations, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Associations)]
#[diesel(belongs_to(TodoList))]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub updated: NaiveDateTime,
    pub list_id: i32,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "todo_item"]
pub struct NewTodoItem<'a> {
    pub title: &'a str,
    pub updated: NaiveDateTime,
    pub list_id: i32,
}
