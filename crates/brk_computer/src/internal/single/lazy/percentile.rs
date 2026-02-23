//! Lazy percentile aggregation via const-generic fn pointers.

use std::sync::Arc;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use vecdb::{Cursor, FromCoarserIndex, ReadableBoxedVec, VecIndex, VecValue};

use crate::internal::ComputedVecValue;

const VERSION: Version = Version::ZERO;

type ForEachRangeFn<S1I, T, I, S2T> =
    fn(usize, usize, &ReadableBoxedVec<S1I, T>, &ReadableBoxedVec<I, S2T>, &mut dyn FnMut(T));

pub struct LazyPercentile<I, T, S1I, S2T>
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

impl_lazy_agg!(LazyPercentile);

fn select_and_pick<T: PartialOrd + Copy>(values: &mut [T], pct: u8) -> Option<T> {
    if values.is_empty() {
        return None;
    }
    let idx = (values.len() - 1) * pct as usize / 100;
    values.select_nth_unstable_by(idx, |a, b| {
        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
    });
    Some(values[idx])
}

impl<I, T, S1I, S2T> LazyPercentile<I, T, S1I, S2T>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
    S1I: VecIndex + 'static + FromCoarserIndex<I>,
    S2T: VecValue,
{
    pub(crate) fn from_source<const PCT: u8>(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<S1I, T>,
        len_source: ReadableBoxedVec<I, S2T>,
    ) -> Self {
        fn for_each_range<
            const PCT: u8,
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
            let mut values = Vec::new();
            for i in from..to {
                let start = S1I::min_from(I::from(i));
                let end = S1I::max_from(I::from(i), source_len) + 1;
                if end <= start {
                    continue;
                }
                values.clear();
                cursor.for_each(end - start, |v| values.push(v));
                if let Some(v) = select_and_pick(&mut values, PCT) {
                    f(v);
                }
            }
        }
        Self {
            name: Arc::from(name),
            version: version + VERSION,
            source,
            mapping: len_source,
            for_each_range: for_each_range::<PCT, I, T, S1I, S2T>,
        }
    }
}

impl<I, T> LazyPercentile<I, T, Height, Height>
where
    I: VecIndex,
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub(crate) fn from_height_source<const PCT: u8>(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        fn for_each_range<const PCT: u8, I: VecIndex, T: ComputedVecValue + JsonSchema>(
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
            let mut values = Vec::new();
            for idx in 0..(to - from) {
                let Some(&cur_h) = heights.get(idx) else {
                    continue;
                };
                let first = cur_h.to_usize();
                let next_first = heights
                    .get(idx + 1)
                    .map(|h| h.to_usize())
                    .unwrap_or(source_len);
                if next_first <= first {
                    continue;
                }
                values.clear();
                cursor.for_each(next_first - first, |v| values.push(v));
                if let Some(v) = select_and_pick(&mut values, PCT) {
                    f(v);
                }
            }
        }
        Self {
            name: Arc::from(name),
            version: version + VERSION,
            source,
            mapping: first_height,
            for_each_range: for_each_range::<PCT, I, T>,
        }
    }
}
