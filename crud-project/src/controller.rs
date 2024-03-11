use axum::http::StatusCode;
use axum::Json;
use axum::extract::Path;
use crate::model::{User, UserInfo};

use serde_json::Value;

pub async fn list_users()->(StatusCode, Json<Value>){
    // get users

}

pub async fn get_user_by_id(Path(id):Path<i64>)->(StatusCode, Json<Value>) {
    // get use by id

}


pub async fn create_user(Json(user):Json<UserInfo>) -> (StatusCode, Json<Value>){
    // createa user



}

pub async fn update_user(Path(id):Path<i64>, Json(user):Json<UserInfo>)-> (StatusCode, Json<Value>){
    // update user 

}

pub async fn delete_user(Path(id): Path<i64>) -> (StatusCode) {
    // delete user

}