// pub enum AssetType {
//     Equity,
//     Debt,
//     Derivative,
//     Commodity,
//     Currency,
// }

pub struct StockAsset {
    symbol: String, //  ticker symbol for the stock (e.g., "AAPL" for Apple, "GOOG" for Google).
    company: String,
}

pub enum CurrencyAsset {
    USD,
    EUR,
    BTC,
    ETH,
}
