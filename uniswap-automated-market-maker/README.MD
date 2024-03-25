## Simple Rust API Mimicking Automated Market Maker (AMM)

created simple Rust api for  Uniswap  is like a simple shop where you can trade cryptocurrencies. It's designed to help you swap one cryptocurrency for another easily and without needing a middleman.

### How Uniswap Works

1. **Liquidity Pool**: Uniswap has a big pool of cryptocurrencies. For example, let's say it has a pool of ETH and USDT. The amount of each cryptocurrency in the pool determines their price.

2. **Constant Product Formula**: Uniswap uses a special formula to keep the pool balanced. This formula ensures that the product of the amounts of each cryptocurrency in the pool always stays the same. So, when you take out some ETH, you have to put in more USDT to keep the product constant, and vice versa.

3. **Swapping**: If you want to trade some ETH for USDT, you put your ETH into the pool, and the pool gives you back some USDT based on the formula. The more ETH you put in, the less USDT you get back, because the product stays constant.

4. **Price Impact**: When you make a trade and put lots of ETH into the pool or take lots out, the price of ETH changes. If you put in more ETH, the price of ETH goes up because there's less of it left in the pool. If you take out lots of ETH, the price goes down because there's more of it left in the pool.

## Usage

To use this API, you can start the server and interact with it using HTTP requests. Below are the available endpoints:

- `POST /buy-asset`: Endpoint to buy ETH by providing an amount of USDT.
- `POST /sell-asset`: Endpoint to sell ETH by providing an amount of ETH.
- `GET /get-balances`: Endpoint to get the current balances of ETH and USDT.



