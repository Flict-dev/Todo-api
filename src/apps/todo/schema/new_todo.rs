use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub status: String,
}

#[derive(Deserialize, Serialize)]
pub struct SchemaTodo {
    pub title: String,
}
