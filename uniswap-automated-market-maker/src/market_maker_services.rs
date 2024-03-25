use axum::{Error, http::StatusCode};


use crate::{models::{LiquidityPool, AssetPrice}};

use crate::balance::Balance;













pub async fn add_liquidity(liquidity_info:LiquidityPool) -> Result<LiquidityPool,Error> {
    let (eth_balance, usdt_balance) = Balance::get_balance();
   
  

    let liquidity = LiquidityPool {
        eth_balance:eth_balance,
        usdt_balance:usdt_balance

    };


   


   

    Ok(liquidity)


}

    
pub async fn buy_asset(ethers:u64) -> Result <AssetPrice,StatusCode> {
    println!("asset you are buying {}", ethers);

    // The Uniswap Protocol AMM sets prices for liquidity pools using the mathematical 
    // formula ether*usdt=k

    // lets say when user buy ether then the usdt_balance of ether determined by 
    // updated_usdt_balance = k / updated_ether_balance

    // where k = eth_balance *  usdt_balance

    // paid_amount =  updated_usdt_price - usdt_balance

    // 1. 

    let (eth_balance, usdt_balance) = Balance::get_balance();
    println!("ether_balance, usdt_balance {} {} ", eth_balance,usdt_balance);

 
 
    if ethers >0 {
    let updated_ether_balance = eth_balance - ethers;

    println!("eth_balance * usdt_balance: {} ", eth_balance * usdt_balance);
    let updated_usdt_balance = eth_balance * usdt_balance/ updated_ether_balance;
    println!("updated ether_balance, updated usdt_balance {} {} ", updated_ether_balance,updated_usdt_balance);
    

    let paid_amount = updated_usdt_balance - usdt_balance;

    LiquidityPool {
        eth_balance:updated_ether_balance,
        usdt_balance:updated_usdt_balance
    };

    let asset_price = AssetPrice {
        eth_price:paid_amount
    };

    Balance::update_balances(updated_ether_balance, updated_usdt_balance);
    Ok(asset_price)

    }else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)

    }
}
    



    

    




    

pub async fn sell_asset(ethers:u64) -> Result<AssetPrice, Error> {

    // The Uniswap Protocol AMM sets prices for liquidity pools using the mathematical 
    // formula ether*usdt=k

    // lets say when user sell ether then the usdt_balance of ether determined by following steps
    // ether balance will be increased in liquidity pool and usd balance will decrease
    // updated_usdt_balance = k / updated_ether_balance

    // where k = eth_balance *  usdt_balance

    // paid_amount = usdt_balance - updated_usdt_price  

   
 

    
    let (ether_balance,usdt_balance) = Balance::get_balance();
    println!("ether_balance, usdt_balance {} {} ", ether_balance,usdt_balance);

    let updated_ether_balance = ether_balance + ethers;
    println!("ether_balance* usdt_balance : {}", ether_balance* usdt_balance);
   
    let updated_usdt_balance = ether_balance* usdt_balance / updated_ether_balance;
    println!("updated ether balance, updated usdt balance {} {}", updated_ether_balance, updated_usdt_balance);

    let paid_amount =  usdt_balance - updated_usdt_balance;
    
    LiquidityPool {
        eth_balance:updated_ether_balance,
        usdt_balance:updated_usdt_balance
    };

    Balance::update_balances(updated_ether_balance,updated_usdt_balance);

    let  asset_price = AssetPrice {
        eth_price:paid_amount

    };


    

    Ok(asset_price)




    
}