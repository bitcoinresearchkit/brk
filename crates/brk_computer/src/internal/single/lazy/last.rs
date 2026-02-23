//! Lazy last-value aggregation.

use std::sync::Arc;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Cursor, FromCoarserIndex, ReadableBoxedVec, VecIndex, VecValue};

use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

type ForEachRangeFn<S1I, T, I, S2T> =
    fn(usize, usize, &ReadableBoxedVec<S1I, T>, &ReadableBoxedVec<I, S2T>, &mut dyn FnMut(T));

pub struct LazyLast<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema,
    S1I: VecIndex,
    S2T: VecValue,
{
    name: Arc<str>,
    version: Version,
    source: ReadableBoxedVec<S1I, T>,
    mapping: ReadableBoxedVec<I, S2T>,
    for_each_range: ForEachRangeFn<S1I, T, I, S2T>,
}

impl_lazy_agg!(LazyLast);

impl<I, T, S1I, S2T> LazyLast<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1I: VecIndex + 'static + FromCoarserIndex<I>,
    S2T: VecValue,
{
    pub(crate) fn from_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<S1I, T>,
        len_source: ReadableBoxedVec<I, S2T>,
    ) -> Self {
        fn for_each_range<
            I: VecIndex,
            T: VecValue,
            S1I: VecIndex + FromCoarserIndex<I>,
            S2T: VecValue,
        >(
            from: usize,
            to: usize,
            source: &ReadableBoxedVec<S1I, T>,
            mapping: &ReadableBoxedVec<I, S2T>,
            f: &mut dyn FnMut(T),
        ) {
            let mapping_len = mapping.len();
            let source_len = source.len();
            let mut cursor = Cursor::from_dyn(&**source);
            for i in from..to {
                if i >= mapping_len {
                    break;
                }
                let target = S1I::max_from(I::from(i), source_len);
                if cursor.position() <= target {
                    cursor.advance(target - cursor.position());
                    if let Some(v) = cursor.next() {
                        f(v);
                    }
                } else if let Some(v) = source.collect_one_at(target) {
                    f(v);
                }
            }
        }
        Self {
            name: Arc::from(name),
            version: version + VERSION,
            source,
            mapping: len_source,
            for_each_range: for_each_range::<I, T, S1I, S2T>,
        }
    }
}

impl<I, T> LazyLast<I, T, Height, Height>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
{
    /// Create from a height-indexed source using an explicit first_height mapping.
    /// For day1 d, looks up value at `first_height[d+1] - 1` (last height of the day).
    pub(crate) fn from_height_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        fn for_each_range<I: VecIndex, T: VecValue>(
            from: usize,
            to: usize,
            source: &ReadableBoxedVec<Height, T>,
            mapping: &ReadableBoxedVec<I, Height>,
            f: &mut dyn FnMut(T),
        ) {
            let map_end = (to + 1).min(mapping.len());
            let heights = mapping.collect_range_dyn(from, map_end);
            let source_len = source.len();
            let mut cursor = Cursor::from_dyn(&**source);
            for idx in 0..(to - from) {
                let next_first = heights
                    .get(idx + 1)
                    .map(|h| h.to_usize())
                    .unwrap_or(source_len);
                if next_first == 0 {
                    continue;
                }
                let target = next_first - 1;
                if cursor.position() <= target {
                    cursor.advance(target - cursor.position());
                    if let Some(v) = cursor.next() {
                        f(v);
                    }
                } else if let Some(v) = source.collect_one_at(target) {
                    f(v);
                }
            }
        }
        Self {
            name: Arc::from(name),
            version: version + VERSION,
            source,
            mapping: first_height,
            for_each_range: for_each_range::<I, T>,
        }
    }
}
