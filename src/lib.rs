#![allow(clippy::all)]
use actix::prelude::*;

pub mod order;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
