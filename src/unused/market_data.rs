use reqwest::get;
use serde_json::{from_str, Value};
use std::{
    error::Error,
    fmt::{Display, Formatter, Write},
};
use tokio::net::TcpStream;

// ----------------------------------------
// ## Establishing a connection to a market data provider
// https://www.alphavantage.co/documentation/
// Base url of market provider
const BASE_URL: &str = "https://www.alphavantage.co/query";
// API key for authentication
const API_KEY: &str = "YOUR_API_KEY";
// Symbol for Apple Inc
const AAPL: &str = "AAPL";

pub enum IntradayInterval {
    _1MIN,
    _5MIN,
    _15MIN,
    _30MIN,
    _60MIN,
}
impl Display for IntradayInterval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IntradayInterval::_1MIN => f.write_str("1min"),
            IntradayInterval::_5MIN => f.write_str("5min"),
            IntradayInterval::_15MIN => f.write_str("15min"),
            IntradayInterval::_30MIN => f.write_str("30min"),
            IntradayInterval::_60MIN => f.write_str("60min"),
        }
    }
}

/* Time_series_intraday JSON:
 {"Meta Data": "...."     // Metadata about the stock data (symbol, interval used, timezone).
  "Time Series (interval)":
        {   "timestamp", {
                "1. open": "....",   // Opening price
                "2. high": "....",   // Highest price,
                "3. low": "....",   // Lowest price,
                "4. close": "...."  // Closing price,
                "5. volume": "...." // Number of shares traded },
            "timestamp", {
                ...
            },
            ..
        }
 }
*/
async fn time_series_intraday(
    symbol: &str,
    interval: IntradayInterval,
) -> Result<(), Box<dyn Error>> {
    // Request a one-minute interval intraday data for Apple Inc.
    let url: String = format!(
        "{}?function=TIME_SERIES_INTRADAY&symbol={}&interval={}&apikey={}",
        BASE_URL, symbol, interval, API_KEY
    );

    let rsp_body: String = get(&url) // Encoded GET response
        .await?
        .text() // Decoded response body as a string
        .await?;

    let data: Value = from_str(&rsp_body)?;
    println!("{}", data);
    Ok(())
}

#[tokio::test]
async fn test_appl() -> Result<(), Box<dyn Error>> {
    time_series_intraday(AAPL, IntradayInterval::_1MIN).await
}

/* ----------------------------------------------------------- */

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
