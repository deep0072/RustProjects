use crate::{db::get_database, handler::MyError};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    auth::decode_jwt,
    handler::BulkUser,
    models::{Account, Claims, TokenResponse, User},
};
// Define a struct to hold the user ID
pub struct AuthUser(pub String);

// Define a struct for the JWT claims

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = MyError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db = get_database().await;

        // Extract the token from the Authorization header
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(MyError::TOKENERROR)?;

        // Decode and validate the JWT
        let secret = "secret".to_string();
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        let user_email = token.claims.username;

        // Check if the user exists in MongoDB
        let users_collection: Collection<BulkUser> = db.collection("users");
        let user = users_collection
            .find_one(doc! { "email": user_email.clone() })
            .await?;
        let user_id = user.as_ref().map(|acct| acct._id.clone()).unwrap();

        if user.is_some() {
            Ok(AuthUser(user_id))
        } else {
            Err(MyError::SomethingWentWrong)
        }
    }
}
