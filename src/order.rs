use actix::prelude::*;
// use futures::{future, Future};
use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub type OrderExecutionAdapter = fn(Order) -> Transaction;

pub struct OrderExecutioner {
    pub subscribers: HashMap<usize, Recipient<Transaction>>,
    pub adapter: OrderExecutionAdapter,
}

impl Actor for OrderExecutioner {
    type Context = Context<Self>;
}

impl Handler<OrderMessage> for OrderExecutioner {
    type Result = Result<Transaction, ()>;

    fn handle(&mut self, msg: OrderMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let (_order, _recipient_id) = msg.into();
        unimplemented!();
    }
}

// TODO: Implement
#[derive(Message)]
pub struct Transaction;

struct OrderMessage {
    order: Order,
    id: usize,
}

impl Into<(Order, usize)> for OrderMessage {
    fn into(self) -> (Order, usize) {
        (self.order, self.id)
    }
}

// TODO: DeterDesign of TransactionMessage
impl Message for OrderMessage {
    type Result = Result<Transaction, ()>;
}

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

/// Contains price and timing variants that determine an
/// [`Order`]'s execution.
///
/// Variant definitions taken from [UBS's Best Execution of Equity Strategies
/// Paper](https://www.ubs.com/content/dam/static/wmamericas/bestexecution.pdf)
#[derive(Debug, PartialEq)]
pub enum OrderType {
    /// A market order is an order to buy or sell as soon as possible at the
    /// best price reasonably available.
    Market,

    /// A limit order is one where the client sets the maximum purchase price,
    /// or minimum sale price, at which the trade is to be executed.
    /// If the market moves away from this price, the order will not be executed
    /// unless or until the market price returns to the limit price.
    Limit(Limit),

    /// A stop limit order can only be executed at a specific price (or better)
    /// after a given stop price has been triggered. When the stop price is
    /// reached by the market, the stop limit order becomes a limit order to buy
    /// or sell at the limit price or better.
    StopLimit(Limit),

    /// A Day Order is an order that is valid until executed or the next market
    /// close.
    Day(Box<OrderType>),

    /// An order that exists until the order is completed or cancelled by the
    /// client.
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
