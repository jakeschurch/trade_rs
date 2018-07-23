pub use self::currency::Currency;
pub use self::order::{Order, Side, State};

mod currency;
mod order;

use chrono::{DateTime, Utc};
use std::option;

type Option<T> = option::Option<T>;

pub trait Instrument<Output = Self>: Sized {
    fn buy(order: Order) -> Self;
    fn sell(&mut self, &Order, u32) -> Self;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Holding {
    pub symbol: String,
    pub volume: u32,
    pub buy_price: Currency,
    pub buy_date: DateTime<Utc>,
    pub sell_price: Option<Currency>,
    pub sell_date: Option<DateTime<Utc>>,
}

impl Holding {
    pub fn new(order: Order) -> Holding {
        Holding {
            symbol: order.symbol.to_string(),
            volume: order.volume,
            buy_price: order.price,
            buy_date: order.datetime,
            sell_price: None,
            sell_date: None,
        }
    }
}
impl Instrument for Holding {
    fn buy(order: Order) -> Self {
        Holding::new(order)
    }
    fn sell(&mut self, order: &Order, vol_to_sell: u32) -> Holding {
        self.volume -= vol_to_sell;

        Holding {
            volume: vol_to_sell,
            sell_price: Some(order.price.clone()),
            sell_date: Some(order.datetime),
            ..self.clone()
        }
    }
}

// TODO: add test function
impl From<Order> for Holding {
    fn from(order: Order) -> Self {
        Holding {
            symbol: order.symbol,
            volume: order.volume,
            buy_price: order.price,
            buy_date: order.datetime,
            sell_date: None,
            sell_price: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Quote {
    pub symbol: String,
    pub volume: u32,
    pub price: Currency,
    pub datetime: DateTime<Utc>,
}

impl From<Order> for Quote {
    fn from(order: Order) -> Self {
        Quote {
            symbol: order.symbol,
            volume: order.volume,
            price: order.price,
            datetime: order.datetime,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::TimeZone;

    lazy_static! {
        static ref ORDER: Order = Order::new(
            String::from("AAPL"),
            10.50,
            10,
            Side::Buy,
            Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
        );
    }

    #[test]
    fn holding_new() {
        let expected = Holding {
            symbol: String::from("AAPL"),
            volume: 10,
            buy_price: Currency::from(10.50),
            buy_date: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
            sell_price: None,
            sell_date: None,
        };

        let tested = Holding::new(ORDER.clone());
        if tested != expected {
            panic!("\nexpected: {:?}, got {:?}", expected, tested)
        }
    }

    #[test]
    fn quote_from_order() {
        let expected = Quote {
            symbol: String::from("AAPL"),
            volume: 10,
            price: Currency::from(10.50),
            datetime: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
        };

        let tested = Quote::from(ORDER.clone());
        if tested != expected {
            panic!("\nexpected: {:?}, got {:?}", expected, tested)
        }
    }
    #[test]
    fn holding_buy() {
        let expected = Holding {
            symbol: String::from("AAPL"),
            volume: 10,
            buy_price: Currency::from(10.50),
            buy_date: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
            sell_price: None,
            sell_date: None,
        };

        let tested = Holding::buy(ORDER.clone());
        if tested != expected {
            panic!("\nexpected: {:?}, got {:?}", expected, tested)
        }
    }
    #[test]
    fn holding_sell() {
        let order = Order {
            volume: 5,
            price: Currency::from(11),
            datetime: Utc.ymd(2018, 1, 30).and_hms(10, 0, 0),
            ..ORDER.clone()
        };

        let mut initial = Holding {
            symbol: String::from("AAPL"),
            volume: 10,
            buy_price: Currency::from(10.50),
            buy_date: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
            sell_price: None,
            sell_date: None,
        };
        let initial_expected = Holding {
            symbol: String::from("AAPL"),
            volume: 5,
            buy_price: Currency::from(10.50),
            buy_date: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
            sell_price: None,
            sell_date: None,
        };
        let sold_expected = Holding {
            volume: 5,
            sell_price: Some(Currency::from(11)),
            sell_date: Some(Utc.ymd(2018, 1, 30).and_hms(10, 0, 0)),
            ..initial.clone()
        };

        let result = initial.sell(&order, order.volume);

        if result != sold_expected {
            panic!("\nexpected: {:?}, got {:?}", sold_expected, result)
        }

        if initial != initial_expected {
            panic!("\nexpected: {:?}, got {:?}", initial_expected, initial)
        }
    }

}
