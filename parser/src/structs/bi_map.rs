use std::{
    iter::Sum,
    ops::{Add, Div, Mul, RangeInclusive, Sub},
};

use allocative::Allocative;

use crate::utils::{LossyFrom, TARGET_BLOCKS_PER_DAY};

use super::{AnyDateMap, AnyHeightMap, AnyMap, Date, DateMap, Height, HeightMap, MapValue};

#[derive(Default, Allocative)]
pub struct BiMap<Value>
where
    Value: MapValue,
{
    pub height: HeightMap<Value>,
    pub date: DateMap<Value>,
}

impl<Value> BiMap<Value>
where
    Value: MapValue,
{
    pub fn new_bin(version: u32, path: &str) -> Self {
        Self {
            height: HeightMap::_new_bin(version, path, true),
            date: DateMap::_new_bin(version, path, false),
        }
    }

    pub fn new_json(version: u32, path: &str) -> Self {
        Self {
            height: HeightMap::new_json(version, path, true),
            date: DateMap::new_json(version, path, false),
        }
    }

    pub fn date_insert_sum_range(&mut self, date: Date, date_blocks_range: &RangeInclusive<u32>)
    where
        Value: Sum,
    {
        self.date
            .insert(date, self.height.sum_range(date_blocks_range));
    }

    pub fn multi_date_insert_sum_range(
        &mut self,
        dates: &[Date],
        first_height: &mut DateMap<Height>,
        last_height: &mut DateMap<Height>,
    ) where
        Value: Sum,
    {
        dates.iter().for_each(|date| {
            let first_height = first_height.get_or_import(date).unwrap();
            let last_height = last_height.get_or_import(date).unwrap();
            let range = (*first_height)..=(*last_height);

            self.date.insert(*date, self.height.sum_range(&range));
        })
    }

    pub fn multi_insert_const(&mut self, heights: &[Height], dates: &[Date], constant: Value) {
        self.height.multi_insert_const(heights, constant);

        self.date.multi_insert_const(dates, constant);
    }

    pub fn multi_insert_simple_transform<F, K>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<K>,
        transform: &F,
    ) where
        Value: Div<Output = Value>,
        F: Fn(K) -> Value,
        K: MapValue,
    {
        self.height
            .multi_insert_simple_transform(heights, &mut source.height, transform);
        self.date
            .multi_insert_simple_transform(dates, &mut source.date, transform);
    }

    #[allow(unused)]
    pub fn multi_insert_add<A, B>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        added: &mut BiMap<A>,
        adder: &mut BiMap<B>,
    ) where
        A: MapValue,
        B: MapValue,
        Value: LossyFrom<A> + LossyFrom<B>,
        Value: Add<Output = Value>,
    {
        self.height
            .multi_insert_add(heights, &mut added.height, &mut adder.height);
        self.date
            .multi_insert_add(dates, &mut added.date, &mut adder.date);
    }

    pub fn multi_insert_subtract<A, B>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        subtracted: &mut BiMap<A>,
        subtracter: &mut BiMap<B>,
    ) where
        A: MapValue,
        B: MapValue,
        Value: LossyFrom<A> + LossyFrom<B>,
        Value: Sub<Output = Value>,
    {
        self.height
            .multi_insert_subtract(heights, &mut subtracted.height, &mut subtracter.height);

        self.date
            .multi_insert_subtract(dates, &mut subtracted.date, &mut subtracter.date);
    }

    pub fn multi_insert_multiply<A, B>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        multiplied: &mut BiMap<A>,
        multiplier: &mut BiMap<B>,
    ) where
        A: MapValue,
        B: MapValue,
        Value: LossyFrom<A> + LossyFrom<B>,
        Value: Mul<Output = Value>,
    {
        self.height
            .multi_insert_multiply(heights, &mut multiplied.height, &mut multiplier.height);
        self.date
            .multi_insert_multiply(dates, &mut multiplied.date, &mut multiplier.date);
    }

    pub fn multi_insert_divide<A, B>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        divided: &mut BiMap<A>,
        divider: &mut BiMap<B>,
    ) where
        A: MapValue,
        B: MapValue,
        Value: LossyFrom<A> + LossyFrom<B>,
        Value: Div<Output = Value> + Mul<Output = Value> + From<u8>,
    {
        self.height
            .multi_insert_divide(heights, &mut divided.height, &mut divider.height);
        self.date
            .multi_insert_divide(dates, &mut divided.date, &mut divider.date);
    }

    pub fn multi_insert_percentage<A, B>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        divided: &mut BiMap<A>,
        divider: &mut BiMap<B>,
    ) where
        A: MapValue,
        B: MapValue,
        Value: LossyFrom<A> + LossyFrom<B>,
        Value: Div<Output = Value> + Mul<Output = Value> + From<u8>,
    {
        self.height
            .multi_insert_percentage(heights, &mut divided.height, &mut divider.height);
        self.date
            .multi_insert_percentage(dates, &mut divided.date, &mut divider.date);
    }

    pub fn multi_insert_cumulative<K>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<K>,
    ) where
        K: MapValue,
        Value: LossyFrom<K>,
        Value: Add<Output = Value> + Sub<Output = Value>,
    {
        self.height
            .multi_insert_cumulative(heights, &mut source.height);

        self.date.multi_insert_cumulative(dates, &mut source.date);
    }

    pub fn multi_insert_last_x_sum<K>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<K>,
        days: usize,
    ) where
        K: MapValue,
        Value: LossyFrom<K>,
        Value: Add<Output = Value> + Sub<Output = Value>,
    {
        self.height.multi_insert_last_x_sum(
            heights,
            &mut source.height,
            TARGET_BLOCKS_PER_DAY * days,
        );

        self.date
            .multi_insert_last_x_sum(dates, &mut source.date, days);
    }

    pub fn multi_insert_simple_average<K>(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<K>,
        days: usize,
    ) where
        Value: Into<f32> + From<f32>,
        K: MapValue + Sum,
        f32: LossyFrom<K>,
    {
        self.height.multi_insert_simple_average(
            heights,
            &mut source.height,
            TARGET_BLOCKS_PER_DAY * days,
        );
        self.date
            .multi_insert_simple_average(dates, &mut source.date, days);
    }

    pub fn multi_insert_net_change(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<Value>,
        days: usize,
    ) where
        Value: Sub<Output = Value>,
    {
        self.height.multi_insert_net_change(
            heights,
            &mut source.height,
            TARGET_BLOCKS_PER_DAY * days,
        );
        self.date
            .multi_insert_net_change(dates, &mut source.date, days);
    }

    pub fn multi_insert_median(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<Value>,
        days: Option<usize>,
    ) where
        Value: LossyFrom<f32>,
        f32: LossyFrom<Value>,
    {
        self.height.multi_insert_median(
            heights,
            &mut source.height,
            days.map(|days| TARGET_BLOCKS_PER_DAY * days),
        );
        self.date.multi_insert_median(dates, &mut source.date, days);
    }

    #[allow(unused)]
    pub fn multi_insert_percentile(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        mut map_and_percentiles: Vec<(&mut BiMap<Value>, f32)>,
        days: Option<usize>,
    ) where
        Value: LossyFrom<f32>,
        f32: LossyFrom<Value>,
    {
        let mut date_map_and_percentiles = vec![];
        let mut height_map_and_percentiles = vec![];

        map_and_percentiles
            .iter_mut()
            .for_each(|(map, percentile)| {
                date_map_and_percentiles.push((&mut map.date, *percentile));
                height_map_and_percentiles.push((&mut map.height, *percentile));
            });

        self.height.multi_insert_percentile(
            heights,
            height_map_and_percentiles,
            days.map(|days| TARGET_BLOCKS_PER_DAY * days),
        );

        self.date
            .multi_insert_percentile(dates, date_map_and_percentiles, days);
    }

    pub fn multi_insert_max(
        &mut self,
        heights: &[Height],
        dates: &[Date],
        source: &mut BiMap<Value>,
    ) where
        Value: PartialOrd,
    {
        self.height.multi_insert_max(heights, &mut source.height);
        self.date.multi_insert_max(dates, &mut source.date);
    }
}

pub trait AnyBiMap {
    #[allow(unused)]
    fn as_any_map(&self) -> Vec<&(dyn AnyMap + Send + Sync)>;

    fn as_any_mut_map(&mut self) -> Vec<&mut dyn AnyMap>;

    fn get_height(&self) -> &(dyn AnyHeightMap + Send + Sync);

    #[allow(unused)]
    fn get_mut_height(&mut self) -> &mut dyn AnyHeightMap;

    fn get_date(&self) -> &(dyn AnyDateMap + Send + Sync);

    #[allow(unused)]
    fn get_mut_date(&mut self) -> &mut dyn AnyDateMap;
}

impl<T> AnyBiMap for BiMap<T>
where
    T: MapValue,
{
    fn as_any_map(&self) -> Vec<&(dyn AnyMap + Send + Sync)> {
        vec![self.date.as_any_map(), self.height.as_any_map()]
    }

    fn as_any_mut_map(&mut self) -> Vec<&mut dyn AnyMap> {
        vec![self.date.as_any_mut_map(), self.height.as_any_mut_map()]
    }

    fn get_height(&self) -> &(dyn AnyHeightMap + Send + Sync) {
        &self.height
    }

    fn get_mut_height(&mut self) -> &mut dyn AnyHeightMap {
        &mut self.height
    }

    fn get_date(&self) -> &(dyn AnyDateMap + Send + Sync) {
        &self.date
    }

    fn get_mut_date(&mut self) -> &mut dyn AnyDateMap {
        &mut self.date
    }
}
