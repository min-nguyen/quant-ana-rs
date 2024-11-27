use crate::order::Order;
use chrono::{NaiveDateTime, Utc};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct OrderQueue(BTreeMap<OrderIndex, Order>);

impl OrderQueue {
    pub fn new() -> Self {
        OrderQueue(BTreeMap::new())
    }

    pub fn insert(&mut self, order_id: u64, order: Order) -> Result<(), OrderQueueError> {
        if (self.get_by_order_id(order_id)).is_some() {
            return Err(OrderQueueError::OrderIdExists);
        }
        match order {
            Order::LimitOrder { limit_price, .. } => {
                let ordex_idx: OrderIndex =
                    OrderIndex::new(limit_price, Utc::now().naive_utc(), order_id);
                self.0.insert(ordex_idx, order);
                Ok(())
            }
            _ => Err(OrderQueueError::NotALimitOrder),
        }
    }

    pub fn get_by_order_id(&self, order_id: u64) -> Option<(&OrderIndex, &Order)> {
        self.0.iter().find_map(|kv: (&OrderIndex, &Order)| {
            if kv.0.order_id == order_id {
                Some(kv)
            } else {
                None
            }
        })
    }

    pub fn get_by_limit_price(&self, limit_price: u64) -> Vec<(&OrderIndex, &Order)> {
        self.0
            .iter()
            .filter(|kv: &(&OrderIndex, &Order)| kv.0.limit_price == limit_price)
            .collect()
    }
}

impl Deref for OrderQueue {
    type Target = BTreeMap<OrderIndex, Order>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for OrderQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/* ##  Order Index
Ordered by member-wise comparison in the order "price, timestamp, id" (can be automatically derived)
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderIndex {
    pub limit_price: u64,
    pub timestamp: NaiveDateTime,
    pub order_id: u64,
    /*
    pub order_asset: Asset,    // asset being bought or sold
    pub price_asset: Asset,    // asset used to pay for the order_asset
    */
}

impl OrderIndex {
    pub fn new(limit_price: u64, timestamp: NaiveDateTime, order_id: u64) -> OrderIndex {
        OrderIndex {
            limit_price,
            timestamp,
            order_id,
        }
    }
}

impl Ord for OrderIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.limit_price == other.limit_price {
            if self.timestamp == other.timestamp {
                return self.order_id.cmp(&other.order_id);
            }
            self.timestamp.cmp(&other.timestamp)
        } else {
            self.limit_price.cmp(&other.limit_price)
        }
    }
}

impl PartialOrd for OrderIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/* ##  Order Index
Ordered by member-wise comparison in the order "price, timestamp, id" (can be automatically derived)
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OrderQueueError {
    OrderIdExists,
    NotALimitOrder,
}
