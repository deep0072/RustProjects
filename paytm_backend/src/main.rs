use account::get_balance;
use auth::my_middleware;
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, StatusCode,
    },
    middleware,
    middleware::from_fn,
    routing::{get, post},
    Extension, Json, Router,
};

use tower::ServiceBuilder;

mod account;
mod auth;
mod db;
mod handler;
mod models;
use std::sync::Arc;
mod custom_extractor;

use crate::db::get_database;

use crate::handler::{bulk_user, create_user, login_user, transfer_money};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let db = get_database().await;
    let app = Router::new()
        .route("/transfer", post(transfer_money))
        .route("/bulk", get(bulk_user))
        .route("/get_balance", get(get_balance))
        .route("/", get(|| async { "Hello, World!" }))
        .route_layer(middleware::from_fn(my_middleware))
        .route("/login", post(login_user))
        .route("/create_user", post(create_user))
        .layer(Extension(db.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_credentials(true)
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
