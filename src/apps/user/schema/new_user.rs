use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SchemaUser {
    pub name: String,
    pub email: String,
    pub hashed_password: String,
}
