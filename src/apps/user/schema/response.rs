use crate::apps::u_models::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    pub token: String,
    pub user: User,
}
