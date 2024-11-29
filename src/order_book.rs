use crate::order::{Order, OrderErr, OrderSide};
use crate::order_queue::{OrderIndex, OrderQueue, OrderQueueError};

/* ## Order Book
Queues of available buy and sell orders for a particular company, trading pair, or commodity.
*/
pub struct OrderBook {
    buy_orders: OrderQueue,
    sell_orders: OrderQueue,
    id_counter: u64,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            buy_orders: OrderQueue::new(),
            sell_orders: OrderQueue::new(),
            id_counter: 0,
        }
    }

    // pub fn handle_order(&mut self, order: Order) -> Result<(), OrderErr> {
    //     order.validate()?;
    //     let buy_or_sell: OrderSide = order.side();
    //     Ok(())
    // }

    fn insert_limit_order(&mut self, order: Order) {
        let res: Result<(), OrderQueueError> = match order.side() {
            OrderSide::Buy => self.buy_orders.insert(self.id_counter, order),
            OrderSide::Sell => self.sell_orders.insert(self.id_counter, order),
        };
        if res.is_ok() {
            self.id_counter += 1;
        } else {
            println!("Insertion error: {:?}", res)
        }
    }

    pub fn best_buy_price(&self) -> Option<u64> {
        self.buy_orders
            .last_key_value()
            .map(|kv: (&OrderIndex, &Order)| kv.0.limit_price)
    }

    pub fn best_sell_price(&self) -> Option<u64> {
        self.sell_orders
            .first_key_value()
            .map(|kv: (&OrderIndex, &Order)| kv.0.limit_price)
    }

    pub fn market_price(&self, side: OrderSide) -> Option<u64> {
        match side {
            OrderSide::Buy => self.best_buy_price(),
            OrderSide::Sell => self.best_sell_price(),
        }
    }
}

mod test {
    use super::{
        Order, OrderBook,
        OrderSide::{Buy, Sell},
    };

    fn order_book() -> OrderBook {
        let mut book: OrderBook = OrderBook::new();
        let limit_orders: Vec<Order> = vec![
            Order::LimitOrder {
                side: Buy,
                limit_price: 690,
                quantity: 35,
            },
            Order::LimitOrder {
                side: Buy,
                limit_price: 685,
                quantity: 30,
            },
            Order::LimitOrder {
                side: Buy,
                limit_price: 690,
                quantity: 15,
            },
            Order::LimitOrder {
                side: Sell,
                limit_price: 700,
                quantity: 10,
            },
            Order::LimitOrder {
                side: Sell,
                limit_price: 705,
                quantity: 25,
            },
            Order::LimitOrder {
                side: Sell,
                limit_price: 700,
                quantity: 30,
            },
        ];
        6;
        limit_orders
            .into_iter()
            .for_each(|order| book.insert_limit_order(order));
        book
    }

    #[test]
    fn test_insert_limit_order() {
        let book: OrderBook = order_book();
        assert_eq!(book.buy_orders.len(), 3);
        assert_eq!(book.sell_orders.len(), 3);
        assert_eq!(book.buy_orders.get_by_limit_price(690).len(), 2);
        assert_eq!(book.buy_orders.get_by_limit_price(685).len(), 1);
        assert_eq!(book.sell_orders.get_by_limit_price(700).len(), 2);
        assert_eq!(book.sell_orders.get_by_limit_price(705).len(), 1);
    }

    #[test]
    fn test_market_price() {
        let book: OrderBook = order_book();
        assert_eq!(book.best_buy_price().unwrap(), 690);
        assert_eq!(book.best_sell_price().unwrap(), 700);
    }

    // #[test]
    // fn test_handle_order() {
    //     let book: OrderBook = order_book();
    // }
}
