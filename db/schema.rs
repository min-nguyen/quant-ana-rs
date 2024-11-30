// @generated automatically by Diesel CLI.

diesel::table! {
    trades (id) {
        id -> Int4,
        symbol -> Varchar,
        price -> Numeric,
        volume -> Int4,
        trade_time -> Timestamp,
    }
}
