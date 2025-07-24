use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct Stamp(u64);

impl Stamp {
    pub fn new(stamp: u64) -> Self {
        Self(stamp)
    }
}
