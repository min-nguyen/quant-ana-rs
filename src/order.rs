/*
An Order Management System (OMS) is responsible for streamlining the order process by managing and tracking orders from inception to execution.
It supports order placement, modification, cancellation, routing, and execution.

Key components:
1. Order Entry
    The interface via which trades are created, and ensures incoming orders conform to the required format.
2. Order validation
    A series of validation checks performed before any order is accepted for processing
3. Order Routing
    Once validated, orders are routed to the appropriate order execution venue, such as an exchange i.e.  a centralized platform for trading.
4. Order Matching
    A matching engine pairs buy-and-sell orders based on price-time priority or other matching rules specific to an exchange.
5. Execution Management
    After matching, the OMS executes orders, confirming trades with counterparties, updating the order status, and capturing execution details.
6. Order Book Management:
    An internal order book is maintained to track the state of all active orders.
7. Order Modification
    Traders or algorithms may adjust or cancel orders, and the OMS must accommodate such requests.
*/
use std::fmt::Debug;

/* ## Types of Orders
*/
#[derive(Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}
#[derive(Debug)]
pub struct Order<Asset>
where
    Asset: Debug + Clone,
{
    pub id: u64,
    pub side: OrderSide,    // buy or sell
    pub order_asset: Asset, // asset being bought or sold in an ordered
    pub price_asset: Asset, // asset used to determine the price of the order_asset
    pub quantity: f64,      // quantity of order asset being bought/sold
    pub price: f64,         // price_asset per unit of order_asset
}

/* ## Types of Order Requests */
/* ### Market Orders
The most straightforward order type. It instructs a system to buy or sell a security immediately at current market price. */
#[derive(Debug)]
struct MarketOrder<Asset> {
    order_asset: Asset,
    price_asset: Asset,
    side: OrderSide,
    quantity: f64,
}
