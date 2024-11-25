use reqwest::get;
use serde_json::{from_str, Value};
use std::error::Error;
use tokio::net::TcpStream;

// ----------------------------------------
// ## Establishing a connection to a market data provider

// API key for authentication
const API_KEY: &str = "YOUR_API_KEY";
// Symbol for Apple Inc
const SYMBOL: &str = "AAPL";
// Base url of market provider
const BASE_URL: &str = "https://www.alphavantage.co/query";

#[tokio::test]
async fn http_get_market_data() -> Result<(), Box<dyn Error>> {
    // Request a one-minute interval intraday data for Apple Inc.
    let url: String = format!(
        "{}?function=TIME_SERIES_INTRADAY&symbol={}&interval=1min&apikey={}",
        BASE_URL, SYMBOL, API_KEY
    );

    let rsp_body: String = get(&url) // Encoded GET response
        .await?
        .text() // Decoded response body as a string
        .await?;

    /*
    JSON structure from Alpha Vantage API:
    - "Meta Data": Contains metadata about the stock data (symbol, interval, timestamp).
    - "Time Series (1min)": Contains minute-by-minute stock prices and volume data.
      Each timestamp includes:
        - "1. open": Opening price
        - "2. high": Highest price
        - "3. low": Lowest price
        - "4. close": Closing price
        - "5. volume": Number of shares traded
    */
    let data: Value = from_str(&rsp_body)?;
    println!("{}", data);
    Ok(())
}

// Connecting to data sources
async fn connect_tcp(addr: &str) -> TcpStream {
    TcpStream::connect(addr).await.unwrap()
}

#[derive(serde::Deserialize)]
pub struct MarketData {
    symbol: String,
    price: f64,
    timestamp: u64,
}

impl MarketData {
    fn parse_json(data: &str) -> Result<MarketData, serde_json::Error> {
        serde_json::from_str(data)
    }

    fn parse_xml(data: &str) -> Result<MarketData, quick_xml::DeError> {
        quick_xml::de::from_str(data)
    }

    fn parse_csv(path: &str) -> Result<Vec<MarketData>, Box<dyn std::error::Error>> {
        let mut csv_reader = csv::Reader::from_path(path)?;
        let mut records = vec![];
        for res in csv_reader.deserialize() {
            let rec: MarketData = res?;
            records.push(rec);
        }
        Ok(records)
    }

    fn filter<F>(&self, p: F) -> Option<&Self>
    where
        F: Fn(&Self) -> bool,
    {
        if p(self) {
            Some(self)
        } else {
            None
        }
    }
}

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
