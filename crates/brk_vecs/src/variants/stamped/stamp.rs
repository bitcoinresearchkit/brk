use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Default, Clone, Copy, FromBytes, IntoBytes, Immutable, KnownLayout)]
pub struct Stamp(u64);
