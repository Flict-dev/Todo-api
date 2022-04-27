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

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoyLCJleHAiOjE2NTEwOTU2OTF9.-c1G4C1RdM75GJyXdXvsf3CecZ-ET4b-p2bdY6jN4Kc