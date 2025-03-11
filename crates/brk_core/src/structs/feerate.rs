use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Clone, Copy, Serialize, FromBytes, Immutable, IntoBytes, KnownLayout)]
pub struct Feerate(f32);
