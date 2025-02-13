use derive_deref::Deref;

use super::Cents;

#[derive(Debug, Default, Clone, Copy, Deref)]
pub struct Dollars(f64);

impl From<f64> for Dollars {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Cents> for Dollars {
    fn from(value: Cents) -> Self {
        Self((*value as f64) / 100.0)
    }
}
