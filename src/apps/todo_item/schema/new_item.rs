use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResultResponse {
    pub success: bool,
}

#[derive(Deserialize, Serialize)]
pub struct SchemaTodoItem {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TodoItem {
    pub id: i32,
}
