use crate::models::TodoList;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client.prepare("select * from todo_list").await.unwrap();

    let todos: Vec<TodoList> = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo list")
        .iter()
        .map(|todo| TodoList::from_row_ref(todo).unwrap())
        .collect();
    Ok(todos)
}
