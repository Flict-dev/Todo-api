use crate::db_models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client
        .prepare("select * from todo_list order by id desc limit 10")
        .await
        .unwrap();

    let todos: Vec<TodoList> = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo list")
        .iter()
        .map(|todo| TodoList::from_row_ref(todo).unwrap())
        .collect();
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id")
        .await
        .unwrap();

    let items: Vec<TodoItem> = client
        .query(&statement, &[&list_id])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|item| TodoItem::from_row_ref(&item).unwrap())
        .collect();

    Ok(items)
}

pub async fn create_list(client: &Client, title: String) -> Result<TodoList, io::Error> {
    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .unwrap();

    client
        .query(&statement, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|list| TodoList::from_row_ref(&list).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating todo list",
        ))
}

pub async fn create_item(
    client: &Client,
    lits_id: i32,
    title: String,
) -> Result<TodoItem, io::Error> {
    let statement = client
        .prepare(
            "insert into todo_item (title, list_id) values ($1, $2) returning id, title, checked, list_id",
        )
        .await
        .unwrap();

    client
        .query(&statement, &[&title, &lits_id])
        .await
        .expect("Error creating todo item")
        .iter()
        .map(|item| TodoItem::from_row_ref(&item).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating todo item",
        ))
}

pub async fn check_todo(client: &Client, id: i32, list_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare(
            "update todo_item set checked = true where id = $1 and list_id = $2 and checked = false"
        )
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&id, &list_id])
        .await
        .expect("Error checking todo item");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to check todo item",
        )),
    }
}
