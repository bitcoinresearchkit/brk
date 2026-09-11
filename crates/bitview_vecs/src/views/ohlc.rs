use std::{convert::Infallible, iter, sync::Arc};

use bitview_traversable::{Traversable, TreeNode, make_leaf};
use brk_types::{Cents, Close, Height, High, Low, OHLCCents, Version};
use vecdb::{
    AnyExportableVec, AnyVec, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, SparseRead,
    TypedVec, VecIndex, short_type_name,
};

/// OHLC candles derived directly from spot prices and period boundaries.
#[derive(Clone)]
pub struct LazyOhlcVec<I: VecIndex> {
    name: Arc<str>,
    base_version: Version,
    prices: ReadableBoxedVec<Height, Cents>,
    first_heights: ReadableBoxedVec<I, Height>,
}

impl<I: VecIndex> LazyOhlcVec<I> {
    pub fn new(
        name: &str,
        version: Version,
        prices: &(impl ReadableCloneableVec<Height, Cents> + ?Sized),
        first_heights: &(impl ReadableCloneableVec<I, Height> + ?Sized),
    ) -> Self {
        Self {
            name: Arc::from(name),
            base_version: version,
            prices: prices.read_only_boxed_clone(),
            first_heights: first_heights.read_only_boxed_clone(),
        }
    }

    fn try_for_each_candle<E>(
        &self,
        from: usize,
        to: usize,
        mut each: impl FnMut(OHLCCents) -> Result<(), E>,
    ) -> Result<(), E> {
        let to = to.min(self.first_heights.len());
        if from >= to {
            return Ok(());
        }
        let first_heights = self
            .first_heights
            .collect_range_dyn(from, to.saturating_add(1));
        let price_len = self.prices.visible_len();
        let price_from = first_heights[0].to_usize().min(price_len).saturating_sub(1);
        let price_to = first_heights
            .get(to - from)
            .map_or(price_len, |height| height.to_usize().min(price_len));
        // Batch adjacent candles so shared physical pages are decoded once.
        // Include the preceding close for a leading empty period.
        let prices = self.prices.collect_range_dyn(price_from, price_to);
        for index in 0..to - from {
            let first = first_heights[index].to_usize().min(price_len);
            let end = first_heights
                .get(index + 1)
                .map_or(price_len, |height| height.to_usize().min(price_len));
            each(Self::candle_from_prices(
                first - price_from,
                end - price_from,
                &prices,
            ))?;
        }

        Ok(())
    }

    fn for_each_candle(&self, from: usize, to: usize, mut each: impl FnMut(OHLCCents)) {
        let result = self.try_for_each_candle(from, to, |candle| {
            each(candle);
            Ok::<_, Infallible>(())
        });
        match result {
            Ok(()) => {}
            Err(error) => match error {},
        }
    }

    fn candle(&self, first: usize, end: usize) -> OHLCCents {
        let from = first.saturating_sub(1);
        let prices = self.prices.collect_range_dyn(from, end.max(first));
        Self::candle_from_prices(first - from, end.saturating_sub(from), &prices)
    }

    fn candle_from_prices(first: usize, end: usize, prices: &[Cents]) -> OHLCCents {
        if first >= end {
            let close = first
                .checked_sub(1)
                .and_then(|height| prices.get(height))
                .copied()
                .unwrap_or_default();
            return OHLCCents::from(Close::new(close));
        }
        let mut candle = OHLCCents::from(Close::new(prices[first]));
        for &price in &prices[first + 1..end] {
            candle.high = candle.high.max(High::new(price));
            candle.low = candle.low.min(Low::new(price));
            candle.close = Close::new(price);
        }
        candle
    }
}

impl<I: VecIndex> AnyVec for LazyOhlcVec<I> {
    fn version(&self) -> Version {
        self.base_version + self.prices.version() + self.first_heights.version()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn len(&self) -> usize {
        self.first_heights.len()
    }

    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<OHLCCents>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<OHLCCents>()
    }
}

impl<I: VecIndex> TypedVec for LazyOhlcVec<I> {
    type I = I;
    type T = OHLCCents;
}

impl<I: VecIndex> ReadableVec<I, OHLCCents> for LazyOhlcVec<I> {
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<OHLCCents>) {
        buf.reserve(to.min(self.len()).saturating_sub(from));
        self.for_each_candle(from, to, |candle| buf.push(candle));
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(OHLCCents)) {
        self.for_each_candle(from, to, each);
    }

    fn fold_range_at<B, F: FnMut(B, OHLCCents) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> B {
        let mut acc = Some(init);
        self.for_each_candle(from, to, |candle| {
            acc = Some(fold(acc.take().unwrap(), candle));
        });
        acc.unwrap()
    }

    fn try_fold_range_at<B, E, F: FnMut(B, OHLCCents) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> Result<B, E> {
        let mut acc = Some(init);
        self.try_for_each_candle(from, to, |candle| {
            acc = Some(fold(acc.take().unwrap(), candle)?);
            Ok(())
        })?;
        Ok(acc.unwrap())
    }

    fn collect_one_at(&self, index: usize) -> Option<OHLCCents> {
        let boundaries = self
            .first_heights
            .collect_range_dyn(index, index.saturating_add(2));
        let price_len = self.prices.visible_len();
        let first = boundaries.first()?.to_usize().min(price_len);
        let end = boundaries
            .get(1)
            .map_or(price_len, |height| height.to_usize().min(price_len));
        Some(self.candle(first, end))
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<OHLCCents>) {
        let len = self.first_heights.len();
        let indices = &indices[..indices.partition_point(|&index| index < len)];
        let boundaries = SparseRead::new(&self.first_heights, indices, |index| {
            (index + 1 < len).then_some(index + 1)
        });
        let price_len = self.prices.visible_len();
        out.reserve(indices.len());
        for slot in 0..indices.len() {
            let first = boundaries.current(slot).to_usize().min(price_len);
            let end = boundaries
                .previous(slot)
                .map_or(price_len, |height| height.to_usize().min(price_len));
            out.push(self.candle(first, end));
        }
    }
}

impl<I: VecIndex> Traversable for LazyOhlcVec<I> {
    fn iter_any_exportable(&self) -> impl Iterator<Item = &dyn AnyExportableVec> {
        iter::once(self as &dyn AnyExportableVec)
    }

    fn to_tree_node(&self) -> TreeNode {
        make_leaf::<I, OHLCCents, _>(self)
    }
}

#[cfg(test)]
mod tests {
    static TEST_CACHE: CacheBudget = CacheBudget::new(64 * 1024 * 1024);
    use std::{
        env, fs, process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use brk_types::Day1;
    use vecdb::{
        AnyStoredVec, Budgeted, CacheBudget, Database, EagerVec, ImportOptions, ImportableVec,
        PcoVec, WritableVec,
    };

    use super::*;
    use crate::LazyFirstHeightVec;

    fn values(candle: &OHLCCents) -> (u64, u64, u64, u64) {
        (**candle.open, **candle.high, **candle.low, **candle.close)
    }

    #[test]
    fn derives_candles_and_preserves_empty_periods() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!("brk-lazy-ohlc-{}-{suffix}", process::id()));
        let db = Database::open(&path).unwrap();

        let mut prices: EagerVec<PcoVec<Height, Cents, Budgeted>> = EagerVec::forced_import_with(
            ImportOptions::new(&db, "prices", Version::ONE).with_cache_budget(&TEST_CACHE),
        )
        .unwrap();
        let mut periods: EagerVec<PcoVec<Height, Day1, Budgeted>> = EagerVec::forced_import_with(
            ImportOptions::new(&db, "periods", Version::ONE).with_cache_budget(&TEST_CACHE),
        )
        .unwrap();

        for value in [10, 20, 5, 7] {
            prices.push(Cents::new(value));
        }
        for period in [0, 2, 2, 4] {
            periods.push(Day1::from(period));
        }
        prices.write().unwrap();
        periods.write().unwrap();

        let first_heights = LazyFirstHeightVec::new(&periods);
        let boundaries = first_heights.collect();
        let ohlc = LazyOhlcVec::new("ohlc", Version::ONE, &prices, &first_heights);

        assert_eq!(&boundaries, &ohlc.first_heights.collect());
        assert_eq!(&prices.collect(), &ohlc.prices.collect());
        let candles = ohlc.collect();
        assert_eq!(candles.len(), 5);
        assert_eq!(
            candles.iter().map(values).collect::<Vec<_>>(),
            [
                (10, 10, 10, 10),
                (10, 10, 10, 10),
                (20, 20, 5, 5),
                (5, 5, 5, 5),
                (7, 7, 7, 7)
            ],
        );
        assert_eq!(
            ohlc.collect_range(Day1::from(1), Day1::from(4))
                .iter()
                .map(values)
                .collect::<Vec<_>>(),
            candles[1..4].iter().map(values).collect::<Vec<_>>(),
        );
        assert_eq!(
            ohlc.collect_one(Day1::from(2)).as_ref().map(values),
            Some(values(&candles[2])),
        );

        drop(ohlc);
        drop(prices);
        drop(periods);
        drop(db);
        fs::remove_dir_all(path).unwrap();
    }
}
