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
    - Request type:
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

Matching Algorithm:

Receive an Incoming Order Request:
    When a new order (request) is submitted, it is attempted to be  matched and
    executed immediately before being inserted into the order book.

Receiving an Existing Order Update:
    When an existing order in the book is updated, its timestamp is changed,
    it is attempted to be matched and executed immediately like a new incoming order request.

First Steps:
    Determine if the order is a market order or a limit order.
    Determine if the order is to buy or sell order

Check for Matches:
    Incoming Market Orders:
        Always processed immediately.
        They match with the best available price on the opposite side of the book:
            A buy order is matched at the lowest ask
            A sell order is matched with the highest buy
    (In implementation, the custom `price` field of a Market Order is irrelevant.)

    Incoming Limit Orders:
        The order is compared with the best prices on the opposite side of the book:
            A buy order is checked against the lowest ask;
                 if the ask is <= the buy limit price, the order is matched.
            A sell order is checked against the highest bid.
                 if the bid is >= the sell limit price, the order is matched

Execute Match
    If a match is possible, the trade is executed immediately either fully or partially.
        - A fully executed trade means both ends are left with a buy and sell quantity of 0, and the existing order in the order book is removed.
        - A partially executed trade means either:
            1. The incoming order is left with a buy/sell quantity > 0,
               The matched order is left with a buy/sell quantity == 0, and removed from the book
               The incoming order is then re-checked for matches against the next best remaining orders in the book; this behavior effectively changes the market price, as each successive match happens at the next-best available price in the order book.
            2. The incoming order is left with a buy/sell quantity == 0, and not inserted into the book.
               The matched order is left with a buy/sell quantity > 0.
               The matched order is then readded into the order book.
    If no match is possible, in the case of either:
        1. a limit order that cannot be satisfied yet
        or/and
        2. no orders on the opposite side of the book,
    The incoming order is added to the book.

Orders are added and organised in the book by
    1. Price Priority: Higher buy prices and lower sell prices are given priority.
    2. Time Priority: For orders with the same price, earlier orders are processed first.
    Hence when matching orders, the best price on the opposite side of the book is selected,
    and within that price, the oldest order is matched first.