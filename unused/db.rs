// use crate::schema::trades;
// use bigdecimal::BigDecimal;
// use chrono::naive::NaiveDateTime;
// use diesel::prelude::*;

// /*
//     Trade represents a trade entry with the following fields:
//     - `symbol`: A string slice referencing the trade symbol (e.g., "AAPL").
//     - `price`: The trade price, represented as a `BigDecimal` for high-precision decimal calculations.
//     - `volume`: The volume of shares traded, represented as a 32-bit integer.
//     - `trade_time`: The timestamp of the trade, represented using `NaiveDateTime` from the `chrono` crate.
// */
// #[derive(Queryable)]
// struct Trade {
//     id: i32,
//     symbol: String,
//     price: BigDecimal,
//     volume: i32,
//     trade_time: NaiveDateTime,
// }

// #[derive(Insertable)]
// #[table_name = "trades"]
// struct NewTrade {
//     symbol: String,
//     price: BigDecimal,
//     volume: i32,
//     trade_time: NaiveDateTime,
// }

// // fn main() {
// //     dotenv();
// // }
