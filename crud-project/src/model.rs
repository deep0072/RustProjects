use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]

pub struct User {
    pub id: i32,
    pub name: String,
    pub Occupation: String,
    pub email:String,
    pub phone: String
}
pub struct UserInfo {
  
    pub name: String,
    pub Occupation: String,
    pub email:String,
    pub phone: String
}