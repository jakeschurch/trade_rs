extern crate chrono;

use std::fmt;

#[derive(Debug)]
pub struct Currency {
    amount: i64,
}

impl Currency {
    pub fn new(input: f64) -> Self {
        Currency {
            amount: (input * 100.00) as i64,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.amount.to_string();
        let mut result = String::with_capacity(s.len());

        let mut i = 0;
        for c in s.chars() {
            if i == s.len() - 2 {
                result.push('.');
            }
            result.push(c);
            i += 1;
        }

        write!(f, "${}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_new_test() {
        assert_eq!(1025, Currency::new(10.25).amount);
        assert_eq!(-1025, Currency::new(-10.25).amount);
    }
    #[test]
    fn currency_fmt_test() {
        println!("{}", Currency::new(10.25));
        println!("{}", Currency::new(1025.00));
    }
}
