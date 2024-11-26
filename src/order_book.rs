use crate::order::{Order, OrderSide};
use std::collections::BTreeMap;

/* ## Order Book
A list of active buy and sell orders for a particular company, trading pair, or commodity.
Each order book is usually dedicated to a specific trading pair or asset under a specific base currency:
   - Stocks: Each specific stock (e.g., AAPL) with a specific price asset (e.g. USD) has its own order book
   - Forex: Currency pairs (e.g., EUR/USD) have separate order books.
   - Commodities: Individual products (e.g. Gold Futures) with a specific price asset (e.g. USD) have their own books.
*/
pub struct OrderBook {
    // Mapping of ordered prices to lists of orders
    buy_orders: BTreeMap<u64, Vec<Order>>,
    sell_orders: BTreeMap<u64, Vec<Order>>,
}
impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: BTreeMap::new(),
            sell_orders: BTreeMap::new(),
        }
    }

    pub fn insert_order(&mut self, order: Order) {
        let buy_or_sell: OrderSide = order.side; // copy
        let insert_into = |m: &mut BTreeMap<_, Vec<_>>| {
            m.entry(order.price)
                .or_default() // empty vec
                .push(order)
        };
        match buy_or_sell {
            OrderSide::Buy => insert_into(&mut self.buy_orders),
            OrderSide::Sell => insert_into(&mut self.sell_orders),
        }
    }

    pub fn best_buy_price(&self) -> Option<u64> {
        self.buy_orders.last_key_value().map(|kv| *kv.0)
    }

    pub fn best_sell_price(&self) -> Option<u64> {
        self.sell_orders.first_key_value().map(|kv| *kv.0)
    }
}

mod test {
    use super::{
        Order, OrderBook,
        OrderSide::{Buy, Sell},
    };

    fn order_book() -> OrderBook {
        let mut book = OrderBook::new();

        let buy_order_1 = Order::new(Buy, 690, 35);
        let buy_order_2 = Order::new(Buy, 685, 20);
        let buy_order_3 = Order::new(Buy, 690, 15);

        let sell_order_1 = Order::new(Sell, 700, 10);
        let sell_order_2 = Order::new(Sell, 705, 25);
        let sell_order_3 = Order::new(Sell, 700, 30);

        book.insert_order(buy_order_1);
        book.insert_order(buy_order_2);
        book.insert_order(buy_order_3);
        book.insert_order(sell_order_1);
        book.insert_order(sell_order_2);
        book.insert_order(sell_order_3);
        book
    }

    #[test]
    fn test_insert_order() {
        let book: OrderBook = order_book();
        assert_eq!(book.buy_orders.len(), 2);
        assert_eq!(book.sell_orders.len(), 2);
        assert_eq!(book.buy_orders.get(&(690)).unwrap().len(), 2);
        assert_eq!(book.buy_orders.get(&(685)).unwrap().len(), 1);
        assert_eq!(book.sell_orders.get(&(700)).unwrap().len(), 2);
        assert_eq!(book.sell_orders.get(&(705)).unwrap().len(), 1);
    }

    #[test]
    fn test_best_price_() {
        let book: OrderBook = order_book();
        assert_eq!(book.best_buy_price().unwrap(), 690);
        assert_eq!(book.best_sell_price().unwrap(), 700);
    }
}
