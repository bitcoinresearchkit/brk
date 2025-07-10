use std::ops::RangeInclusive;

pub trait FromCoarserIndex<T> {
    fn min_from(coarser: T) -> usize;
    fn max_from(coarser: T) -> usize;
    fn inclusive_range_from(coarser: T) -> RangeInclusive<usize>
    where
        T: Clone,
    {
        Self::min_from(coarser.clone())..=Self::max_from(coarser)
    }
}
