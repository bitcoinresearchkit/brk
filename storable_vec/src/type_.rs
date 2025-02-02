use std::fmt::Debug;

pub trait StorableVecType
where
    Self: Sized + Debug + Clone,
{
}
impl<T> StorableVecType for T where T: Sized + Debug + Clone {}
