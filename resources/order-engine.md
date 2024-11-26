# Order Management System (OMS)
Responsible for managing and tracking orders from inception to execution.
It supports order placement, modification, cancellation, routing, and execution.

Key components:
1. Order Request Entry and Validation
    The interface via which orders are created that ensures incoming orders conform to the required format and rules.

    An order request at least consists of:
    - Order side: Buy or Sell
    - Asset e.g. a stock name (AAPL)
    - Quantity
    - Price
    And also can consist of:
    - Product type: Intraday or Delivery
    - Order type:
        1. Market Order: Buy/sell immediately at the current market price
        2. Limit Order: Buy when price falls to _, or Sell when price rises to _.


2. Order Routing
    Once validated, orders are routed to the appropriate order execution venue i.e. exchange.
    An exchange maintains multiple order books, one for each financial instrument or trading pair it supports.

    For Example:
    - Stock Exchange Order Books:
        ├── AAPL (Apple Inc.)
        ├── TSLA (Tesla Inc.)
        ├── MSFT (Microsoft Corporation)
        └── GOOGL (Alphabet Inc.)
    - Forex Exchange Order Books:
        ├── EUR/USD
        ├── GBP/JPY
        ├── AUD/CAD
        └── USD/CHF

3. Order Book Management:
    A list of active buy and sell orders for a particular company, trading pair, or commodity.
    Each order book is usually dedicated to a specific trading pair or financial asset under a specific base currency.

    For example:
    - Stocks: Each specific stock (e.g., AAPL, TSLA) with a specific price asset (e.g. USD) has its own order book
    - Forex: Currency pairs (e.g., EUR/USD, GBP/JPY) have separate order books.
    - Commodities: Individual products (e.g., Crude Oil Futures, Gold Futures) with a specific price asset (e.g. USD) have their own books.

4. Order Engine Matching
    The order matching engine is the processing unit responsible for executing trades based on the orders in the order book.
    There is either one engine per order book, or one engine that may perform matches with multiple order books.
    It pairs buy and sell orders based on  matching rules specific to an exchange.
    - Order Matching: Finds and executes matches between bids and asks.
    - Order Management: Adds, modifies, or cancels orders in the order book.
    - Price Discovery: Helps determine the market price by executing trades at the best available prices.

5. Execution Management
    After matching, the OMS executes orders, confirming trades with counterparties, updating the order status, and capturing execution details.

-----

# Order

-----

# Order Request