use super::Sats;

#[derive(Debug, Default, Clone, Copy)]
pub struct Bitcoin(f64);

impl Bitcoin {
    const ONE: Self = Self(100_000_000.0);
}

impl From<Sats> for Bitcoin {
    fn from(value: Sats) -> Self {
        Self((*value as f64) / Self::ONE.0)
    }
}
