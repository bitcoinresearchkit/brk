use std::marker::PhantomData;

use vecdb::{UnaryTransform, VecValue};

/// T -> T (identity transform for lazy references)
pub struct Identity<T>(PhantomData<T>);

impl<T: VecValue> UnaryTransform<T, T> for Identity<T> {
    #[inline(always)]
    fn apply(v: T) -> T {
        v
    }
}
