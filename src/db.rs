use crate::db_models::{TodoItem, TodoList};
use crate::errors::{AppError, AppErrorType};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, AppError> {
    let statement = client
        .prepare("select * from todo_list order by id desc limit 10")
        .await
        .map_err(AppError::db_error)?;

    let todos: Vec<TodoList> = client
        .query(&statement, &[])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|todo| TodoList::from_row_ref(todo).unwrap())
        .collect();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, AppError> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id")
        .await
        .map_err(AppError::db_error)?;

    let items: Vec<TodoItem> = client
        .query(&statement, &[&list_id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|item| TodoItem::from_row_ref(&item).unwrap())
        .collect();

    Ok(items)
}

pub async fn create_list(client: &Client, title: String) -> Result<TodoList, AppError> {
    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .map_err(AppError::db_error)?;

    client
        .query(&statement, &[&title])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|list| TodoList::from_row_ref(&list).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating todo list".to_string()),
            cause: Some("Uknown error".to_string()),
            error_type: AppErrorType::DbError,
        })
}

pub async fn create_item(
    client: &Client,
    lits_id: i32,
    title: String,
) -> Result<TodoItem, AppError> {
    let statement = client
        .prepare(
            "insert into todo_item (title, list_id) values ($1, $2) returning id, title, checked, list_id",
        )
        .await
        .map_err(AppError::db_error)?;

    client
        .query(&statement, &[&title, &lits_id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|item| TodoItem::from_row_ref(&item).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating todo item".to_string()),
            cause: Some("Uknown error".to_string()),
            error_type: AppErrorType::DbError,
        })
}

pub async fn check_todo(client: &Client, id: i32, list_id: i32) -> Result<bool, AppError> {
    let statement = client
        .prepare(
            "update todo_item set checked = true where id = $1 and list_id = $2 and checked = false"
        )
        .await
        .map_err(AppError::db_error)?;

    let result = client
        .execute(&statement, &[&id, &list_id])
        .await
        .map_err(AppError::db_error)?;

    match result {
        ref updated if *updated == 1 => Ok(true),
        _ => Ok(false),
    }
}
