use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;

use instruments::Currency;
use instruments::{Holding, Instrument, Order};
use Algorithm;
use TransactionError;

pub struct Account {
    cash: Currency,
    holdings: HashMap<String, Listing>,
    algorithm: &'static dyn Algorithm,
    benchmark: Option<&'static Account>,
}

impl Account {
    fn new<C>(starting_cash: C, algorithm: &'static dyn Algorithm) -> Self
    where
        C: Into<Currency>,
    {
        Account {
            cash: starting_cash.into(),
            holdings: HashMap::new(),
            algorithm,
            benchmark: None,
        }
    }

    fn with_benchmark(&mut self, benchmark: &'static Account) {
        self.benchmark = Some(benchmark);
    }
}

// REVIEW: should Listing assets be generic to Instrument trait?
#[derive(Debug, PartialEq, Eq)]
/// Listing is a struct for a collection of holdings.
pub struct Listing {
    symbol: String,
    volume: u32,
    assets: VecDeque<Holding>,
}

impl Listing {
    fn new(symbol: String) -> Listing {
        Listing {
            symbol,
            volume: 0,
            assets: VecDeque::new(),
        }
    }

    fn buy(&mut self, holding: Holding) {
        self.volume += holding.volume;
        self.assets.push_back(holding);
    }

    // TEMP|REVIEW: Selling off holdings in FIFO order. Should we allow for checking cost-methods?
    fn sell(&mut self, order: &mut Order) -> Result<Vec<Holding>, TransactionError<Order>> {
        if self.volume < order.volume {
            return Err(TransactionError {
                details: order.clone(),
            });
        };
        let mut sold: Vec<Holding> = vec![];

        let mut n = 0;
        loop {
            if order.volume == 0 {
                break;
            };

            if let Some(holding) = self.assets.get_mut(n) {
                let volume_to_sell = match holding.volume.cmp(&order.volume) {
                    Ordering::Less => holding.volume,
                    _ => order.volume,
                };
                sold.push(holding.sell(order, volume_to_sell));
                self.volume -= volume_to_sell;
                order.volume -= volume_to_sell;

                n += 1;
            };
        }
        self.assets.retain(|ref holding| holding.volume > 0);

        Ok(sold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use instruments::{Currency, OrderSide, OrderState, OrderStatus};

    #[test]
    fn listing_new_str() {
        let expected = Listing {
            symbol: "aapl".to_string(),
            volume: 0,
            assets: VecDeque::new(),
        };

        let result = Listing::new("aapl".to_string());
        if result != expected {
            panic!("\nexpected: {:?}, got {:?}", expected, result)
        };
    }

    #[test]
    fn listing_new_string() {
        let expected = Listing {
            symbol: "aapl".to_string(),
            volume: 0,
            assets: VecDeque::new(),
        };

        let symbol = &String::from("aapl".to_string())[..];
        let result = Listing::new(symbol.to_string());
        if result != expected {
            panic!("\nexpected: {:?}, got {:?}", expected, result)
        };
    }

    #[test]
    fn listing_buy() {
        let order = Order {
            volume: 5,
            price: Currency::from(11),
            datetime: Utc.ymd(2018, 1, 30).and_hms(10, 0, 0),
            symbol: "aapl".to_string().to_string(),
            status: OrderStatus::Open,
            side: OrderSide::Buy,
            state: OrderState::New,
        };
        let expected = Listing {
            symbol: "aapl".to_string(),
            volume: 5,
            assets: vec![Holding::from(order.clone())].into(),
        };
        let mut result = Listing::new("aapl".to_string());
        result.buy(Holding::from(order));

        if result != expected {
            panic!("\nexpected: {:?}, got {:?}", expected, result);
        };
    }

    #[test]
    fn listing_sell() {
        let order = Order {
            volume: 5,
            price: Currency::from(11),
            datetime: Utc.ymd(2018, 1, 30).and_hms(10, 0, 0),
            symbol: "aapl".to_string().to_string(),
            status: OrderStatus::Open,
            side: OrderSide::Sell,
            state: OrderState::New,
        };
        let mut holdings = Listing {
            symbol: "aapl".to_string(),
            volume: 5,
            assets: vec![Holding::from(order.clone())].into(),
        };
        let expected_holdings = Listing {
            symbol: "aapl".to_string(),
            volume: 0,
            assets: vec![].into(),
        };
        let sold_holding = Holding {
            sell_price: Some(order.price),
            sell_date: Some(order.datetime),
            ..order.clone().into()
        };
        let result_expected = vec![sold_holding];

        let result = holdings.sell(&mut order.clone());
        if holdings != expected_holdings {
            panic!("\nexpected: {:?}, got {:?}", expected_holdings, holdings);
        };

        match result {
            Ok(sold) => if sold != result_expected {
                panic!("\nexpected: {:?}, got {:?}", expected_holdings, holdings);
            },
            Err(e) => panic!(e),
        };
    }

    #[test]
    #[should_panic]
    fn listing_sell_panic() {
        let mut order = Order {
            volume: 5,
            price: Currency::from(11),
            datetime: Utc.ymd(2018, 1, 30).and_hms(10, 0, 0),
            symbol: "aapl".to_string().to_string(),
            status: OrderStatus::Open,
            side: OrderSide::Buy,
            state: OrderState::New,
        };
        let mut holdings = Listing {
            symbol: "aapl".to_string(),
            volume: 5,
            assets: vec![Holding::from(order.clone())].into(),
        };

        let mut results = vec![];
        results.push(holdings.sell(&mut order.clone()));
        results.push(holdings.sell(&mut order));

        for result in results {
            if let Err(e) = result {
                panic!(e);
            };
        }
    }
}
