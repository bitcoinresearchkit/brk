use std::{collections::BTreeMap, mem, path::Path};

use fjall::{
    PartitionCreateOptions, PersistMode, ReadTransaction, Result, Slice, TransactionalKeyspace,
    TransactionalPartitionHandle,
};
use storable_vec::UnsafeSizedSerDe;

use crate::structs::{Height, Version};

use super::Meta;

pub struct Partition<Key, Value> {
    meta: Meta,
    keyspace: TransactionalKeyspace,
    part: TransactionalPartitionHandle,
    rtx: ReadTransaction,
    puts: BTreeMap<Key, Value>,
}

impl<Key, Value> Partition<Key, Value>
where
    Key: Into<Slice> + Ord,
    Value: Into<Slice> + TryFrom<Slice> + Clone,
{
    pub fn import(path: &Path, version: Version) -> color_eyre::Result<Self> {
        let meta = Meta::checked_open(path, version)?;
        let keyspace = if let Ok(keyspace) = Self::open_keyspace(path) {
            keyspace
        } else {
            meta.reset()?;
            return Self::import(path, version);
        };
        let part = if let Ok(part) = Self::open_partition_handle(&keyspace) {
            part
        } else {
            drop(keyspace);
            meta.reset()?;
            return Self::import(path, version);
        };
        let rtx = keyspace.read_tx();

        Ok(Self {
            meta,
            keyspace,
            part,
            rtx,
            puts: BTreeMap::new(),
        })
    }

    pub fn get(&self, key: &Key) -> color_eyre::Result<Option<Value>>
    where
        <Value as TryFrom<Slice>>::Error: std::error::Error + Send + Sync + 'static,
    {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(v.clone()))
        } else if let Some(slice) = self.rtx.get(&self.part, key.unsafe_as_slice())? {
            Ok(Some(Value::try_from(slice)?))
        } else {
            Ok(None)
        }
    }

    pub fn insert_if_needed(&mut self, key: Key, value: Value, height: Height) {
        if self.needs(height) {
            self.puts.insert(key, value);
        }
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        if self.has(height) && self.puts.is_empty() {
            return Ok(());
        }

        self.meta.export(self.len(), height)?;

        let mut wtx = self.keyspace.write_tx();
        mem::take(&mut self.puts)
            .into_iter()
            .for_each(|(key, value)| wtx.insert(&self.part, key, value));

        wtx.commit()?;

        self.keyspace.persist(PersistMode::SyncAll)?;

        self.rtx = self.keyspace.read_tx();

        Ok(())
    }

    pub fn height(&self) -> Option<&Height> {
        self.meta.height()
    }

    pub fn len(&self) -> usize {
        self.meta.len() + self.puts.len()
    }

    pub fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }
    pub fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }

    fn open_keyspace(path: &Path) -> Result<TransactionalKeyspace> {
        fjall::Config::new(path.join("fjall")).open_transactional()
    }

    fn open_partition_handle(keyspace: &TransactionalKeyspace) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(
            "partition",
            PartitionCreateOptions::default().manual_journal_persist(true),
        )
    }
}
