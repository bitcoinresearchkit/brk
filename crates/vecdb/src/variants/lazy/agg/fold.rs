use std::convert::Infallible;

use crate::{ReadableVec, VecIndex, VecValue};

/// Aggregation strategy for [`super::LazyAggVec`].
///
/// Determines how values are produced from a source vec and a pre-materialized mapping.
/// Implement this on a zero-sized marker type to define a custom strategy.
///
/// Built-in strategy: `Sparse`.
pub trait AggFold<O: VecValue, S1I: VecIndex, S2T: VecValue, S1T: VecValue>: 'static {
    fn try_fold<S: ReadableVec<S1I, S1T> + ?Sized, B, E, F: FnMut(B, O) -> Result<B, E>>(
        source: &S,
        mapping: &[S2T],
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E>;

    fn fold<S: ReadableVec<S1I, S1T> + ?Sized, B, F: FnMut(B, O) -> B>(
        source: &S,
        mapping: &[S2T],
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B {
        match Self::try_fold(source, mapping, from, to, init, |b, o| {
            Ok::<_, Infallible>(f(b, o))
        }) {
            Ok(b) => b,
            Err(e) => match e {},
        }
    }

    fn collect_one<S: ReadableVec<S1I, S1T> + ?Sized>(
        source: &S,
        mapping: &[S2T],
        index: usize,
    ) -> Option<O> {
        let mut result = None;
        Self::fold(source, mapping, index, index + 1, (), |(), v| {
            result = Some(v)
        });
        result
    }

    /// Evaluate only the requested output buckets. Strategies can batch their
    /// source lookups without computing adjacent, unrequested buckets.
    fn read_sorted_into<S: ReadableVec<S1I, S1T> + ?Sized>(
        source: &S,
        mapping: &[S2T],
        indices: &[usize],
        out: &mut Vec<O>,
    ) {
        out.reserve(indices.len());
        for &index in indices {
            if let Some(value) = Self::collect_one(source, mapping, index) {
                out.push(value);
            }
        }
    }
}
