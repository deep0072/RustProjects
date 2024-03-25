// balance.rs

pub struct Balance {
    pub eth_balance: u64,
    pub usdt_balance: u64,
}

// Define global mutable static variables for eth_balance and usdt_balance
pub static mut ETH_BALANCE: u64 = 0;
pub static mut USDT_BALANCE: u64 = 0;

// Implement functions to initialize and update the balances
impl Balance {
    pub fn init_balances(eth: u64, usdt: u64) {
        unsafe {
            ETH_BALANCE = eth;
            USDT_BALANCE = usdt;
        }
    }

    pub fn update_balances(eth: u64, usdt: u64) {
        unsafe {
            ETH_BALANCE += eth;
            USDT_BALANCE += usdt;
        }
    }

    // Function to get the current balances
    pub fn get_balance() -> (u64, u64) {
        unsafe {
            (ETH_BALANCE, USDT_BALANCE)
        }
    }
}
