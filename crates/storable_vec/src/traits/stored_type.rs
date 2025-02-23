use std::fmt::Debug;

use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

pub trait StoredType
where
    Self: Sized + Debug + Clone + TryFromBytes + IntoBytes + Immutable + KnownLayout + Send + Sync,
{
}
impl<T> StoredType for T where
    T: Sized + Debug + Clone + TryFromBytes + IntoBytes + Immutable + KnownLayout + Send + Sync
{
}
