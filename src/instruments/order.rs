use chrono::{DateTime, Utc};
use instruments::currency::Currency;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
// REVIEW: add volume_filled field?
pub struct Order {
    // REVIEW: id field?
    pub symbol: String,
    pub price: Currency,
    pub volume: u32,
    pub side: Side,
    pub datetime: DateTime<Utc>,
    pub state: State,
}

impl Order {
    pub fn new(
        symbol: String,
        price: f32,
        volume: u32,
        side: Side,
        datetime: DateTime<Utc>,
    ) -> Self {
        Order {
            symbol,
            price: Currency::from(price),
            volume,
            side,
            datetime,
            state: State::New,
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Symbol:\t{}
            Price:\t{}
            Volume:\t{}
            Type of Order:\t{}
            Date:\t{}",
            self.symbol, self.price, self.volume, self.side, self.datetime
        )
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
/// State indicates the state machine of an order.
pub enum State {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Suspended,
    Expired,
    Rejected,
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
/// Side indicates an order's intention to either buy or sell shares.  
pub enum Side {
    Buy,
    Sell,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Side::*;
        match &self {
            Buy => write!(f, "Buy"),
            Sell => write!(f, "Sell"),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
/// Logic indicates an order's execution logic.
/// NOTE(Logic): Currently only allowing market logic orders.
pub enum Logic {
    Market,
}
