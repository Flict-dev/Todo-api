use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct SchemaNewTodo {
    pub title: String,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct SchemaTodo {
    pub todo_id: i32,
}
