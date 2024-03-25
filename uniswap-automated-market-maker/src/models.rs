use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]




pub struct LiquidityPool {
    pub eth_balance: u64,
    pub usdt_balance: u64
}

#[derive(Serialize,Deserialize)]
pub struct AssetPrice {
    pub eth_price:u64,


}


