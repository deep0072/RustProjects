use mongodb::bson::{oid::ObjectId, serde_helpers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub user_id: String,
    pub balance: i64,
}

// is used to store
// username that will
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub user_id: String,
}
