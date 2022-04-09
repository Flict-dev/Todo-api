use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResultResponse {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct CreateList {
    pub title: String,
}

#[derive(Deserialize)]
pub struct CreateListItem {
    pub title: String,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct ListItem {
    pub id: i32,
    pub list_id: i32,
}
