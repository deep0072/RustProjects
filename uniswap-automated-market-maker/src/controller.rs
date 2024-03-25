use axum::{http::StatusCode, Extension};

use axum::{Json, Error};
use serde_json::Value;

use crate::models::{LiquidityPool, AssetPrice};
use crate::market_maker_services::{add_liquidity, buy_asset,sell_asset};


pub async fn add_liquidty(Json(liquidity_info): Json<LiquidityPool>) -> Result<Json<LiquidityPool>,StatusCode> {

    match add_liquidity(liquidity_info).await {
        Ok(liquidity_info) =>Ok(Json(liquidity_info)),
        Err(err) =>{
            eprintln!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }

    

}

pub async fn buy_aset(Json(payload): Json<Value>) -> Result<Json<AssetPrice>,StatusCode> {
    let payload: Value = serde_json::from_str(&payload.to_string()).unwrap();
    let ethers = payload["ethers"].as_i64().unwrap_or(0) as u64;
    println!("ethers {:?}",ethers);
    match buy_asset(ethers).await {
        Ok(AssetPrice) =>Ok(Json(AssetPrice)),
        Err(err)=>{
            eprintln!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)

        }
       
    }

    

}


pub async fn sel_asset(Json(payload): Json<Value>) -> Result<Json<AssetPrice>, StatusCode> {
    let payload: Value = serde_json::from_str(&payload.to_string()).unwrap();
    println!("payload {}", payload);
    let ethers = payload["ethers"].as_i64().unwrap_or(0) as u64;
    println!("ethers {:?}",ethers);
    match sell_asset(ethers).await {
        Ok(AssetPrice) =>Ok(Json(AssetPrice)),
        Err(err)=>{
            eprintln!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)

        }
       
    }
}