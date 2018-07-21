pub use self::currency::Currency;
pub use self::order::{Order, Side};

mod currency;
mod order;

use chrono::{DateTime, Utc};
use std::option;

type Option<T> = option::Option<T>;

// TODO: Buy/Sell Error types.

trait Instrument<Output = Self>: Sized {
    fn buy(order: Order) -> Self;
    fn sell(&mut self, qty: u32, price: Currency, date: DateTime<Utc>) -> Option<Self>;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Holding {
    symbol: String,
    qty: u32,
    buy_price: Currency,
    buy_date: DateTime<Utc>,
    sell_price: Option<Currency>,
    sell_date: Option<DateTime<Utc>>,
}

impl Holding {
    pub fn new(order: Order) -> Holding {
        Holding {
            symbol: order.symbol.to_string(),
            qty: order.qty,
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
    // REVIEW: substitute input params for sell order?
    fn sell(&mut self, qty: u32, price: Currency, date: DateTime<Utc>) -> Option<Holding> {
        if qty > self.qty {
            return None;
        }
        self.qty -= qty;

        Some(Holding {
            qty: qty,
            sell_price: Some(price),
            sell_date: Some(date),
            ..self.clone()
        })
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Quote {
    pub symbol: String,
    pub qty: u32,
    pub price: Currency,
    pub datetime: DateTime<Utc>,
}

impl From<Order> for Quote {
    fn from(order: Order) -> Self {
        Quote {
            symbol: order.symbol,
            qty: order.qty,
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
            qty: 10,
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
            qty: 10,
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
            qty: 10,
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
        let mut initial = Holding {
            symbol: String::from("AAPL"),
            qty: 10,
            buy_price: Currency::from(10.50),
            buy_date: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
            sell_price: None,
            sell_date: None,
        };
        let initial_expected = Holding {
            symbol: String::from("AAPL"),
            qty: 5,
            buy_price: Currency::from(10.50),
            buy_date: Utc.ymd(2018, 1, 20).and_hms(10, 0, 0),
            sell_price: None,
            sell_date: None,
        };
        let sold_expected = Holding {
            qty: 5,
            sell_price: Some(Currency::from(11)),
            sell_date: Some(Utc.ymd(2018, 1, 30).and_hms(10, 0, 0)),
            ..initial.clone()
        };

        let sold = initial.sell(
            5,
            Currency::from(11),
            Utc.ymd(2018, 1, 30).and_hms(10, 0, 0),
        );

        let sold_tested = sold.unwrap();
        if sold_tested != sold_expected {
            panic!("\nexpected: {:?}, got {:?}", sold_expected, sold_tested)
        }

        if initial != initial_expected {
            panic!("\nexpected: {:?}, got {:?}", initial_expected, initial)
        }
    }

}
