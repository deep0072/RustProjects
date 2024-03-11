use axum::{Router, routing::{get,post, delete,put}};

use crate::controller::{create_user, delete_user, update_user,list_users,get_user_by_id};
mod model;
mod controller;

#[tokio::main]
async fn main() {

    let app = Router::new()
                                .route("/user", get(list_users))
                                .route("/user/:id", get(get_user_by_id))
                                .route("/create_user", post(create_user))
                                .route("/user/:id", put(update_user))
                                .route("/user/:id", delete(delete_user));
    
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();

    axum::serve(listener,app).await.unwrap();


}
