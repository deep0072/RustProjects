use axum::{Router, routing::{get,post, delete,put}, Extension};

use crate::controller::{add_liquidty,buy_aset,sel_asset};

mod models;
mod controller;
mod market_maker_services;
use balance::Balance;

mod balance;  // Import the balance module from balance.rs




#[tokio::main]
async fn main() {
    Balance::init_balances(200, 700000);
    println!("Hello, world!");

   


    

    let app = Router::new()
        .route("/add-liquidity", post(add_liquidty))
        .route("/buy-asset", post(buy_aset))
        .route("/sell-asset", post(sel_asset));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    match axum::serve(listener,app).await {
        Ok(_) =>       println!("listenkkkkking...."),
        Err(e)=>{
            println!("connection error {}",e)
        }

    }
}
