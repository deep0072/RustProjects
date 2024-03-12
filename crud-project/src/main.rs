use axum::{Router, routing::{get,post, delete,put}, Extension};

use crate::controller::{create_user, delete_user, update_user,list_users,get_user_by_id};
mod model;
mod controller;
use crate::user_service::UserService;

mod user_service;

#[tokio::main]
async fn main() {

    println!("server starting....");

    let service = UserService::new().await.unwrap();


    let app = Router::new()
                                .route("/user", get(list_users))
                                .route("/user/:id", get(get_user_by_id))
                                .route("/create_user", post(create_user))
                                .route("/user/:id", put(update_user))
                                .route("/user/:id", delete(delete_user))
                                .layer(Extension(service));

    //.layer(Extension(service)); is used to attach an instance of UserService to the Axum application's layer stack. 
    // This allows all handlers within the application to access the UserService instance through the request's 
    //extensions
    
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    println!("listening....")

    axum::serve(listener,app).await.unwrap();


}
