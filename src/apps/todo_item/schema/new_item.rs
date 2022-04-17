use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResultResponse {
    pub success: bool,
}

#[derive(Deserialize, Serialize)]
pub struct SchemaTodoItem {
    pub title: String,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
}
