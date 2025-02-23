use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, path::Path};

use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::io::Serialization;

use super::{HeightMap, MapChunkId, MapKey, MapSerialized, MapValue};

#[derive(Debug, Default, Serialize, Deserialize, Encode, Decode, Allocative)]
pub struct SerializedVec<Value> {
    version: u32,
    pub map: Vec<Value>,
}

impl<Value> SerializedVec<Value> {
    pub fn get_index(&self, index: usize) -> Option<&Value> {
        self.map.get(index)
    }
}

impl<Key, Value, ChunkId> MapSerialized<Key, Value, ChunkId> for SerializedVec<Value>
where
    Self: Debug + Serialize + DeserializeOwned + Encode + Decode,
    ChunkId: MapChunkId,
    Key: MapKey<ChunkId>,
    Value: MapValue,
{
    #[inline(always)]
    fn new(version: u32) -> Self {
        Self {
            version,
            map: vec![],
        }
    }

    fn get_last_key(&self, chunk_id: &ChunkId) -> Option<Key> {
        Some(Key::from_usize(chunk_id.to_usize() + self.map.len()))
    }

    #[inline(always)]
    fn version(&self) -> u32 {
        self.version
    }

    #[inline(always)]
    fn get(&self, serialized_key: &Key) -> Option<&Value> {
        self.map.get(serialized_key.to_usize())
    }

    #[inline(always)]
    fn last(&self) -> Option<&Value> {
        self.map.last()
    }

    fn extend(&mut self, map: BTreeMap<Key, Value>) {
        map.into_iter().for_each(|(key, value)| {
            let key = key.to_serialized_key().to_usize();

            match self.map.len().cmp(&key) {
                Ordering::Greater => self.map[key] = value,
                Ordering::Equal => self.map.push(value),
                Ordering::Less => {
                    dbg!(&self.map, key, value);
                    panic!()
                }
            }
        });
    }

    fn import_all(path: &Path, serialization: &Serialization) -> Self
    where
        Self: Debug + Serialize + DeserializeOwned + Encode + Decode,
        Value: MapValue,
    {
        let mut s = None;

        HeightMap::<usize>::_read_dir(path, serialization)
            .iter()
            .for_each(|(_, path)| {
                let mut map = serialization.import::<Self>(path).unwrap();

                if s.is_none() {
                    s.replace(map);
                } else {
                    #[allow(clippy::unnecessary_unwrap)]
                    s.as_mut().unwrap().map.append(&mut map.map);
                }
            });

        s.unwrap()
    }

    fn to_csv(self, id: &str) -> String {
        let mut csv = format!("{},{}\n", Key::map_name(), id);
        self.map.iter().enumerate().for_each(|(k, v)| {
            csv += &format!("{:?},{:?}\n", k, v);
        });
        csv
    }

    fn map(&self) -> &impl Serialize {
        &self.map
    }
}
