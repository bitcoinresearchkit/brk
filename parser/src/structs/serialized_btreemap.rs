use std::{collections::BTreeMap, fmt::Debug, path::Path};

use allocative::Allocative;
use bincode::{Decode, Encode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Serialization;

use super::{DateMap, MapChunkId, MapKey, MapSerialized, MapValue};

#[derive(Debug, Default, Serialize, Deserialize, Encode, Decode, Allocative)]
pub struct SerializedBTreeMap<Key, Value>
where
    Key: Ord,
{
    version: u32,
    pub map: BTreeMap<Key, Value>,
}

impl<Key, Value> SerializedBTreeMap<Key, Value>
where
    Key: Ord,
{
    pub fn import_all<ChunkId>(path: &Path, serialization: &Serialization) -> Self
    where
        Self: Debug + Serialize + DeserializeOwned + Encode + Decode,
        ChunkId: MapChunkId,
        Key: MapKey<ChunkId>,
        Value: MapValue,
    {
        let mut s = None;

        DateMap::<usize>::_read_dir(path, serialization)
            .iter()
            .for_each(|(_, path)| {
                let map = serialization.import::<Self>(path).unwrap();

                if s.is_none() {
                    s.replace(map);
                } else {
                    #[allow(clippy::unnecessary_unwrap)]
                    s.as_mut().unwrap().map.extend(map.map);
                }
            });

        s.unwrap()
    }
}

impl<Key, Value, ChunkId> MapSerialized<Key, Value, ChunkId> for SerializedBTreeMap<Key, Value>
where
    Self: Debug + Serialize + DeserializeOwned + Encode + Decode,
    ChunkId: MapChunkId,
    Key: MapKey<ChunkId>,
    Value: MapValue,
{
    fn new(version: u32) -> Self {
        Self {
            version,
            map: BTreeMap::default(),
        }
    }

    fn get_last_key(&self, _: &ChunkId) -> Option<Key> {
        self.map.last_key_value().map(|(k, _)| k.to_owned())
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        self.map.get(key)
    }

    fn last(&self) -> Option<&Value> {
        self.map.last_key_value().map(|(_, v)| v)
    }

    fn extend(&mut self, map: BTreeMap<Key, Value>) {
        self.map.extend(map)
    }
}
