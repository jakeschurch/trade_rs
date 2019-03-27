use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub enum Order {
    Buy {
        ticker: String,
        amount: f32,
        order_type: OrderType,
    },
    Sell {
        ticker: String,
        amount: f32,
        order_type: OrderType,
    },
}

/// [`OrderType`] contains price and timing variants that determine an [`Order`]'s execution.
///
/// Variant definitions taken from [UBS's Best Execution of Equity Strategies
/// Paper](https://www.ubs.com/content/dam/static/wmamericas/bestexecution.pdf)
#[derive(Debug, PartialEq)]
pub enum OrderType {
    /// A market order is an order to buy or sell as soon as possible at the best price reasonably available.
    Market,

    /// A limit order is one where the client sets the maximum purchase price, or minimum sale price, at which the trade is to be executed.
    /// If the market moves away from this price, the order will not be executed unless or until the market price returns to the limit price.
    Limit(Limit),

    /// A stop limit order can only be executed at a specific price (or better) after a given stop price has been triggered.
    /// When the stop price is reached by the market, the stop limit order becomes a limit order to buy or sell at the limit price or better.
    StopLimit(Limit),

    /// A Day Order is an order that is valid until executed or the next market close.
    Day(Box<OrderType>),

    /// An order that exists until the order is completed or cancelled by the client.
    GoodTillCancelled(Box<OrderType>, DateTime<Utc>),
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

#[derive(Debug, PartialEq)]
pub enum Limit {
    Max(f64),
    Min(f64),
}
