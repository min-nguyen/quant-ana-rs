use crate::market_data::MarketData;
use tokio::net::TcpStream;

// Connecting to data sources
async fn connect_tcp(addr: &str) -> TcpStream {
    TcpStream::connect(addr).await.unwrap()
}
// Handling different data formats
fn str_to_json(data: &str) -> serde_json::Value {
    serde_json::from_str(data).unwrap()
}
// Filtering data
fn filter_by_price(data: &MarketData, threshold: f64) {}
