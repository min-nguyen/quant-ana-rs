use reqwest::get;
use serde_json::{from_str, Value};
use std::error::Error;

// ----------------------------------------
// ## Establishing a connection to a market data provider

// API key for authentication
const API_KEY: &str = "YOUR_API_KEY";
// Symbol for Apple Inc
const SYMBOL: &str = "AAPL";
// Base url of market provider
const BASE_URL: &str = "https://www.alphavantage.co/query";

#[tokio::test]
async fn req_market_data() -> Result<(), Box<dyn Error>> {
    // Request a one-minute interval intraday data for Apple Inc.
    let url: String = format!(
        "{}?function=TIME_SERIES_INTRADAY&symbol={}&interval=1min&apikey={}",
        BASE_URL, SYMBOL, API_KEY
    );

    let rsp_body: String = get(&url) // Encoded GET response
        .await?
        .text() // Decoded response body as a string
        .await?;

    // Response body as a parsed JSON value
    let data: Value = from_str(&rsp_body)?;
    println!("{}", data);
    Ok(())
}
