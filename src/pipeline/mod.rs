use collections::Account;
use instruments::Quote;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash;

pub struct Broker {
    pub stocks: HashMap<String, Quote, BuildHasherDefault<XxHash>>,
    accounts: Vec<Account>,
}

impl Broker {
    fn new() -> Broker {
        Broker {
            stocks: Default::default(),
            accounts: vec![],
        }
    }

    fn update(&mut self, quote: Quote) {
        let key = quote.symbol.to_string();

        let stocks = &mut self.stocks;
        stocks
            .entry(key)
            .and_modify(|stock| stock.update(quote.clone()))
            .or_insert(quote);
    }
}

pub struct OrderBook {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_broker_update() {}
}
