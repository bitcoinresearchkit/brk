use std::ops::RangeInclusive;

/// Maps coarser-grained indices to ranges of finer-grained indices.
///
/// Useful for hierarchical index systems where one index type represents
/// a range of another (e.g., mapping hour indices to timestamp ranges).
pub trait FromCoarserIndex<T>
where
    T: Ord + From<usize>,
{
    /// Returns the minimum fine-grained index represented by the coarse index.
    fn min_from(coarser: T) -> usize;

    /// Returns the maximum fine-grained index represented by the coarse index.
    /// Note: May exceed actual data length - use `max_from` for bounded results.
    fn max_from_(coarser: T) -> usize;

    /// Returns the maximum fine-grained index, bounded by the data length.
    /// Returns 0 if len is 0 (empty data).
    fn max_from(coarser: T, len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        Self::max_from_(coarser).min(len - 1)
    }

    /// Returns the inclusive range of fine-grained indices for the coarse index.
    /// Returns an empty range (1..=0) if len is 0.
    fn inclusive_range_from(coarser: T, len: usize) -> RangeInclusive<usize>
    where
        T: Clone,
    {
        if len == 0 {
            #[allow(clippy::reversed_empty_ranges)]
            return 1..=0; // Empty range
        }
        Self::min_from(coarser.clone())..=Self::max_from(coarser, len)
    }
}
