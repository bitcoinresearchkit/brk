use std::ops::Mul;

use super::Sats;

#[derive(Debug, Default, Clone, Copy)]
pub struct Bitcoin(f64);

impl Mul for Bitcoin {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl From<Sats> for Bitcoin {
    fn from(value: Sats) -> Self {
        Self(u64::from(value) as f64 / (u64::from(Sats::ONE_BTC) as f64))
    }
}

impl From<f64> for Bitcoin {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Bitcoin> for f64 {
    fn from(value: Bitcoin) -> Self {
        value.0
    }
}
