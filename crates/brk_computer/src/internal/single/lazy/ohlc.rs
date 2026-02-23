//! Lazy OHLC aggregation — single-pass first/max/min/last from height-level data.

use std::sync::Arc;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use schemars::JsonSchema;
use serde::Serialize;
use vecdb::{Cursor, Formattable, ReadableBoxedVec, VecIndex, VecValue};

use brk_types::{Cents, Close, Dollars, High, Low, OHLCCents, OHLCDollars, OHLCSats, Open, Sats};

use crate::internal::ComputedVecValue;

/// Trait for OHLC bundle types that can be constructed from / decomposed into
/// their open/high/low/close components.
pub trait OHLCRecord: VecValue + Formattable + Serialize + JsonSchema {
    type Inner: ComputedVecValue + JsonSchema + Copy;
    fn ohlc_open(&self) -> Self::Inner;
    fn ohlc_high(&self) -> Self::Inner;
    fn ohlc_low(&self) -> Self::Inner;
    fn ohlc_close(&self) -> Self::Inner;
    fn from_parts(
        open: Self::Inner,
        high: Self::Inner,
        low: Self::Inner,
        close: Self::Inner,
    ) -> Self;
}

impl OHLCRecord for OHLCCents {
    type Inner = Cents;
    fn ohlc_open(&self) -> Cents {
        *self.open
    }
    fn ohlc_high(&self) -> Cents {
        *self.high
    }
    fn ohlc_low(&self) -> Cents {
        *self.low
    }
    fn ohlc_close(&self) -> Cents {
        *self.close
    }
    fn from_parts(open: Cents, high: Cents, low: Cents, close: Cents) -> Self {
        Self {
            open: Open::new(open),
            high: High::new(high),
            low: Low::new(low),
            close: Close::new(close),
        }
    }
}

impl OHLCRecord for OHLCDollars {
    type Inner = Dollars;
    fn ohlc_open(&self) -> Dollars {
        *self.open
    }
    fn ohlc_high(&self) -> Dollars {
        *self.high
    }
    fn ohlc_low(&self) -> Dollars {
        *self.low
    }
    fn ohlc_close(&self) -> Dollars {
        *self.close
    }
    fn from_parts(open: Dollars, high: Dollars, low: Dollars, close: Dollars) -> Self {
        Self {
            open: Open::new(open),
            high: High::new(high),
            low: Low::new(low),
            close: Close::new(close),
        }
    }
}

impl OHLCRecord for OHLCSats {
    type Inner = Sats;
    fn ohlc_open(&self) -> Sats {
        *self.open
    }
    fn ohlc_high(&self) -> Sats {
        *self.high
    }
    fn ohlc_low(&self) -> Sats {
        *self.low
    }
    fn ohlc_close(&self) -> Sats {
        *self.close
    }
    fn from_parts(open: Sats, high: Sats, low: Sats, close: Sats) -> Self {
        Self {
            open: Open::new(open),
            high: High::new(high),
            low: Low::new(low),
            close: Close::new(close),
        }
    }
}

const VERSION: Version = Version::ZERO;

type ForEachRangeFn<S1I, ST, I, S2T, OHLC> =
    fn(usize, usize, &ReadableBoxedVec<S1I, ST>, &ReadableBoxedVec<I, S2T>, &mut dyn FnMut(OHLC));

/// Lazy OHLC aggregation vec. For each coarser period, computes open (first),
/// high (max), low (min), close (last) in a single pass over the source range.
pub struct LazyOHLC<I, OHLC, S1I, ST, S2T>
where
    I: VecIndex,
    OHLC: OHLCRecord,
    S1I: VecIndex,
    ST: VecValue,
    S2T: VecValue,
{
    name: Arc<str>,
    version: Version,
    source: ReadableBoxedVec<S1I, ST>,
    mapping: ReadableBoxedVec<I, S2T>,
    for_each_range: ForEachRangeFn<S1I, ST, I, S2T, OHLC>,
}

// --- From height source (Day1, DifficultyEpoch) ---

impl<I, OHLC, T> LazyOHLC<I, OHLC, Height, T, Height>
where
    I: VecIndex,
    OHLC: OHLCRecord<Inner = T> + 'static,
    T: ComputedVecValue + JsonSchema + 'static,
{
    pub(crate) fn from_height_source(
        name: &str,
        version: Version,
        source: ReadableBoxedVec<Height, T>,
        first_height: ReadableBoxedVec<I, Height>,
    ) -> Self {
        fn for_each_range<
            I: VecIndex,
            OHLC: OHLCRecord<Inner = T>,
            T: ComputedVecValue + JsonSchema,
        >(
            from: usize,
            to: usize,
            source: &ReadableBoxedVec<Height, T>,
            mapping: &ReadableBoxedVec<I, Height>,
            f: &mut dyn FnMut(OHLC),
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
                    let (high, low, close) = cursor.fold(
                        count - 1,
                        (first_val, first_val, first_val),
                        |(hi, lo, _), v| {
                            (if v > hi { v } else { hi }, if v < lo { v } else { lo }, v)
                        },
                    );
                    f(OHLC::from_parts(first_val, high, low, close));
                }
            }
        }
        Self {
            name: Arc::from(format!("{name}_ohlc")),
            version: version + VERSION,
            source,
            mapping: first_height,
            for_each_range: for_each_range::<I, OHLC, T>,
        }
    }
}

// --- Trait implementations ---

impl<I, OHLC, S1I, ST, S2T> Clone for LazyOHLC<I, OHLC, S1I, ST, S2T>
where
    I: VecIndex,
    OHLC: OHLCRecord,
    S1I: VecIndex,
    ST: VecValue,
    S2T: VecValue,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            version: self.version,
            source: self.source.clone(),
            mapping: self.mapping.clone(),
            for_each_range: self.for_each_range,
        }
    }
}

impl<I, OHLC, S1I, ST, S2T> vecdb::AnyVec for LazyOHLC<I, OHLC, S1I, ST, S2T>
where
    I: VecIndex,
    OHLC: OHLCRecord,
    S1I: VecIndex,
    ST: VecValue,
    S2T: VecValue,
{
    fn version(&self) -> Version {
        self.version + self.source.version() + self.mapping.version()
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }
    fn len(&self) -> usize {
        self.mapping.len()
    }
    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<OHLC>()
    }
    #[inline]
    fn value_type_to_string(&self) -> &'static str {
        vecdb::short_type_name::<OHLC>()
    }
    #[inline]
    fn region_names(&self) -> Vec<String> {
        vec![]
    }
}

impl<I, OHLC, S1I, ST, S2T> vecdb::TypedVec for LazyOHLC<I, OHLC, S1I, ST, S2T>
where
    I: VecIndex,
    OHLC: OHLCRecord,
    S1I: VecIndex,
    ST: VecValue,
    S2T: VecValue,
{
    type I = I;
    type T = OHLC;
}

impl<I, OHLC, S1I, ST, S2T> vecdb::ReadableVec<I, OHLC> for LazyOHLC<I, OHLC, S1I, ST, S2T>
where
    I: VecIndex,
    OHLC: OHLCRecord,
    S1I: VecIndex,
    ST: VecValue,
    S2T: VecValue,
{
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<OHLC>) {
        let to = to.min(self.mapping.len());
        if from >= to {
            return;
        }
        buf.reserve(to - from);
        (self.for_each_range)(from, to, &self.source, &self.mapping, &mut |v| buf.push(v));
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(OHLC)) {
        let to = to.min(self.mapping.len());
        if from >= to {
            return;
        }
        (self.for_each_range)(from, to, &self.source, &self.mapping, f);
    }

    #[inline]
    fn fold_range_at<B, F: FnMut(B, OHLC) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B
    where
        Self: Sized,
    {
        let to = to.min(self.mapping.len());
        if from >= to {
            return init;
        }
        let mut acc = Some(init);
        (self.for_each_range)(from, to, &self.source, &self.mapping, &mut |v| {
            acc = Some(f(acc.take().unwrap(), v));
        });
        acc.unwrap()
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, OHLC) -> std::result::Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> std::result::Result<B, E>
    where
        Self: Sized,
    {
        let to = to.min(self.mapping.len());
        if from >= to {
            return Ok(init);
        }
        let mut acc: Option<std::result::Result<B, E>> = Some(Ok(init));
        (self.for_each_range)(from, to, &self.source, &self.mapping, &mut |v| {
            if let Some(Ok(a)) = acc.take() {
                acc = Some(f(a, v));
            }
        });
        acc.unwrap()
    }

    #[inline]
    fn collect_one_at(&self, index: usize) -> Option<OHLC> {
        if index >= self.mapping.len() {
            return None;
        }
        let mut result = None;
        (self.for_each_range)(index, index + 1, &self.source, &self.mapping, &mut |v| {
            result = Some(v)
        });
        result
    }
}

impl<I, OHLC, S1I, ST, S2T> Traversable for LazyOHLC<I, OHLC, S1I, ST, S2T>
where
    I: VecIndex + 'static,
    OHLC: OHLCRecord + 'static,
    S1I: VecIndex + 'static,
    ST: VecValue,
    S2T: VecValue,
{
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn vecdb::AnyExportableVec> {
        std::iter::once(self as &dyn vecdb::AnyExportableVec)
    }

    fn to_tree_node(&self) -> brk_types::TreeNode {
        use vecdb::AnyVec;
        let index_str = I::to_string();
        let index = brk_types::Index::try_from(index_str).ok();
        let indexes = index.into_iter().collect();
        let leaf = brk_types::MetricLeaf::new(
            self.name().to_string(),
            self.value_type_to_string().to_string(),
            indexes,
        );
        let schema = schemars::SchemaGenerator::default().into_root_schema_for::<OHLC>();
        let schema_json = serde_json::to_value(schema).unwrap_or_default();
        brk_types::TreeNode::Leaf(brk_types::MetricLeafWithSchema::new(leaf, schema_json))
    }
}
