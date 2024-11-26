
# Financial Assets

A broad category that includes any non-physical asset with monetary value.
These are typically traded or held for investment, financing, or hedging purposes.

## Categories of Financial Assets

### 1. **Equity Securities**
- Represent **ownership** in an entity.
- Key Features:
  - Shareholders may receive dividends and have voting rights.
  - Value depends on the issuing company’s performance and market conditions.
- Examples:
  - **Common Stocks**: Provide ownership and voting rights.
  - **Preferred Stocks**: Offer fixed dividends with no voting rights.

---

### 2. **Debt Securities**
- Represent a **loan relationship** where the issuer borrows funds from the investor.
- Key Features:
  - Regular interest payments (coupons).
  - Repayment of principal at maturity.
- Examples:
  - **Bonds**: Secured or unsecured long-term debt.
  - **Treasury Bills**: Short-term government debt.
  - **Debentures**: Unsecured debt without collateral.

---

### 4. **Hybrid Securities**
- Combine characteristics of both equity and debt securities.
- Key Features:
  - Offer flexibility in risk and return.
  - Can be converted or prioritized depending on terms.
- Examples:
  - **Convertible Bonds**: Bonds that can be converted into equity shares.
  - **Preferred Shares**: Hybrid securities offering fixed dividends and seniority over common stock in liquidation.

---

### 3. **Derivatives**
- Financial contracts whose value is derived from an underlying asset or benchmark.
- Key Features:
  - Used for hedging or speculation.
  - Value fluctuates with the underlying asset (e.g., stocks, commodities).
- Examples:
  - **Options**: Right to buy/sell an asset at a specified price within a timeframe.
  - **Futures**: Obligation to buy/sell an asset at a predetermined date and price.
  - **Swaps**: Exchange of cash flows between two parties (e.g., interest rate swaps).

---

### 5. **Cash and Cash Equivalents**
- Highly liquid financial assets used for immediate transactions or short-term investments.
- Key Features:
  - Low risk and low return.
  - Easily converted into cash.
- Examples:
  - **Cash Deposits**: Bank accounts with on-demand withdrawals.
  - **Money Market Funds**: Short-term, low-risk investment funds.
  - **Treasury Bills**: Short-term government-issued securities.

```
Financial Assets
├── Equity Securities
│   ├── Stocks
│   ├── ETFs (Exchange-Traded Funds)
│   └── REITs (Real Estate Investment Trusts)
│
├── Debt Securities
│   ├── Bonds (Government, Corporate, Municipal)
│   ├── Money Market Instruments (T-Bills, Commercial Paper)
│   └── Structured Debt (MBS, ABS)
│
├── Derivatives
│   ├── Options (Call, Put)
│   ├── Futures
│   ├── Swaps
│   └── Forwards
│
├── Commodities
│   ├── Energy (Oil, Gas)
│   ├── Metals (Gold, Silver)
│   ├── Agriculture (Grains, Softs)
│   └── Livestock
│
├── Currencies
│   ├── Major Pairs (e.g., EUR/USD)
│   ├── Minor Pairs (e.g., GBP/AUD)
│   └── Exotic Pairs (e.g., USD/TRY)
│
├── Cryptocurrencies
│   ├── Coins (BTC, ETH)
│   └── Tokens (Utility, Security, Stablecoins)
│
└── Alternative Investments
    ├── Private Equity
    ├── Hedge Funds
    ├── Venture Capital
    └── Real Estate
```

------------------------------------------------------------------------------

## General
- **Market Order**: Buy/sell immediately at the best available price.
- **Limit Order**: Buy/sell at a specific price or better.
- **Stop-Loss**: An order to sell when a price hits a set level, limiting losses.
- **Take-Profit**: An order to close a position once a target profit level is reached.

## Market Dynamics
- **Market price**: The current price at which a financial asset can be bought or sold in the marketplace.
   It is determined by either:
    1. The most recent transaction.
       Typically considered the "official" market price for reporting and analytics.
    2. Best Available Price in the Order Book.
       Relevant when executing *Market Orders* immediately.
        The Best Ask Price: The lowest price at which a seller is willing to sell.
        The Best Bid Price: The highest price a buyer is willing to pay.
    The context determines which definition of market price is more relevant.
- **Bid-Ask Spread**: The difference between the highest bid and lowest ask price.
- **Liquidity**: How easily an asset can be bought/sold without affecting price.
- **Volume**: The number of shares/contracts traded in a period.

## Technical Analysis
- **Support**: A price level where buying interest is strong, preventing further decline.
- **Resistance**: A price level where selling interest is strong, preventing further rise.
- **Trend**: The general direction (upward, downward, or sideways) of an asset’s price.
- **Volatility**: The measure of price fluctuations over time.

## Risk & Leverage
- **Leverage**: Borrowed funds to amplify trading positions. Increases both potential profits and losses.
- **Margin**: The amount of capital required to open a leveraged position.
- **Risk-Reward Ratio**: Ratio of potential profit to potential loss.

## Order Types
- **Market Depth**: The number of buy and sell orders at different prices.
- **Stop Order**: Executes a trade when a specific price is reached.
- **Trailing Stop**: A dynamic stop-loss that adjusts with price movement.

## Asset Types
- **Equities**: Stocks or shares of companies.
- **Forex**: Currency trading (e.g., EUR/USD).
- **Derivatives**: Contracts based on underlying assets (e.g., options, futures).
- **Cryptocurrencies**: Digital currencies like Bitcoin or Ethereum.

## Market Events
- **Bull Market**: Rising prices, optimism.
- **Bear Market**: Falling prices, pessimism.
- **Correction**: A temporary price decline, typically 10% or more from a peak.

## Metrics
- **P/E Ratio**: Price-to-Earnings ratio; evaluates if a stock is over or undervalued.
- **EPS**: Earnings Per Share; a company's profitability per share.
- **ROI**: Return on Investment; a measure of profitability.

## Strategies
- **Scalping**: Quick, small trades to capture small price movements.
- **Swing Trading**: Holding positions for days or weeks to profit from price trends.
- **Day Trading**: Buying and selling within the same trading day.
- **Hedging**: Reducing risk by taking an offsetting position.

---

# Order Book
A record of all **buy (bid)** and **sell (ask)** orders for a specific asset at various price levels.

## Key Components
1. **Bids**: Orders to buy at specific prices.
   - Example: "Buy 10 shares at $50"
2. **Asks**: Orders to sell at specific prices.
   - Example: "Sell 10 shares at $52"
3. **Depth**: The volume of bids and asks available at each price level.

## Visual Example
| Price | Bids (Buy Volume) | Asks (Sell Volume) |
|-------|-------------------|--------------------|
| $50   | 100 shares        | -                  |
| $51   | 200 shares        | -                  |
| $52   | -                 | 150 shares         |
| $53   | -                 | 250 shares         |