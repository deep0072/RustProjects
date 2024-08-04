use crate::{
    account::UserParams,
    models::{Account, Claims, TokenResponse, User},
    custom_extractor::AuthUser
};



use axum::{
    body,
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use futures::TryStreamExt;

use serde_json::{json, Value};
// use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::auth::encode_jwt;
use mongodb::{
    bson::{doc, oid::ObjectId,serde_helpers},
    error::{ TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT},

    options::FindOptions,
    Collection, Cursor, Database,ClientSession,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display, u128};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
   
    pub to:String,
    pub amount: String
}



#[derive(Debug, Serialize, Deserialize)]
pub struct BulkUser {
    #[serde(deserialize_with= "serde_helpers::deserialize_hex_string_from_object_id")]
    pub _id:String,
    
    pub username: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupRequest {
    username: String,
    email: String,
    password: String,
    firstname: String,
    lastname: String,
}

use std::any::type_name;

pub fn type_of<T>(_: &T) -> &str {
    type_name::<T>()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    filter: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInRequest {
    email: String,
    password: String,
}

#[derive(Debug)]
pub enum MyError {
    SomethingWentWrong,
    MONGOERROR,
    ENCODERROR,
    TOKENERROR,
}

impl From<mongodb::error::Error> for MyError {
    fn from(value: mongodb::error::Error) -> Self {
        MyError::MONGOERROR
    }
}
impl From<jsonwebtoken::errors::Error> for MyError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        MyError::TOKENERROR
    }
}

impl From<axum::http::StatusCode> for MyError {
    fn from(value: axum::http::StatusCode) -> Self {
        MyError::ENCODERROR
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let body = match self {
            MyError::SomethingWentWrong => "something went wrong",
            MyError::MONGOERROR => "mongo error Error",
            MyError::ENCODERROR => "encode error",
            MyError::TOKENERROR => "token error",
        };

        // its often easiest to implement `IntoResponse` by calling other implementations
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl Error for MyError {}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let message = match self {
        //     Self::MONGOERROR => "Error",
        //     Self::SomethingWentWrong => "something went wrong",
        //     Self::ENCODERROR => "Encodeing went wrong",
        //     Self::TOKENERROR => "TOKENERROR wrong",
        // };
        write!(f, "Error")
    }
}

// Box<dyn Error>> used to handle any type of error
#[axum_macros::debug_handler]
pub async fn create_user(
    Extension(db): Extension<Database>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<TokenResponse>, MyError> {
    let users = User {
        username: payload.username,
        email: payload.email,
        password: payload.password,
        firstname: payload.firstname,
        lastname: payload.lastname,
    };
    let db_ref1 = db.clone();
    let collection: Collection<User> = db_ref1.collection("users"); // Ensure the type matches the collection documents
    let result = collection.insert_one(&users).await?;

    let user_obj_id = result.inserted_id.clone().to_string();
    let random_multiplier: i64 = rand::thread_rng().gen_range(1..=5000 / 100);
    // println!("user_obj_id.clone(), {}", user_obj_id.clone());
    let account = Account {
        user_id: user_obj_id,
        balance: random_multiplier * 1000,
    };
    println!("account {:?}", account);
    let db_ref2 = db.clone();
    let account_collection: Collection<Account> = db_ref2.collection("accounts");

    let result = account_collection.insert_one(&account).await?;
    println!("account, {:?}", &account);

    let token = encode_jwt(users.email.clone()).await?;

    Ok(Json(TokenResponse {
        token: token,
        user_id: users.email,
    }))
}

#[axum_macros::debug_handler]
pub async fn login_user(
    Extension(db): Extension<Database>,
    Json(payload): Json<SignInRequest>,
) -> Result<Json<TokenResponse>, MyError> {
    let collection: Collection<User> = db.collection("users");

    let result = collection
        .find_one(doc! {
            "email": payload.email.clone(),
            "password": payload.password


        })
        .await?;
    let user_email = result
        .as_ref()
        .map(|user| user.email.clone())
        .unwrap_or("0".to_string());
    if user_email.clone() == payload.email {
        let token = encode_jwt(user_email).await?;

        Ok(Json(TokenResponse {
            token: token,
            user_id: payload.email,
        }))
    } else {
        Err(MyError::SomethingWentWrong)
    }
}

pub async fn bulk_user(
    Extension(db): Extension<Database>,
    user_params: Query<QueryParams>,
) -> Result<Json<Value>, MyError> {
    let collection: Collection<BulkUser> = db.collection("users");
    let filter = &user_params.filter.clone();

    let mut cursor = collection
        .find(doc! {
           "$or": vec! [
              doc! { "firstname": { "$regex":&filter}},
              doc! { "lastname": {"$regex":&filter}}
           ]
        })
        .await?;

    let result:Vec<BulkUser> = cursor.try_collect().await?;

    Ok(Json(json!({"user":result})))
}


pub async fn transfer_money( AuthUser(user_id): AuthUser,Extension(db): Extension<Database>,Json(payload):Json<TransferRequest>) -> Result<Json<String>, MyError> {
    let from = user_id;
    let to = payload.to;
    let amount:i64 = payload.amount.parse().expect("Not a valid number");
   

    let mut session = db.client().start_session().await?;
    session.start_transaction().await?;
    
    

    let collection:Collection<Account> = db.collection("accounts");
    let from_obj_id = format!("ObjectId(\"{}\")", from.clone());
    let to_obj_id = format!("ObjectId(\"{}\")", to.clone());
    println!("from: {} to: {}", from_obj_id.clone(),from_obj_id.clone() );
    let from_user = collection.find_one_and_update(doc! {"user_id": from_obj_id },doc!{ "$inc": { "balance": -amount.clone() }}).session(&mut session).await?;
    let to_user = collection.find_one_and_update(doc! {"user_id": to_obj_id }, doc!{"$inc":  { "balance": amount.clone() }}).session(&mut session).await?;
    session.commit_transaction().await?;
    Ok(Json("successfully transfered".to_string()))

}



