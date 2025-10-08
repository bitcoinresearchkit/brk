use derive_deref::Deref;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vin(u32);

impl Vin {
    pub const ZERO: Self = Vin(0);
    pub const ONE: Self = Vin(1);

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl From<u32> for Vin {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for Vin {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<Vin> for u64 {
    fn from(value: Vin) -> Self {
        value.0 as u64
    }
}
