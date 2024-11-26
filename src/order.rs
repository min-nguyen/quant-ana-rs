use std::{collections::BTreeMap, fmt::Debug};

use chrono::{NaiveDateTime, Utc};

/* ## Order Engine
The order matching engine is the processing unit responsible for executing trades based on the orders in the order book.
    - Order Matching: Finds and executes matches between bids and asks.
    - Order Management: Adds, modifies, or cancels orders in the order book.
    - Price Discovery: Helps determine the market price by executing trades at the best available prices.
*/

#[derive(Debug, Copy, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

/* ##  Orders
An order to be matched as the result of a successfully processed order request */
#[derive(Debug)]
pub struct Order {
    pub side: OrderSide, // buy or sell
    pub price: u64,      // amount of price_asset to pay for the quantity of order_asset
    pub quantity: u64,   // quantity of order_asset being bought/sold
    pub timestamp: NaiveDateTime, // time of order process
                         /*
                          pub id: u64,
                         pub order_asset: Asset,    // asset being bought or sold
                         pub price_asset: Asset,    // asset used to pay for the order_asset
                         */
}

impl Order {
    pub fn new(side: OrderSide, price: u64, quantity: u64) -> Order {
        Order {
            side,
            price,
            quantity,
            timestamp: Utc::now().naive_utc(),
        }
    }
}

/* ## Order Requests
A request to Buy or Sell at a particular price and quantity */
trait OrderRequest {}

/* ### Market Orders
Instructs a system to buy or sell an asset immediately at current market price. */
#[derive(Debug)]
struct MarketOrder {
    side: OrderSide,
    quantity: u64,
    // order_asset: Asset,
    // price_asset: Asset,
}
impl OrderRequest for MarketOrder {}
