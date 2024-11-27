use chrono::{NaiveDateTime, Utc};

use crate::order::{Order, OrderErr, OrderSide};
use std::collections::BTreeMap;

/* ## Order Book
A list of active buy and sell orders for a particular company, trading pair, or commodity.
Each order book is usually dedicated to a specific trading pair or asset under a specific base currency:
   - Stocks: Each specific stock (e.g., AAPL) with a specific price asset (e.g. USD) has its own order book
   - Forex: Currency pairs (e.g., EUR/USD) have separate order books.
   - Commodities: Individual products (e.g. Gold Futures) with a specific price asset (e.g. USD) have their own books.
*/
pub struct OrderBook {
    // Ordered mapping of price-times to their active limit orders
    buy_orders: BTreeMap<OrderIndex, Order>,
    sell_orders: BTreeMap<OrderIndex, Order>,
    id_counter: u64,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
            id_counter: 0,
        }
    }

    // pub fn handle_order(&mut self, order: Order) -> Result<(), OrderErr> {
    //     order.validate()?;
    //     let buy_or_sell: OrderSide = order.side(); // copy
    //                                                // let insert_into = |m: &mut BTreeMap<_, Vec<_>>| {
    //                                                //     m.entry(order.price)
    //                                                //         .or_default() // empty vec
    //                                                //         .push(order)
    //                                                // };
    //                                                // match buy_or_sell {
    //                                                //     OrderSide::Buy => insert_into(&mut self.buy_orders),
    //                                                //     OrderSide::Sell => insert_into(&mut self.sell_orders),
    //                                                // }
    //     Ok(())
    // }

    pub fn insert_limit_order(&mut self, order: Order) {
        if let Order::LimitOrder {
            side, limit_price, ..
        } = order
        {
            let order_idx: OrderIndex =
                OrderIndex::new(limit_price, Utc::now().naive_local(), self.id_counter);
            self.id_counter += 1;

            match side {
                OrderSide::Buy => self.buy_orders.insert(order_idx, order),
                OrderSide::Sell => self.sell_orders.insert(order_idx, order),
            };
        }
    }

    pub fn best_buy_price(&self) -> Option<u64> {
        self.buy_orders.last_key_value().map(|kv| kv.0.price)
    }

    pub fn best_sell_price(&self) -> Option<u64> {
        self.sell_orders.first_key_value().map(|kv| kv.0.price)
    }

    pub fn market_price(&self, side: OrderSide) -> Option<u64> {
        match side {
            OrderSide::Buy => self.best_buy_price(),
            OrderSide::Sell => self.best_sell_price(),
        }
    }
}

/* ##  Order Index
Ordered by member-wise comparison in the order "price, timestamp, id" (can be automatically derived)
*/
#[derive(Debug, PartialEq, Eq)]
pub struct OrderIndex {
    pub price: u64,
    pub timestamp: NaiveDateTime,
    pub id: u64,
    /*
    pub order_asset: Asset,    // asset being bought or sold
    pub price_asset: Asset,    // asset used to pay for the order_asset
    */
}

impl OrderIndex {
    pub fn new(price: u64, timestamp: NaiveDateTime, id: u64) -> OrderIndex {
        OrderIndex {
            price,
            timestamp,
            id,
        }
    }
}

impl Ord for OrderIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.price == other.price {
            if self.timestamp == other.timestamp {
                return self.id.cmp(&other.id);
            }
            self.timestamp.cmp(&other.timestamp)
        } else {
            self.price.cmp(&other.price)
        }
    }
}

impl PartialOrd for OrderIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod test {
    use super::{
        Order, OrderBook,
        OrderSide::{Buy, Sell},
    };

    fn order_book() -> OrderBook {
        let mut book = OrderBook::new();

        let buy_order_1 = Order::LimitOrder {
            side: Buy,
            limit_price: 690,
            quantity: 35,
        };
        let buy_order_2 = Order::LimitOrder {
            side: Buy,
            limit_price: 685,
            quantity: 30,
        };
        let buy_order_3 = Order::LimitOrder {
            side: Buy,
            limit_price: 690,
            quantity: 15,
        };

        let sell_order_1 = Order::LimitOrder {
            side: Sell,
            limit_price: 700,
            quantity: 10,
        };
        let sell_order_2 = Order::LimitOrder {
            side: Sell,
            limit_price: 705,
            quantity: 25,
        };
        let sell_order_3 = Order::LimitOrder {
            side: Sell,
            limit_price: 700,
            quantity: 30,
        };
        book.insert_limit_order(buy_order_1);
        book.insert_limit_order(buy_order_2);
        book.insert_limit_order(buy_order_3);
        book.insert_limit_order(sell_order_1);
        book.insert_limit_order(sell_order_2);
        book.insert_limit_order(sell_order_3);
        book
    }

    #[test]
    fn test_insert_limit_order() {
        let book: OrderBook = order_book();
        assert_eq!(book.buy_orders.len(), 2);
        assert_eq!(book.sell_orders.len(), 2);
        assert_eq!(book.buy_orders.get(&(690)).unwrap().len(), 2);
        assert_eq!(book.buy_orders.get(&(685)).unwrap().len(), 1);
        assert_eq!(book.sell_orders.get(&(700)).unwrap().len(), 2);
        assert_eq!(book.sell_orders.get(&(705)).unwrap().len(), 1);
    }

    #[test]
    fn test_market_price() {
        let book: OrderBook = order_book();
        assert_eq!(book.best_buy_price().unwrap(), 690);
        assert_eq!(book.best_sell_price().unwrap(), 700);
    }
}
