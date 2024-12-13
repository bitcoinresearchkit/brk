use std::{iter::Sum, ops::RangeInclusive};

use itertools::Itertools;

use super::{AnyMap, GenericMap, Height, HeightMapChunkId, MapValue, SerializedVec};

pub const HEIGHT_MAP_CHUNK_SIZE: u32 = 10_000;

pub type HeightMap<Value> = GenericMap<Height, Value, HeightMapChunkId, SerializedVec<Value>>;

impl<Value> HeightMap<Value>
where
    Value: MapValue,
{
    pub fn sum_range(&self, range: &RangeInclusive<u32>) -> Value
    where
        Value: Sum,
    {
        range
            .to_owned()
            .flat_map(|height| self.get(&Height::new(height)))
            .sum::<Value>()
    }

    pub fn get_or_import_range_inclusive(&mut self, first: Height, last: Height) -> Vec<Value> {
        ((*first)..=(*last))
            .map(Height::new)
            .map(|h| self.get_or_import(&h).unwrap())
            .collect_vec()
    }
}

pub trait AnyHeightMap: AnyMap {
    fn get_initial_first_unsafe_height(&self) -> Option<Height>;

    fn get_initial_last_height(&self) -> Option<Height>;

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync);

    fn as_any_mut_map(&mut self) -> &mut dyn AnyMap;
}

impl<T> AnyHeightMap for HeightMap<T>
where
    T: MapValue,
{
    #[inline(always)]
    fn get_initial_first_unsafe_height(&self) -> Option<Height> {
        self.initial_first_unsafe_key
    }

    #[inline(always)]
    fn get_initial_last_height(&self) -> Option<Height> {
        self.initial_last_key
    }

    fn as_any_map(&self) -> &(dyn AnyMap + Send + Sync) {
        self
    }

    fn as_any_mut_map(&mut self) -> &mut dyn AnyMap {
        self
    }
}
