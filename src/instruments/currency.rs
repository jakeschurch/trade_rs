use std::cmp::Ordering;
use std::fmt;
use std::ops::Div;

/// Currency is a struct used to represent monetary value, such as a price.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Currency(pub i64);

impl From<usize> for Currency {
    fn from(input: usize) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<u8> for Currency {
    fn from(input: u8) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<u16> for Currency {
    fn from(input: u16) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<u32> for Currency {
    fn from(input: u32) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<u64> for Currency {
    fn from(input: u64) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<isize> for Currency {
    fn from(input: isize) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<i8> for Currency {
    fn from(input: i8) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<i16> for Currency {
    fn from(input: i16) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<i32> for Currency {
    fn from(input: i32) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<i64> for Currency {
    fn from(input: i64) -> Self {
        Currency {
            0: input as i64 * 100,
        }
    }
}

impl From<f32> for Currency {
    fn from(input: f32) -> Self {
        Currency {
            0: (input * 100.00) as i64,
        }
    }
}

impl From<f64> for Currency {
    fn from(input: f64) -> Self {
        Currency {
            0: (input * 100.00) as i64,
        }
    }
}

impl Div<Currency> for Currency {
    type Output = f32;
    fn div(self, rhs: Currency) -> Self::Output {
        let numerator = self.0 * 200 + rhs.0;
        let denominator = rhs.0 * 2;
        ((numerator / denominator) / 100) as f32
    }
}

// TODO: Impl Mul<Currency> trait

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Currency) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Currency) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.0.to_string();
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

    const EXPECTED: Currency = Currency(200);

    #[test]
    fn currency_from_usize() {
        let num: usize = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_u8() {
        let num: u8 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_u16() {
        let num: u16 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_u32() {
        let num: u32 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_u64() {
        let num: u64 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_isize() {
        let num: isize = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_i8() {
        let num: i8 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_i16() {
        let num: i16 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_i32() {
        let num: i32 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_i64() {
        let num: i64 = 2;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_f32() {
        let num: f32 = 2.0;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_from_f64() {
        let num: f64 = 2.0;
        assert_eq!(Currency::from(num), EXPECTED)
    }

    #[test]
    fn currency_ord() {
        assert!(Currency::from(10.00) > Currency::from(9.00));
    }

    #[test]
    fn currency_div() {
        assert_eq!(Currency::from(20) / Currency::from(2), 10.00)
    }
}
