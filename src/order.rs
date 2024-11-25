use std::fmt::Debug;

/* ##  Orders */
#[derive(Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

// An order to be handled as the result of a successfully processed order request
#[derive(Debug)]
pub struct Order<Asset>
where
    Asset: Debug + Clone,
{
    pub id: u64,
    pub order_type: OrderType, // buy or sell
    pub order_asset: Asset,    // asset being bought or sold
    pub order_quantity: f64,   // quantity of order_asset being bought/sold
    pub price_asset: Asset,    // asset used to pay for the order_asset
    pub price: f64,            // amount of price_asset to pay for the quantity of order_asset
}

/* ## Order Requests */
trait OrderRequest {}

/* ### Market Orders
Instructs a system to buy or sell an asset immediately at current market price. */
#[derive(Debug)]
struct MarketOrder<Asset> {
    order_asset: Asset,
    price_asset: Asset,
    side: OrderType,
    quantity: f64,
}
impl<Asset> OrderRequest for MarketOrder<Asset> {}
