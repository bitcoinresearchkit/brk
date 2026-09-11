use std::convert::Infallible;

use brk_types::{Day1, Dollars, Height, Sats, Version};
use vecdb::{
    AnyVec, PrintableIndex, ReadOnlyClone, ReadableBoxedVec, ReadableVec, TypedVec, VecIndex,
    short_type_name,
};

use super::DCA_AMOUNT;

/// Cumulative daily purchases projected onto the monotonic height-to-day mapping.
#[derive(Clone)]
pub struct DcaSats {
    daily_close: ReadableBoxedVec<Day1, Option<Dollars>>,
    days: ReadableBoxedVec<Height, Day1>,
}

impl DcaSats {
    pub fn new(
        daily_close: ReadableBoxedVec<Day1, Option<Dollars>>,
        days: impl ReadableVec<Height, Day1> + Clone + 'static,
    ) -> Self {
        Self {
            daily_close,
            days: ReadableBoxedVec::new(days),
        }
    }

    fn sats_at_price(price: Dollars) -> Sats {
        Sats::from_dollars_at_price(DCA_AMOUNT, price)
    }

    fn try_for_each_value<E>(
        &self,
        from: usize,
        to: usize,
        mut each: impl FnMut(Sats) -> Result<(), E>,
    ) -> Result<(), E> {
        let days = self.days.collect_range_dyn(from, to);
        for value in self.daily_values(&days) {
            each(value)?;
        }
        Ok(())
    }

    fn daily_values(&self, days: &[Day1]) -> Vec<Sats> {
        let Some(last) = self.daily_close.len().checked_sub(1) else {
            return vec![Sats::ZERO; days.len()];
        };
        let to = days.last().map_or(0, |day| day.to_usize().min(last) + 1);
        let mut values = Vec::with_capacity(days.len());
        let mut cumulative = Sats::ZERO;
        let mut day = 0;
        // One forward pass serves every requested day, including duplicates.
        self.daily_close.for_each_range_dyn_at(0, to, &mut |price| {
            cumulative += Self::sats_at_price(price.unwrap_or_default());
            while days
                .get(values.len())
                .is_some_and(|requested| requested.to_usize().min(last) == day)
            {
                values.push(cumulative);
            }
            day += 1;
        });
        values
    }

    fn for_each_value(&self, from: usize, to: usize, mut each: impl FnMut(Sats)) {
        let result = self.try_for_each_value(from, to, |value| {
            each(value);
            Ok::<_, Infallible>(())
        });
        match result {
            Ok(()) => {}
            Err(error) => match error {},
        }
    }
}

impl AnyVec for DcaSats {
    fn version(&self) -> Version {
        self.days.version() + self.daily_close.version()
    }

    fn name(&self) -> &str {
        "dca_sats_cumulative"
    }

    fn len(&self) -> usize {
        self.days.len()
    }

    fn index_type_to_string(&self) -> &'static str {
        <Height as PrintableIndex>::to_string()
    }

    fn region_names(&self) -> Vec<String> {
        Vec::new()
    }

    fn value_type_to_size_of(&self) -> usize {
        size_of::<Sats>()
    }

    fn value_type_to_string(&self) -> &'static str {
        short_type_name::<Sats>()
    }
}

impl TypedVec for DcaSats {
    type I = Height;
    type T = Sats;
}

impl ReadableVec<Height, Sats> for DcaSats {
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<Sats>) {
        buf.reserve(to.min(self.len()).saturating_sub(from));
        self.for_each_value(from, to, |value| buf.push(value));
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(Sats)) {
        self.for_each_value(from, to, each);
    }

    fn fold_range_at<B, F: FnMut(B, Sats) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> B {
        let mut acc = Some(init);
        self.for_each_value(from, to, |value| {
            acc = Some(fold(acc.take().unwrap(), value));
        });
        acc.unwrap()
    }

    fn try_fold_range_at<B, E, F: FnMut(B, Sats) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut fold: F,
    ) -> Result<B, E> {
        let mut acc = Some(init);
        self.try_for_each_value(from, to, |value| {
            acc = Some(fold(acc.take().unwrap(), value)?);
            Ok(())
        })?;
        Ok(acc.unwrap())
    }

    fn collect_one_at(&self, index: usize) -> Option<Sats> {
        let day = self.days.collect_one_at(index)?.to_usize();
        Some(self.daily_close.fold_range_at(
            0,
            day.saturating_add(1).min(self.daily_close.len()),
            Sats::ZERO,
            |sum, price| sum + Self::sats_at_price(price.unwrap_or_default()),
        ))
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<Sats>) {
        let days = self.days.read_sorted_at(indices);
        out.extend(self.daily_values(&days));
    }
}

impl ReadOnlyClone for DcaSats {
    type ReadOnly = Self;

    fn read_only_clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::{marker::PhantomData, sync::Arc};

    use bitview_vecs::RangeMapLookupVec;
    use parking_lot::RwLock;
    use rangeindex::SharedRangeMap;
    use vecdb::{LazyVec, ReadBounds, ReadableCloneableVec, VecValue, short_type_name};

    use super::*;

    #[derive(Clone)]
    struct MemoryVec<I, T> {
        values: Arc<RwLock<Vec<T>>>,
        index: PhantomData<fn() -> I>,
    }

    impl<I, T> MemoryVec<I, T> {
        fn new(values: impl IntoIterator<Item = T>) -> Self {
            Self {
                values: Arc::new(RwLock::new(values.into_iter().collect())),
                index: PhantomData,
            }
        }

        fn replace(&self, index: usize, value: T) {
            self.values.write()[index] = value;
        }
    }

    impl<I: VecIndex, T: VecValue> AnyVec for MemoryVec<I, T> {
        fn version(&self) -> Version {
            Version::ONE
        }

        fn name(&self) -> &str {
            "memory"
        }

        fn len(&self) -> usize {
            self.values.read().len()
        }

        fn index_type_to_string(&self) -> &'static str {
            I::to_string()
        }

        fn region_names(&self) -> Vec<String> {
            Vec::new()
        }

        fn value_type_to_size_of(&self) -> usize {
            size_of::<T>()
        }

        fn value_type_to_string(&self) -> &'static str {
            short_type_name::<T>()
        }
    }

    impl<I: VecIndex, T: VecValue> TypedVec for MemoryVec<I, T> {
        type I = I;
        type T = T;
    }

    impl<I: VecIndex, T: VecValue> ReadableVec<I, T> for MemoryVec<I, T> {
        fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
            let values = self.values.read();
            let to = to.min(values.len());
            if from < to {
                buf.extend_from_slice(&values[from..to]);
            }
        }

        fn for_each_range_dyn_at(&self, from: usize, to: usize, each: &mut dyn FnMut(T)) {
            let values = self.values.read();
            let to = to.min(values.len());
            for value in &values[from.min(to)..to] {
                each(value.clone());
            }
        }

        fn fold_range_at<B, F: FnMut(B, T) -> B>(
            &self,
            from: usize,
            to: usize,
            init: B,
            mut fold: F,
        ) -> B {
            let values = self.values.read();
            let to = to.min(values.len());
            values[from.min(to)..to]
                .iter()
                .cloned()
                .fold(init, &mut fold)
        }

        fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
            &self,
            from: usize,
            to: usize,
            init: B,
            mut fold: F,
        ) -> Result<B, E> {
            let values = self.values.read();
            let to = to.min(values.len());
            values[from.min(to)..to]
                .iter()
                .cloned()
                .try_fold(init, &mut fold)
        }
    }

    #[test]
    fn shared_day_mapping_preserves_purchases_through_reorg_and_publication() {
        let prices = MemoryVec::<Day1, _>::new([
            Some(Dollars::mint(100.0)),
            None,
            Some(Dollars::mint(200.0)),
        ]);
        let domain = MemoryVec::<Height, _>::new([Day1::default(); 5]);
        let metadata = LazyVec::init(
            "day1",
            Version::ZERO,
            domain.read_only_boxed_clone(),
            |_, _| -> Day1 { panic!("DCA must use shared day boundaries") },
        );
        let starts = SharedRangeMap::new([0usize, 2, 3, 4].map(Height::from).to_vec());
        let days = RangeMapLookupVec::new(&starts, &metadata);
        let cumulative = DcaSats::new(prices.read_only_boxed_clone(), days).clone();
        let first = DcaSats::sats_at_price(Dollars::mint(100.0));
        let third = first + DcaSats::sats_at_price(Dollars::mint(200.0));
        assert_eq!(cumulative.collect(), [first, first, first, third, third]);
        starts.update_at(1, [1usize, 4, 4].map(Height::from));
        assert_eq!(cumulative.collect(), [first, first, first, first, third]);
        assert_eq!(
            cumulative.read_sorted_at(&[0, 3, 3, 4, 5]),
            [first, first, first, third]
        );
        let mut bounds = ReadBounds::new();
        bounds.set("height", 4);
        bounds.scope(|| {
            assert_eq!(cumulative.collect(), [first; 4]);
            assert_eq!(cumulative.collect_one_at(4), None);
        });
    }

    #[test]
    fn daily_purchases_are_cumulative_and_follow_same_length_rewrites() {
        let prices = MemoryVec::<Day1, Option<Dollars>>::new([
            Some(Dollars::mint(100.0)),
            None,
            Some(Dollars::mint(200.0)),
        ]);
        let days = MemoryVec::<Height, Day1>::new([Day1::from(0), Day1::from(1), Day1::from(2)]);
        let cumulative = DcaSats::new(
            ReadableBoxedVec::new(prices.clone()),
            days.read_only_boxed_clone(),
        );

        let first = DcaSats::sats_at_price(Dollars::mint(100.0));
        let third = first + DcaSats::sats_at_price(Dollars::mint(200.0));
        assert_eq!(cumulative.collect().as_slice(), &[first, first, third]);

        prices.replace(0, Some(Dollars::mint(50.0)));
        let replaced = DcaSats::sats_at_price(Dollars::mint(50.0));
        let third = replaced + DcaSats::sats_at_price(Dollars::mint(200.0));
        assert_eq!(
            cumulative.collect().as_slice(),
            &[replaced, replaced, third]
        );
    }

    #[test]
    fn height_source_maps_days_and_carries_the_latest_available_total() {
        let prices = MemoryVec::<Day1, Option<Dollars>>::new([
            Some(Dollars::mint(100.0)),
            None,
            Some(Dollars::mint(200.0)),
        ]);
        let days = MemoryVec::<Height, Day1>::new([
            Day1::from(0),
            Day1::from(0),
            Day1::from(1),
            Day1::from(2),
            Day1::from(3),
        ]);
        let cumulative = DcaSats::new(ReadableBoxedVec::new(prices), days.read_only_boxed_clone());

        let first = DcaSats::sats_at_price(Dollars::mint(100.0));
        let third = first + DcaSats::sats_at_price(Dollars::mint(200.0));
        assert_eq!(cumulative.collect(), [first, first, first, third, third]);
        assert_eq!(cumulative.collect_one_at(3), Some(third));
        assert_eq!(cumulative.collect_one_at(5), None);
        assert_eq!(
            cumulative.read_sorted_at(&[0, 2, 2, 4, 5]),
            [first, first, first, third],
        );
    }
}
