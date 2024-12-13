use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
    fs,
    iter::Sum,
    mem,
    ops::{Add, ControlFlow, Div, Mul, Sub},
    path::{Path, PathBuf},
};

use allocative::Allocative;
use bincode::{Decode, Encode};
use itertools::Itertools;
use ordered_float::OrderedFloat;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    io::Serialization,
    utils::{get_percentile, LossyFrom},
};

use super::{AnyMap, MapPath, MapValue};

#[derive(Debug, Clone, Copy, Allocative, PartialEq, Eq)]
pub enum MapKind {
    Inserted,
    Computed,
}

pub trait MapKey<ChunkId>
where
    Self: Sized + PartialOrd + Ord + Clone + Copy + Debug,
    ChunkId: MapChunkId,
{
    fn to_chunk_id(&self) -> ChunkId;
    fn to_first_unsafe(&self) -> Option<Self>;
    fn to_serialized_key(&self) -> Self;
    fn is_out_of_bounds(&self) -> bool;
    fn is_first(&self) -> bool;
    fn checked_sub(&self, x: usize) -> Option<Self>;
    fn min_percentile_key() -> Self;
    fn iter_up_to(&self, other: &Self) -> impl Iterator<Item = Self>;
    fn map_name<'a>() -> &'a str;

    fn from_usize(_: usize) -> Self {
        unreachable!()
    }
    fn to_usize(&self) -> usize {
        unreachable!()
    }
}

pub trait MapSerialized<Key, Value, ChunkId>
where
    Self: Debug + Serialize + DeserializeOwned + Encode + Decode,
    ChunkId: MapChunkId,
{
    fn new(version: u32) -> Self;
    fn get_last_key(&self, last_chunk_id: &ChunkId) -> Option<Key>;
    fn version(&self) -> u32;
    fn get(&self, serialized_key: &Key) -> Option<&Value>;
    fn last(&self) -> Option<&Value>;
    fn extend(&mut self, map: BTreeMap<Key, Value>);
}

pub trait MapChunkId
where
    Self: Ord + Debug + Copy + Clone,
{
    fn to_name(&self) -> String;
    fn from_path(path: &Path) -> color_eyre::Result<Self>;
    fn to_usize(self) -> usize;
    fn from_usize(id: usize) -> Self;
}

#[derive(Debug, Allocative)]
pub struct GenericMap<Key, Value, ChunkId, Serialized> {
    version: u32,
    kind: MapKind,

    path_all: MapPath,
    path_last: Option<MapPath>,

    chunks_in_memory: usize,

    serialization: Serialization,

    pub initial_last_key: Option<Key>,
    pub initial_first_unsafe_key: Option<Key>,

    imported: BTreeMap<ChunkId, Serialized>,
    to_insert: BTreeMap<ChunkId, BTreeMap<Key, Value>>,
}

impl<Key, Value, ChunkId, Serialized> GenericMap<Key, Value, ChunkId, Serialized>
where
    Value: MapValue,
    ChunkId: MapChunkId,
    Key: MapKey<ChunkId>,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    pub fn new_bin(version: u32, kind: MapKind, path: &MapPath) -> Self {
        Self::new(version, kind, path, Serialization::Binary, 1, true)
    }

    pub fn _new_bin(version: u32, kind: MapKind, path: &MapPath, export_last: bool) -> Self {
        Self::new(version, kind, path, Serialization::Binary, 1, export_last)
    }

    pub fn new_json(version: u32, kind: MapKind, path: &MapPath, export_last: bool) -> Self {
        Self::new(
            version,
            kind,
            path,
            Serialization::Json,
            usize::MAX,
            export_last,
        )
    }

    fn new(
        version: u32,
        kind: MapKind,
        path: &MapPath,
        serialization: Serialization,
        chunks_in_memory: usize,
        export_last: bool,
    ) -> Self {
        if chunks_in_memory < 1 {
            panic!("Should always have at least the latest chunk in memory");
        }

        let path_all = path.join(Key::map_name());

        fs::create_dir_all(&*path_all).unwrap_or_else(|_| {
            dbg!(&path_all);
            panic!()
        });

        let path_last = {
            if export_last {
                Some(path.join("last"))
            } else {
                None
            }
        };

        let mut s = Self {
            version,
            kind,

            path_all,
            path_last,

            chunks_in_memory,

            serialization,

            initial_last_key: None,
            initial_first_unsafe_key: None,

            to_insert: BTreeMap::default(),
            imported: BTreeMap::default(),
        };

        s.read_dir()
            .into_iter()
            .rev()
            .take(chunks_in_memory)
            .for_each(|(chunk_start, path)| {
                if let Ok(serialized) = s.import(&path) {
                    if serialized.version() == s.version {
                        s.imported.insert(chunk_start, serialized);
                    } else {
                        s.delete_files();
                    }
                }
            });

        s.initial_last_key = s
            .imported
            .iter()
            .last()
            .and_then(|(last_chunk_id, serialized)| serialized.get_last_key(last_chunk_id));

        s.initial_first_unsafe_key = s
            .initial_last_key
            .and_then(|last_key| last_key.to_first_unsafe());

        // if s.initial_first_unsafe_key.is_none() {
        //     log(&format!("Missing dataset: {path:?}/{}", Key::map_name()));
        // }

        s
    }

    fn read_dir(&self) -> BTreeMap<ChunkId, PathBuf> {
        Self::_read_dir(&self.path_all, &self.serialization)
    }

    pub fn _read_dir(path: &Path, serialization: &Serialization) -> BTreeMap<ChunkId, PathBuf> {
        fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| serialization.is_serializable(path))
            .flat_map(|path| {
                if let Ok(chunk_id) = ChunkId::from_path(&path) {
                    Some((chunk_id, path))
                } else {
                    None
                }
            })
            .collect()
    }

    fn import(&self, path: &Path) -> color_eyre::Result<Serialized> {
        self.serialization.import::<Serialized>(path)
    }

    pub fn insert(&mut self, key: Key, value: Value) -> Value {
        self.checked_insert(key, value, MapKind::Inserted)
    }

    pub fn insert_computed(&mut self, key: Key, value: Value) -> Value {
        self.checked_insert(key, value, MapKind::Computed)
    }

    fn checked_insert(&mut self, key: Key, value: Value, kind: MapKind) -> Value {
        if self.kind != kind {
            dbg!(&self.path());
            panic!("Called at the wrong place");
        }

        if !self.is_key_safe(key) {
            self.to_insert
                .entry(key.to_chunk_id())
                .or_default()
                .insert(key.to_serialized_key(), value);
        }

        value
    }

    pub fn insert_default(&mut self, key: Key) -> Value {
        self.insert(key, Value::default())
    }

    #[inline(always)]
    pub fn is_key_safe(&self, key: Key) -> bool {
        self.initial_first_unsafe_key
            .map_or(false, |initial_first_unsafe_key| {
                initial_first_unsafe_key > key
            })
    }

    pub fn get(&self, key: &Key) -> Option<Value> {
        let chunk_id = key.to_chunk_id();

        let serialized_key = key.to_serialized_key();

        self.to_insert
            .get(&chunk_id)
            .and_then(|tree| tree.get(&serialized_key).cloned())
            .or_else(|| {
                self.imported
                    .get(&chunk_id)
                    .and_then(|serialized| serialized.get(&serialized_key))
                    .cloned()
            })
    }

    pub fn get_or_import(&mut self, key: &Key) -> Option<Value> {
        if key.is_out_of_bounds() {
            return None;
        }

        let chunk_id = key.to_chunk_id();

        let serialized_key = key.to_serialized_key();

        self.to_insert
            .get(&chunk_id)
            .and_then(|tree| tree.get(&serialized_key).cloned())
            .or_else(|| {
                #[allow(clippy::map_entry)] // Can't be mut and then use read_dir()
                if !self.imported.contains_key(&chunk_id) {
                    let dir_content = self.read_dir();

                    if let Some(path) = dir_content.get(&chunk_id) {
                        let serialized = self.import(path).unwrap();

                        self.imported.insert(chunk_id, serialized);
                    }
                }

                self.imported
                    .get(&chunk_id)
                    .and_then(|serialized| serialized.get(&serialized_key))
                    .cloned()
            })
    }
}

impl<Key, Value, ChunkId, Serialized> AnyMap for GenericMap<Key, Value, ChunkId, Serialized>
where
    Value: MapValue,
    ChunkId: MapChunkId,
    Key: MapKey<ChunkId>,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    fn path(&self) -> &Path {
        &self.path_all
    }

    fn path_last(&self) -> &Option<MapPath> {
        &self.path_last
    }

    fn last_value(&self) -> Option<serde_json::Value> {
        self.imported
            .last_key_value()
            .and_then(|(_, serialized)| serialized.last())
            .and_then(|v| serde_json::to_value(v).ok())
    }

    fn t_name(&self) -> &str {
        std::any::type_name::<Value>()
    }

    fn pre_export(&mut self) {
        self.to_insert.iter_mut().for_each(|(chunk_id, map)| {
            if let Some((key, _)) = map.first_key_value() {
                if !key.is_first() && !self.imported.contains_key(chunk_id) {
                    // Had to copy paste many lines from functions as calling a function from self isn't allowed because of the &mut

                    let dir_content = Self::_read_dir(&self.path_all, &self.serialization);

                    let path = dir_content.get(chunk_id).unwrap_or_else(|| {
                        dbg!(&self.path_all, chunk_id, &dir_content);
                        panic!();
                    });

                    let serialized = self.serialization.import::<Serialized>(path).unwrap();

                    self.imported.insert(*chunk_id, serialized);
                }
            }

            self.imported
                .entry(*chunk_id)
                .or_insert(Serialized::new(self.version))
                .extend(mem::take(map));
        });
    }

    fn export(&self) -> color_eyre::Result<()> {
        let len = self.imported.len();

        self.to_insert.iter().enumerate().try_for_each(
            |(index, (chunk_id, map))| -> color_eyre::Result<()> {
                if !map.is_empty() {
                    unreachable!()
                }

                let serialized = self.imported.get(chunk_id).unwrap_or_else(|| {
                    dbg!(&self.path_all, chunk_id, &self.imported);
                    panic!();
                });

                let path = self.path_all.join(&chunk_id.to_name());

                self.serialization.export(&path, serialized)?;

                // Export last
                if index == len - 1 {
                    if let Some(path_last) = self.path_last.as_ref() {
                        self.serialization
                            .export(path_last, serialized.last().unwrap())?;
                    }
                }

                Ok(())
            },
        )
    }

    fn post_export(&mut self) {
        self.imported
            .keys()
            .rev()
            .enumerate()
            .filter(|(index, _)| *index + 1 > self.chunks_in_memory)
            .map(|(_, key)| *key)
            .collect_vec()
            .iter()
            .for_each(|key| {
                self.imported.remove(key);
            });

        self.to_insert.clear();
    }

    fn delete_files(&self) {
        self.read_dir()
            .iter()
            .for_each(|(_, path)| fs::remove_file(path).unwrap())
    }

    fn kind(&self) -> MapKind {
        self.kind
    }
}

impl<Key, Value, ChunkId, Serialized> GenericMap<Key, Value, ChunkId, Serialized>
where
    Value: MapValue,
    ChunkId: MapChunkId,
    Key: MapKey<ChunkId>,
    Serialized: MapSerialized<Key, Value, ChunkId>,
{
    // pub fn sum_keys(&mut self, keys: &[Key]) -> Value
    // where
    //     Value: Sum,
    // {
    //     keys.iter()
    //         .map(|key| self.get_or_import(key).unwrap())
    //         .sum::<Value>()
    // }

    // pub fn average_keys(&mut self, keys: &[Key]) -> f32
    // where
    //     Value: Sum,
    //     f32: LossyFrom<Value>,
    // {
    //     f32::lossy_from(self.sum_keys(keys)) / keys.len() as f32
    // }

    pub fn multi_insert<F>(&mut self, keys: &[Key], mut callback: F)
    where
        F: FnMut(&Key) -> Value,
    {
        keys.iter().for_each(|key| {
            self.insert_computed(*key, callback(key));
        });
    }

    pub fn multi_insert_const(&mut self, keys: &[Key], constant: Value) {
        keys.iter().for_each(|key| {
            self.insert_computed(*key, constant);
        });
    }

    pub fn multi_insert_simple_transform<SourceValue, SourceSerialized, F>(
        &mut self,
        keys: &[Key],
        source: &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
        mut transform: F,
    ) where
        SourceValue: MapValue,
        SourceSerialized: MapSerialized<Key, SourceValue, ChunkId>,
        F: FnMut(SourceValue, &Key) -> Value,
    {
        keys.iter().for_each(|key| {
            self.insert_computed(*key, transform(source.get_or_import(key).unwrap(), key));
        });
    }

    pub fn multi_insert_complex_transform<SourceValue, SourceSerialized, F>(
        &mut self,
        keys: &[Key],
        source: &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
        mut transform: F,
    ) where
        SourceValue: MapValue,
        SourceSerialized: MapSerialized<Key, SourceValue, ChunkId>,
        F: FnMut(
            (
                SourceValue,
                &Key,
                &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
                &mut Self,
            ),
        ) -> Value,
    {
        keys.iter().for_each(|key| {
            let value = transform((source.get_or_import(key).unwrap(), key, source, self));

            self.insert_computed(*key, value);
        });
    }

    pub fn multi_insert_add<A, B, ASerialized, BSerialized>(
        &mut self,
        keys: &[Key],
        added: &mut GenericMap<Key, A, ChunkId, ASerialized>,
        adder: &mut GenericMap<Key, B, ChunkId, BSerialized>,
    ) where
        A: MapValue,
        ASerialized: MapSerialized<Key, A, ChunkId>,
        B: MapValue,
        BSerialized: MapSerialized<Key, B, ChunkId>,
        Value: LossyFrom<A> + LossyFrom<B> + Add<Output = Value>,
    {
        keys.iter().for_each(|key| {
            self.insert_computed(
                *key,
                Value::lossy_from(added.get_or_import(key).unwrap())
                    + Value::lossy_from(adder.get_or_import(key).unwrap()),
            );
        });
    }

    pub fn multi_insert_subtract<A, B, ASerialized, BSerialized>(
        &mut self,
        keys: &[Key],
        subtracted: &mut GenericMap<Key, A, ChunkId, ASerialized>,
        subtracter: &mut GenericMap<Key, B, ChunkId, BSerialized>,
    ) where
        A: MapValue,
        ASerialized: MapSerialized<Key, A, ChunkId>,
        B: MapValue,
        BSerialized: MapSerialized<Key, B, ChunkId>,
        Value: LossyFrom<A> + LossyFrom<B> + Sub<Output = Value>,
    {
        keys.iter().for_each(|key| {
            self.insert_computed(
                *key,
                Value::lossy_from(subtracted.get_or_import(key).unwrap())
                    - Value::lossy_from(subtracter.get_or_import(key).unwrap()),
            );
        });
    }

    pub fn multi_insert_multiply<A, B, ASerialized, BSerialized>(
        &mut self,
        keys: &[Key],
        multiplied: &mut GenericMap<Key, A, ChunkId, ASerialized>,
        multiplier: &mut GenericMap<Key, B, ChunkId, BSerialized>,
    ) where
        A: MapValue,
        ASerialized: MapSerialized<Key, A, ChunkId>,
        B: MapValue,
        BSerialized: MapSerialized<Key, B, ChunkId>,
        Value: LossyFrom<A> + LossyFrom<B> + Mul<Output = Value>,
    {
        keys.iter().for_each(|key| {
            self.insert_computed(
                *key,
                Value::lossy_from(multiplied.get_or_import(key).unwrap())
                    * Value::lossy_from(multiplier.get_or_import(key).unwrap()),
            );
        });
    }

    pub fn multi_insert_divide<A, B, ASerialized, BSerialized>(
        &mut self,
        keys: &[Key],
        divided: &mut GenericMap<Key, A, ChunkId, ASerialized>,
        divider: &mut GenericMap<Key, B, ChunkId, BSerialized>,
    ) where
        A: MapValue,
        ASerialized: MapSerialized<Key, A, ChunkId>,
        B: MapValue,
        BSerialized: MapSerialized<Key, B, ChunkId>,
        Value: LossyFrom<A> + LossyFrom<B> + Div<Output = Value> + Mul<Output = Value> + From<u8>,
    {
        self._multi_insert_divide(keys, divided, divider, false)
    }

    pub fn multi_insert_percentage<A, B, ASerialized, BSerialized>(
        &mut self,
        keys: &[Key],
        divided: &mut GenericMap<Key, A, ChunkId, ASerialized>,
        divider: &mut GenericMap<Key, B, ChunkId, BSerialized>,
    ) where
        A: MapValue,
        ASerialized: MapSerialized<Key, A, ChunkId>,
        B: MapValue,
        BSerialized: MapSerialized<Key, B, ChunkId>,
        Value: LossyFrom<A> + LossyFrom<B> + Div<Output = Value> + Mul<Output = Value> + From<u8>,
    {
        self._multi_insert_divide(keys, divided, divider, true)
    }

    fn _multi_insert_divide<A, B, ASerialized, BSerialized>(
        &mut self,
        keys: &[Key],
        divided: &mut GenericMap<Key, A, ChunkId, ASerialized>,
        divider: &mut GenericMap<Key, B, ChunkId, BSerialized>,
        as_percentage: bool,
    ) where
        A: MapValue,
        ASerialized: MapSerialized<Key, A, ChunkId>,
        B: MapValue,
        BSerialized: MapSerialized<Key, B, ChunkId>,
        Value: LossyFrom<A> + LossyFrom<B> + Div<Output = Value> + Mul<Output = Value> + From<u8>,
    {
        let multiplier = Value::from(if as_percentage { 100 } else { 1 });

        keys.iter().for_each(|key| {
            self.insert_computed(
                *key,
                Value::lossy_from(divided.get_or_import(key).unwrap())
                    / Value::lossy_from(divider.get_or_import(key).unwrap())
                    * multiplier,
            );
        });
    }

    pub fn multi_insert_cumulative<SourceValue, SourceSerialized>(
        &mut self,
        keys: &[Key],
        source: &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
    ) where
        SourceValue: MapValue,
        SourceSerialized: MapSerialized<Key, SourceValue, ChunkId>,
        Value: LossyFrom<SourceValue> + Add<Output = Value> + Sub<Output = Value>,
    {
        self._multi_insert_last_x_sum(keys, source, None)
    }

    pub fn multi_insert_last_x_sum<SourceValue, SourceSerialized>(
        &mut self,
        keys: &[Key],
        source: &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
        len: usize,
    ) where
        SourceValue: MapValue,
        SourceSerialized: MapSerialized<Key, SourceValue, ChunkId>,
        Value: LossyFrom<SourceValue> + Add<Output = Value> + Sub<Output = Value>,
    {
        self._multi_insert_last_x_sum(keys, source, Some(len))
    }

    fn _multi_insert_last_x_sum<SourceValue, SourceSerialized>(
        &mut self,
        keys: &[Key],
        source: &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
        len: Option<usize>,
    ) where
        SourceValue: MapValue,
        SourceSerialized: MapSerialized<Key, SourceValue, ChunkId>,
        Value: LossyFrom<SourceValue> + Add<Output = Value> + Sub<Output = Value>,
    {
        let mut sum = None;

        keys.iter().for_each(|key| {
            let to_subtract = len
                .and_then(|x| {
                    key.checked_sub(x)
                        .and_then(|previous_key| source.get_or_import(&previous_key))
                })
                .unwrap_or_default();

            let previous_sum = sum.unwrap_or_else(|| {
                key.checked_sub(1)
                    .and_then(|previous_sum_key| self.get_or_import(&previous_sum_key))
                    .unwrap_or_default()
            });

            let last_value = source.get_or_import(key).unwrap_or_else(|| {
                dbg!(&source.to_insert, &source.path(), key);
                panic!();
            });

            sum.replace(
                previous_sum + Value::lossy_from(last_value) - Value::lossy_from(to_subtract),
            );

            self.insert_computed(*key, sum.unwrap());
        });
    }

    pub fn multi_insert_simple_average<SourceValue, SourceSerialized>(
        &mut self,
        keys: &[Key],
        source: &mut GenericMap<Key, SourceValue, ChunkId, SourceSerialized>,
        len: usize,
    ) where
        SourceValue: MapValue + Sum,
        SourceSerialized: MapSerialized<Key, SourceValue, ChunkId>,
        Value: Into<f32> + From<f32>,
        f32: LossyFrom<SourceValue>,
    {
        if len <= 1 {
            panic!("Average of 1 or less is not useful");
        }

        let len = len as f32;

        let mut average = None;

        keys.iter().for_each(|key| {
            let mut previous_average: f32 = average
                .unwrap_or_else(|| {
                    key.checked_sub(1)
                        .and_then(|previous_average_key| self.get_or_import(&previous_average_key))
                        .unwrap_or_default()
                })
                .into();

            if previous_average.is_nan() || previous_average.is_infinite() {
                previous_average = 0.0;
            }

            let mut last_value = f32::lossy_from(source.get_or_import(key).unwrap_or_else(|| {
                dbg!(key);
                panic!()
            }));

            if last_value.is_nan() || last_value.is_infinite() {
                last_value = 0.0;
            }

            average.replace(((previous_average * (len - 1.0) + last_value) / len).into());

            self.insert_computed(*key, average.unwrap());
        });
    }

    pub fn multi_insert_net_change(&mut self, keys: &[Key], source: &mut Self, len: usize)
    where
        Value: Sub<Output = Value>,
    {
        keys.iter().for_each(|key| {
            let previous_value = key
                .checked_sub(len)
                .and_then(|previous_key| source.get_or_import(&previous_key))
                .unwrap_or_default();

            let last_value = source.get_or_import(key).unwrap();

            let net_change = last_value - previous_value;

            self.insert_computed(*key, net_change);
        });
    }

    pub fn multi_insert_percentage_change(&mut self, keys: &[Key], source: &mut Self, len: usize)
    where
        Value: Sub<Output = Value> + LossyFrom<f32>,
        f32: LossyFrom<Value>,
    {
        let one = 1.0;
        let hundred = 100.0;

        keys.iter().for_each(|key| {
            let previous_value = f32::lossy_from(
                key.checked_sub(len)
                    .and_then(|previous_key| source.get_or_import(&previous_key))
                    .unwrap_or_default(),
            );

            let last_value = f32::lossy_from(source.get_or_import(key).unwrap());

            let percentage_change = ((last_value / previous_value) - one) * hundred;

            self.insert_computed(*key, Value::lossy_from(percentage_change));
        });
    }

    pub fn multi_insert_median(&mut self, keys: &[Key], source: &mut Self, len: Option<usize>)
    where
        Value: LossyFrom<f32>,
        f32: LossyFrom<Value>,
    {
        source.multi_insert_percentile(keys, vec![(self, 0.5)], len);
    }

    pub fn multi_insert_percentile(
        &mut self,
        keys: &[Key],
        mut map_and_percentiles: Vec<(&mut Self, f32)>,
        len: Option<usize>,
    ) where
        Value: LossyFrom<f32>,
        f32: LossyFrom<Value>,
    {
        if len.map_or(false, |size| size < 3) {
            panic!("Computing a percentile for a size lower than 3 is useless");
        }

        let mut ordered_vec = None;
        let mut sorted_vec = None;

        let min_percentile_key = Key::min_percentile_key();

        let nan = Value::lossy_from(f32::NAN);

        keys.iter().cloned().try_for_each(|key| {
            if key < min_percentile_key {
                map_and_percentiles.iter_mut().for_each(|(map, _)| {
                    (*map).insert_computed(key, nan);
                });
                return ControlFlow::Continue::<()>(());
            }

            if let Some(start) = len.map_or(Some(min_percentile_key), |size| key.checked_sub(size))
            {
                if sorted_vec.is_none() {
                    let mut vec = start
                        .iter_up_to(&key)
                        .flat_map(|key| self.get_or_import(&key))
                        .map(|v| f32::lossy_from(v))
                        .filter(|f| !f.is_nan())
                        .map(OrderedFloat)
                        .collect_vec();

                    if len.is_some() {
                        ordered_vec.replace(VecDeque::from(vec.clone()));
                    }

                    vec.sort_unstable();

                    sorted_vec.replace(vec);
                } else {
                    let float_value = f32::lossy_from(self.get_or_import(&key).unwrap());

                    if !float_value.is_nan() {
                        let float_value = OrderedFloat(float_value);

                        if let Some(len) = len {
                            if let Some(ordered_vec) = ordered_vec.as_mut() {
                                if ordered_vec.len() == len {
                                    let first = ordered_vec.pop_front().unwrap();

                                    let pos =
                                        sorted_vec.as_ref().unwrap().binary_search(&first).unwrap();

                                    sorted_vec.as_mut().unwrap().remove(pos);
                                }

                                ordered_vec.push_back(float_value);
                            }
                        }

                        let pos = sorted_vec
                            .as_ref()
                            .unwrap()
                            .binary_search(&float_value)
                            .unwrap_or_else(|pos| pos);

                        sorted_vec.as_mut().unwrap().insert(pos, float_value);
                    }
                }

                let vec = sorted_vec.as_ref().unwrap();

                map_and_percentiles
                    .iter_mut()
                    .for_each(|(map, percentile)| {
                        if !(0.0..=1.0).contains(percentile) {
                            panic!("The percentile should be between 0.0 and 1.0");
                        }

                        let float_value = get_percentile::<OrderedFloat<f32>>(vec, *percentile).0;

                        (*map).insert_computed(key, Value::lossy_from(float_value));
                    });
            } else {
                map_and_percentiles.iter_mut().for_each(|(map, _)| {
                    (*map).insert_computed(key, nan);
                });
            }

            ControlFlow::Continue(())
        });
    }

    pub fn multi_insert_max(&mut self, keys: &[Key], source: &mut Self)
    where
        Value: Default + PartialOrd,
    {
        let mut previous_max = None;

        keys.iter().for_each(|key| {
            if previous_max.is_none() {
                key.checked_sub(1)
                    .and_then(|previous_max_key| self.get_or_import(&previous_max_key))
                    .and_then(|v| previous_max.replace(v));
            }

            let last_value = source.get_or_import(key).unwrap_or_else(|| {
                dbg!(key);
                panic!()
            });

            if previous_max.is_none()
                || previous_max.is_some_and(|previous_max| previous_max < last_value)
            {
                previous_max.replace(last_value);
            }

            self.insert_computed(*key, previous_max.unwrap());
        });
    }

    pub fn multi_insert_min(&mut self, keys: &[Key], source: &mut Self, min_excluded: Value)
    where
        Value: Default + PartialOrd,
    {
        let mut previous_min = None;

        keys.iter().for_each(|key| {
            if previous_min.is_none() {
                if let Some(value) = key
                    .checked_sub(1)
                    .and_then(|previous_min_key| self.get_or_import(&previous_min_key))
                {
                    if value > min_excluded {
                        previous_min.replace(value);
                    }
                }
            }

            let last_value = source.get_or_import(key).unwrap_or_else(|| {
                dbg!(key);
                panic!()
            });

            if last_value > min_excluded
                && (previous_min.is_none()
                    || previous_min.is_some_and(|previous_min| previous_min > last_value))
            {
                previous_min.replace(last_value);
            }

            self.insert_computed(*key, previous_min.unwrap_or_default());
        });
    }
}
