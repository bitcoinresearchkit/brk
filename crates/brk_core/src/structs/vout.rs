use derive_deref::Deref;

#[derive(Debug, Deref, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vout(u32);

impl Vout {
    const ZERO: Self = Vout(0);

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl From<u32> for Vout {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<usize> for Vout {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<Vout> for u64 {
    fn from(value: Vout) -> Self {
        value.0 as u64
    }
}
