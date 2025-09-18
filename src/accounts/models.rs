use serde::Deserialize;
use uuid::Uuid;

pub struct Account {
    id: Uuid,
    name: String,
    balance: i64,
    password_hash: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}
