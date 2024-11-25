/* Financial quantifiable assets */

// Cash amount under a currency
pub struct CashAsset {
    currency: String, // e.g. USD,  EUR,  BTC,  ETH
}

// // Share amount of a company
// pub struct StockAsset {
//     symbol: String, // e.g., "AAPL" for Apple, "GOOG" for Google
// }

// // Loan amount to a bond issuer (e.g. company), expecting a returned interest rate
//
// use chrono::NaiveDateTime;
// pub struct BondAsset {
//     issuer: String, // Issuer of the bond, e.g., "Tesla" or "US Treasury"
//     currency: String,
//     face_value: f64,              // Nominal value of the bond
//     interest_rate: f64,           // The annual interest rate (coupon rate), e.g., 5%
//     maturity_date: NaiveDateTime, // The date the bond matures (e.g., "2025-12-31")
// }
