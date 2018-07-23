#![allow(dead_code)]
extern crate chrono;
extern crate num;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use std::fmt::{Debug, Display};
use std::{error, fmt};

mod collections;
pub mod instruments;

#[derive(Debug, Clone)]
struct TransactionError<T: Debug + Display> {
    details: T,
}

impl<T> fmt::Display for TransactionError<T>
where
    T: Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid transaction, cannot satisfy Order: {:?}",
            self.details
        )
    }
}

impl<T> error::Error for TransactionError<T>
where
    T: Debug + Display,
{
    fn description(&self) -> &str {
        "invalid transaction, cannot satisfy order"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
