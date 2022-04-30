use crate::apps::td_models::{NewTodo, TodoList};
use crate::errors::AppError;
use crate::schema::todo_list::dsl::*;
use crate::Connection;
use diesel::prelude::*;

pub fn get_todos(conn: &Connection) -> Result<Vec<TodoList>, AppError> {
    let todos = todo_list
        .select(todo_list::all_columns())
        .order(id.desc())
        .limit(10)
        .load::<TodoList>(conn)
        .map_err(AppError::db_not_found)?;

    Ok(todos)
}

pub fn create_todo<'a>(conn: &Connection, other_title: &'a str) -> Result<TodoList, AppError> {
    let new_todo = NewTodo { title: other_title };

    let new_todo = diesel::insert_into(todo_list)
        .values(&new_todo)
        .get_result(conn)
        .map_err(AppError::db_not_found)?;

    Ok(new_todo)
}
