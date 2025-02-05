use std::fmt::Debug;

use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes};

pub trait StorableVecType
where
    Self: Sized + Debug + Clone + TryFromBytes + IntoBytes + Immutable + KnownLayout,
{
}
impl<T> StorableVecType for T where T: Sized + Debug + Clone + TryFromBytes + IntoBytes + Immutable + KnownLayout {}
