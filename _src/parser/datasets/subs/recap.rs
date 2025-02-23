use std::{iter::Sum, ops::Add};

use allocative::Allocative;

use crate::{
    structs::{
        Date, DateMapChunkId, GenericMap, MapChunkId, MapKey, MapKind, MapPath, MapSerialized,
        MapValue, SerializedDateMap,
    },
    utils::{get_percentile, LossyFrom},
};

pub type DateRecapDataset<T> = RecapDataset<Date, T, DateMapChunkId, SerializedDateMap<T>>;

#[derive(Allocative)]
pub struct RecapDataset<Key, Value, ChunkId, Serialized> {
    average: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    sum: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    max: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    _90p: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    _75p: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    median: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    _25p: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    _10p: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
    min: Option<GenericMap<Key, Value, ChunkId, Serialized>>,
}

#[derive(Default)]
pub struct RecapOptions {
    average: bool,
    sum: bool,
    max: bool,
    _90p: bool,
    _75p: bool,
    median: bool,
    _25p: bool,
    _10p: bool,
    min: bool,
}

impl RecapOptions {
    pub fn add_min(mut self) -> Self {
        self.min = true;
        self
    }

    pub fn add_max(mut self) -> Self {
        self.max = true;
        self
    }

    pub fn add_median(mut self) -> Self {
        self.median = true;
        self
    }

    pub fn add_average(mut self) -> Self {
        self.average = true;
        self
    }

    #[allow(unused)]
    pub fn add_sum(mut self) -> Self {
        self.sum = true;
        self
    }

    pub fn add_90p(mut self) -> Self {
        self._90p = true;
        self
    }

    pub fn add_75p(mut self) -> Self {
        self._75p = true;
        self
    }

    pub fn add_25p(mut self) -> Self {
        self._25p = true;
        self
    }

    pub fn add_10p(mut self) -> Self {
        self._10p = true;
        self
    }
}

impl<Key, Value, ChunkId, Serialized> RecapDataset<Key, Value, ChunkId, Serialized>
where
    Value: MapValue,
    ChunkId: MapChunkId,
    Key: MapKey<ChunkId>,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    pub fn import(path: &MapPath, options: RecapOptions) -> color_eyre::Result<Self> {
        let f = |s: &str| path.join(s);

        let s = Self {
            // ---
            // Computed
            // ---
            min: options
                .min
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("min"))),
            max: options
                .max
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("max"))),
            median: options
                .median
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("median"))),
            average: options
                .average
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("average"))),
            sum: options
                .sum
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("sum"))),
            _90p: options
                ._90p
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("90p"))),
            _75p: options
                ._75p
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("75p"))),
            _25p: options
                ._25p
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("25p"))),
            _10p: options
                ._10p
                .then(|| GenericMap::new_bin(1, MapKind::Computed, &f("10p"))),
        };

        Ok(s)
    }

    pub fn compute<'a, Value2>(&mut self, key: Key, values: &'a mut [Value2])
    where
        Value: LossyFrom<f32> + LossyFrom<Value2>,
        Value2: Sum<&'a Value2> + Ord + Add<Output = Value2> + Clone + Copy + LossyFrom<f32>,
        f32: LossyFrom<Value> + LossyFrom<Value2>,
    {
        if self.max.is_some()
            || self._90p.is_some()
            || self._75p.is_some()
            || self.median.is_some()
            || self._25p.is_some()
            || self._10p.is_some()
            || self.min.is_some()
        {
            values.sort_unstable();

            if let Some(max) = self.max.as_mut() {
                max.insert_computed(key, Value::lossy_from(*values.last().unwrap()));
            }

            if let Some(_90p) = self._90p.as_mut() {
                _90p.insert_computed(key, Value::lossy_from(get_percentile(values, 0.90)));
            }

            if let Some(_75p) = self._75p.as_mut() {
                _75p.insert_computed(key, Value::lossy_from(get_percentile(values, 0.75)));
            }

            if let Some(median) = self.median.as_mut() {
                median.insert_computed(key, Value::lossy_from(get_percentile(values, 0.50)));
            }

            if let Some(_25p) = self._25p.as_mut() {
                _25p.insert_computed(key, Value::lossy_from(get_percentile(values, 0.25)));
            }

            if let Some(_10p) = self._10p.as_mut() {
                _10p.insert_computed(key, Value::lossy_from(get_percentile(values, 0.10)));
            }

            if let Some(min) = self.min.as_mut() {
                min.insert_computed(key, Value::lossy_from(*values.first().unwrap()));
            }
        }

        if self.sum.is_some() || self.average.is_some() {
            let sum = Value::lossy_from(values.iter().sum::<Value2>());

            if let Some(sum_map) = self.sum.as_mut() {
                sum_map.insert_computed(key, sum);
            }

            if let Some(average) = self.average.as_mut() {
                let len = values.len() as f32;
                average.insert_computed(key, Value::lossy_from(f32::lossy_from(sum) / len));
            }
        }
    }

    pub fn as_vec(&self) -> Vec<&GenericMap<Key, Value, ChunkId, Serialized>> {
        let mut v = vec![];

        if let Some(min) = self.min.as_ref() {
            v.push(min);
        }

        if let Some(max) = self.max.as_ref() {
            v.push(max);
        }

        if let Some(median) = self.median.as_ref() {
            v.push(median);
        }

        if let Some(average) = self.average.as_ref() {
            v.push(average);
        }

        if let Some(sum) = self.sum.as_ref() {
            v.push(sum);
        }

        if let Some(_90p) = self._90p.as_ref() {
            v.push(_90p);
        }

        if let Some(_75p) = self._75p.as_ref() {
            v.push(_75p);
        }

        if let Some(_25p) = self._25p.as_ref() {
            v.push(_25p);
        }

        if let Some(_10p) = self._10p.as_ref() {
            v.push(_10p);
        }

        v
    }

    pub fn as_mut_vec(&mut self) -> Vec<&mut GenericMap<Key, Value, ChunkId, Serialized>> {
        let mut v = vec![];

        if let Some(min) = self.min.as_mut() {
            v.push(min);
        }

        if let Some(max) = self.max.as_mut() {
            v.push(max);
        }

        if let Some(median) = self.median.as_mut() {
            v.push(median);
        }

        if let Some(average) = self.average.as_mut() {
            v.push(average);
        }

        if let Some(sum) = self.sum.as_mut() {
            v.push(sum);
        }

        if let Some(_90p) = self._90p.as_mut() {
            v.push(_90p);
        }

        if let Some(_75p) = self._75p.as_mut() {
            v.push(_75p);
        }

        if let Some(_25p) = self._25p.as_mut() {
            v.push(_25p);
        }

        if let Some(_10p) = self._10p.as_mut() {
            v.push(_10p);
        }

        v
    }
}
