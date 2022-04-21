use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SchemaNewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct SchemaUser {
    pub name: String,
    pub plain_password: String,
}
