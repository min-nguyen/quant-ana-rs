-- Up migration
CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    volume INT NOT NULL,
    trade_time TIMESTAMP NOT NULL
)