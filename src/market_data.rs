#[derive(serde::Deserialize)]
pub struct MarketData {
    symbol: String,
    price: f64,
    timestamp: u64,
}

impl MarketData {
    fn parse_json(s: &str) -> Result<MarketData, serde_json::Error> {
        serde_json::from_str(s)
    }

    fn parse_xml(s: &str) -> Result<MarketData, quick_xml::DeError> {
        quick_xml::de::from_str(s)
    }
}
