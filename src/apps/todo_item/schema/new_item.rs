use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct ResultResponse {
    pub success: bool,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct SchemaTodoItem {
    pub title: String,
}

#[derive(Deserialize, Apiv2Schema)]
pub struct TodoItem {
    pub id: i32,
}
