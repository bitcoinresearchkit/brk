//! Lazy min-value aggregation.

use std::sync::Arc;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Cursor, FromCoarserIndex, ReadableBoxedVec, VecIndex, VecValue};

use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

type ForEachRangeFn<S1I, T, I, S2T> =
    fn(usize, usize, &ReadableBoxedVec<S1I, T>, &ReadableBoxedVec<I, S2T>, &mut dyn FnMut(T));

pub struct LazyMin<I, T, S1I, S2T>
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

impl_lazy_agg!(LazyMin);

impl<I, T, S1I, S2T> LazyMin<I, T, S1I, S2T>
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
        Self::from_source_inner(&format!("{name}_min"), version, source, len_source)
    }

    fn from_source_inner(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<S1I, T>,
        len_source: ReadableBoxedVec<I, S2T>,
    ) -> Self {
        fn for_each_range<
            I: VecIndex,
            T: ComputedVecValue + JsonSchema,
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
            let to = to.min(mapping_len);
            if from >= to {
                return;
            }
            let mut cursor = Cursor::from_dyn(&**source);
            cursor.advance(S1I::min_from(I::from(from)));
            for i in from..to {
                let start = S1I::min_from(I::from(i));
                let end = S1I::max_from(I::from(i), source_len) + 1;
                let count = end.saturating_sub(start);
                if count == 0 {
                    continue;
                }
                if let Some(first) = cursor.next() {
                    f(cursor.fold(count - 1, first, |m, v| if v < m { v } else { m }));
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

impl<I, T> LazyMin<I, T, Height, Height>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub(crate) fn from_height_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        Self::from_height_source_inner(&format!("{name}_min"), version, source, first_height)
    }

    fn from_height_source_inner(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        fn for_each_range<I: VecIndex, T: ComputedVecValue + JsonSchema>(
            from: usize,
            to: usize,
            source: &ReadableBoxedVec<Height, T>,
            mapping: &ReadableBoxedVec<I, Height>,
            f: &mut dyn FnMut(T),
        ) {
            let map_end = (to + 1).min(mapping.len());
            let heights = mapping.collect_range_dyn(from, map_end);
            let source_len = source.len();
            let Some(&first_h) = heights.first() else {
                return;
            };
            let mut cursor = Cursor::from_dyn(&**source);
            cursor.advance(first_h.to_usize());
            for idx in 0..(to - from) {
                let Some(&cur_h) = heights.get(idx) else {
                    continue;
                };
                let first = cur_h.to_usize();
                let next_first = heights
                    .get(idx + 1)
                    .map(|h| h.to_usize())
                    .unwrap_or(source_len);
                let count = next_first.saturating_sub(first);
                if count == 0 {
                    continue;
                }
                if let Some(first_val) = cursor.next() {
                    f(cursor.fold(count - 1, first_val, |m, v| if v < m { v } else { m }));
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
