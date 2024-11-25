# Order Management System (OMS)
Responsible for managing and tracking orders from inception to execution.
It supports order placement, modification, cancellation, routing, and execution.

Key components:
1. Order Entry
    The interface via which trades are created, and ensures incoming orders conform to the required format.
2. Order validation
    A series of validation checks performed before any order is accepted for processing
3. Order Routing
    Once validated, orders are routed to the appropriate order execution venue, such as an exchange i.e.  a centralized platform for trading.
4. Order Matching
    A matching engine pairs buy and sell orders based on  matching rules specific to an exchange.
5. Execution Management
    After matching, the OMS executes orders, confirming trades with counterparties, updating the order status, and capturing execution details.
6. Order Book Management:
    An internal order book is maintained to track the state of all active orders.
7. Order Modification
    Traders or algorithms may adjust or cancel orders, and the OMS must accommodate such requests.

-----

# Order

-----

# Order Request