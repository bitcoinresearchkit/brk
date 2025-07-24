use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct Stamp(u64);

impl Stamp {
    pub fn new(stamp: u64) -> Self {
        Self(stamp)
    }
}

impl From<u64> for Stamp {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Stamp> for u64 {
    fn from(value: Stamp) -> Self {
        value.0
    }
}
