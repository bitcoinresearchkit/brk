use std::{convert::Infallible, marker::PhantomData, sync::Arc};

pub mod any_vec;
pub mod avg;
pub mod change;
pub mod clone;
pub mod op;
pub mod rate;
pub mod readable;
pub mod sub;
pub mod typed;

pub use avg::DeltaAvg;
pub use change::DeltaChange;
pub use op::DeltaOp;
pub use rate::DeltaRate;
pub use sub::DeltaSub;

use crate::{ReadableBoxedVec, VecIndex, VecValue, Version};

/// Lazily computed vector that combines a source value with a lookback value.
///
/// For each index `h` with `start = window_starts[h]`:
/// - `INCLUSIVE` ops (cumulative source): reads `source[h]` and `source[start - 1]`,
///   count = `h - start + 1`. Used for rolling sums/averages from prefix sums.
/// - Non-inclusive ops (raw source): reads `source[h]` and `source[start]`,
///   count = `h - start`. Used for point-to-point deltas (change, rate).
///
/// Nothing is stored on disk — values are computed on-the-fly during iteration.
pub struct LazyDeltaVec<I: VecIndex, S: VecValue, T, Op> {
    name: Arc<str>,
    base_version: Version,
    source: ReadableBoxedVec<I, S>,
    window_starts_version: Version,
    #[allow(clippy::type_complexity)]
    window_starts: Arc<dyn Fn() -> Arc<Vec<I>> + Send + Sync>,
    _op: PhantomData<(Op, T)>,
}

impl<I, S, T, Op> LazyDeltaVec<I, S, T, Op>
where
    I: VecIndex,
    S: VecValue,
    T: VecValue,
    Op: DeltaOp<S, T>,
{
    pub fn new(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<I, S>,
        window_starts_version: Version,
        window_starts: impl Fn() -> Arc<Vec<I>> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: Arc::from(name),
            base_version: version,
            source,
            window_starts_version,
            window_starts: Arc::new(window_starts),
            _op: PhantomData,
        }
    }

    /// Read overlapping ranges once, but never read the gap between disjoint
    /// current and historical ranges. Complete reads before invoking callbacks:
    /// sources may hold non-reentrant publication locks while lending chunks.
    fn with_source_ranges<R>(
        &self,
        from: usize,
        to: usize,
        starts: &[I],
        visit: impl FnOnce(&[S], usize, &[S]) -> R,
    ) -> R {
        let starts = &starts[from..to];
        let first = starts
            .iter()
            .find_map(|start| Op::ago_index(start.to_usize()));
        let last = starts
            .iter()
            .rev()
            .find_map(|start| Op::ago_index(start.to_usize()));
        if let Some((first, last)) = first.zip(last) {
            if last.saturating_add(1) >= from {
                let read_from = first.min(from);
                let values = self.source.collect_range_dyn(read_from, to);
                return visit(&values[from - read_from..], read_from, &values);
            }
            let previous = self.source.collect_range_dyn(first, last + 1);
            let current = self.source.collect_range_dyn(from, to);
            return visit(&current, first, &previous);
        }
        let current = self.source.collect_range_dyn(from, to);
        visit(&current, 0, &[])
    }

    fn transformed_values<'a>(
        at: usize,
        current: &'a [S],
        starts: &'a [I],
        previous_from: usize,
        previous: &'a [S],
    ) -> impl Iterator<Item = T> + 'a {
        (at..at + current.len())
            .zip(current)
            .zip(starts)
            .map(move |((index, current), start)| {
                let start = start.to_usize();
                let ago = Op::ago_index(start)
                    .map(|index| previous[index - previous_from].clone())
                    .unwrap_or_else(Op::ago_default);
                Op::combine(current.clone(), ago, Op::count(index, start))
            })
    }

    /// Scalar transform/consumer ordering is retained for fallible folds.
    #[inline]
    fn bulk_try_fold<B, E>(
        &self,
        from: usize,
        to: usize,
        starts: &[I],
        init: B,
        f: impl FnMut(B, T) -> Result<B, E>,
    ) -> Result<B, E> {
        if from >= to {
            return Ok(init);
        }

        self.with_source_ranges(from, to, starts, |current, previous_from, previous| {
            Self::transformed_values(from, current, &starts[from..to], previous_from, previous)
                .try_fold(init, f)
        })
    }

    #[inline]
    fn bulk_for_each(&self, from: usize, to: usize, starts: &[I], mut each: impl FnMut(T)) {
        self.bulk_try_fold(from, to, starts, (), |(), v| {
            each(v);
            Ok::<_, Infallible>(())
        })
        .unwrap_or_else(|e: Infallible| match e {})
    }
}
