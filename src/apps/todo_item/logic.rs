use crate::apps::ti_models::{NewTodoItem, TodoItem};
use crate::errors::AppError;
use crate::schema::todo_item::dsl::*;
use crate::Connection;
use chrono::Utc;
use diesel::prelude::*;

pub fn get_items(conn: &Connection, ti_list_id: i32) -> Result<Vec<TodoItem>, AppError> {
    let todo_items = todo_item
        .filter(list_id.eq(ti_list_id))
        .load::<TodoItem>(conn)
        .map_err(AppError::db_not_found)?;

    Ok(todo_items)
}

pub fn create_item<'a>(
    conn: &Connection,
    ti_list_id: i32,
    ti_title: &'a str,
) -> Result<TodoItem, AppError> {
    let new_todo_item = NewTodoItem {
        list_id: ti_list_id,
        title: ti_title,
        updated: Utc::now().naive_utc(),
    };

    let new_todo_item = diesel::insert_into(todo_item)
        .values(&new_todo_item)
        .get_result(conn)
        .map_err(AppError::db_not_found)?;

    Ok(new_todo_item)
}

pub fn check_todo_item(conn: &Connection, ti_id: i32, ti_list_id: i32) -> Result<bool, AppError> {
    let up_todo_item = diesel::update(
        todo_item.filter(
            id.eq(ti_id)
                .and(list_id.eq(ti_list_id))
                .and(checked.eq(false)),
        ),
    )
    .set(checked.eq(true))
    .get_result::<TodoItem>(conn)
    .map_err(AppError::db_not_found)?;

    match up_todo_item {
        ref updated_item if updated_item.checked => Ok(true),
        _ => Ok(false),
    }
}

pub fn delete_item(conn: &Connection, ti_id: i32, ti_list_id: i32) -> Result<usize, AppError> {
    diesel::delete(todo_item)
        .filter(id.eq(ti_id).and(list_id.eq(ti_list_id)))
        .execute(conn)
        .map_err(AppError::db_not_found)
}
