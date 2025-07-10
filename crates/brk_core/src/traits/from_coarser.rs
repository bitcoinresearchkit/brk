use std::ops::RangeInclusive;

pub trait FromCoarserIndex<T> {
    fn min_from(coarser: T) -> usize;
    fn max_from_(coarser: T) -> usize;
    fn max_from(coarser: T, len: usize) -> usize {
        Self::max_from_(coarser).min(len - 1)
    }
    fn inclusive_range_from(coarser: T, len: usize) -> RangeInclusive<usize>
    where
        T: Clone,
    {
        Self::min_from(coarser.clone())..=Self::max_from(coarser, len)
    }
}
