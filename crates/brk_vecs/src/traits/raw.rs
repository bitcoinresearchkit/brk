use std::fmt::Debug;

use serde::Serialize;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

pub trait StoredRaw
where
    Self: Sized
        + Debug
        + Clone
        + FromBytes
        + IntoBytes
        + Immutable
        + KnownLayout
        + Send
        + Sync
        + Serialize,
{
}

impl<T> StoredRaw for T where
    T: Sized
        + Debug
        + Clone
        + FromBytes
        + IntoBytes
        + Immutable
        + KnownLayout
        + Send
        + Sync
        + Serialize
{
}
