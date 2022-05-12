use crate::apps::td_models::{NewTodo, TodoList};
use crate::errors::AppError;
use crate::schema::{todo_item, todo_list};
use crate::Connection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todos {
    todo: TodoList,
    quantity: i64,
}

pub fn get_todos(conn: &Connection, q_user_id: i32) -> Result<Vec<Todos>, AppError> {
    let todos = todo_list::table
        .select((
            todo_list::table::all_columns(),
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(todo_item.id)"),
        ))
        .left_join(todo_item::table.on(todo_list::id.eq(todo_item::list_id)))
        .group_by(todo_list::id)
        .filter(todo_list::user_id.eq(q_user_id))
        .load::<Todos>(conn)
        .map_err(AppError::db_not_found)?;

    Ok(todos)
}

pub fn create_todo<'a>(
    conn: &Connection,
    other_title: &'a str,
    new_user_id: i32,
) -> Result<TodoList, AppError> {
    let new_todo = NewTodo {
        title: other_title,
        user_id: new_user_id,
    };

    let new_todo = diesel::insert_into(todo_list::table)
        .values(&new_todo)
        .get_result(conn)
        .map_err(AppError::db_not_found)?;

    Ok(new_todo)
}

pub fn delete_todo<'a>(conn: &Connection, todo_id: i32) -> Result<usize, AppError> {
    diesel::delete(todo_list::table)
        .filter(todo_list::id.eq(todo_id))
        .execute(conn)
        .map_err(AppError::db_not_found)
}
