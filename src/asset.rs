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

pub struct BondAsset {
    issuer: String,
}

pub enum CashAsset {
    USD,
    EUR,
    BTC,
    ETH,
}
