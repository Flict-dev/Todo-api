use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct SchemaNewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
pub struct SchemaUser {
    pub name: String,
    pub plain_password: String,
}
