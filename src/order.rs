use std::fmt::Debug;

/* ## Order Requests
A request to Buy or Sell at a particular price and quantity */
#[derive(Debug, Copy, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone)]
pub enum Order {
    /* Instruct a system to buy/sell when opposite order is below/above threshold . */
    LimitOrder {
        side: OrderSide,
        quantity: u64,
        limit_price: u64, // order_asset: Asset, price_asset: Asset,
    },
    /* Instruct a system to buy/sell immediately at current market price. */
    MarketOrder {
        side: OrderSide,
        quantity: u64, // order_asset: Asset, price_asset: Asset,
    },
}

impl Order {
    pub fn side(&self) -> OrderSide {
        match self {
            Self::LimitOrder { side, .. } | Self::MarketOrder { side, .. } => *side,
        }
    }
    pub fn validate(&self) -> Result<(), OrderErr> {
        match self {
            Order::LimitOrder {
                quantity,
                limit_price,
                ..
            } => {
                if *quantity <= 0 {
                    return Err(OrderErr::InvalidQuantity);
                }
                if *limit_price <= 0 {
                    return Err(OrderErr::InvalidPrice);
                }
                return Ok(());
            }
            Order::MarketOrder { quantity, .. } => {
                if *quantity <= 0 {
                    return Err(OrderErr::InvalidQuantity);
                }
                return Ok(());
            }
        }
    }
}

pub enum OrderErr {
    InvalidQuantity,
    InvalidPrice,
}
