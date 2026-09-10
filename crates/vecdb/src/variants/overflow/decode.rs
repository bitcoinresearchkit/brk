use crate::{OverflowVecValue, unlikely};

/// Decode the representation independently of the reader's visibility policy.
#[inline(always)]
pub(super) fn decode<T: OverflowVecValue>(
    compact: T::Compact,
    overflow: impl FnOnce(usize) -> T,
) -> T {
    let index = T::overflow_index(compact);
    if unlikely(index.is_some()) {
        overflow(index.unwrap())
    } else {
        T::from_compact(compact)
    }
}
