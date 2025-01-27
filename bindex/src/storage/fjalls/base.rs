use std::{collections::BTreeMap, mem, path::Path};

use fjall::{
    PartitionCreateOptions, PersistMode, ReadTransaction, Result, Slice, TransactionalKeyspace,
    TransactionalPartitionHandle,
};

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

        let keyspace = fjall::Config::new(path.join("fjall")).open_transactional()?;
        let handle = keyspace.open_partition(
            "partition",
            PartitionCreateOptions::default().manual_journal_persist(true),
        )?;
        let rtx = keyspace.read_tx();

        Ok(Self {
            meta,
            keyspace,
            part: handle,
            rtx,
            puts: BTreeMap::new(),
        })
    }

    pub fn len(&self) -> usize {
        self.meta.len() + self.puts.len()
    }

    pub fn has(&self, height: Height) -> bool {
        self.height().is_some_and(|self_height| self_height >= &height)
    }
    pub fn needs(&self, height: Height) -> bool {
        !self.has(height)
    }

    pub fn get<'a>(&self, key: &'a Key) -> color_eyre::Result<Option<Value>>
    where
        fjall::Slice: std::convert::From<&'a Key>,
        <Value as std::convert::TryFrom<fjall::Slice>>::Error: std::error::Error + Send + Sync,
        <Value as std::convert::TryFrom<fjall::Slice>>::Error: 'static,
    {
        if let Some(v) = self.puts.get(key) {
            return Ok(Some(v.clone()));
        }

        if let Some(slice) = self.rtx.get(&self.part, Slice::from(key))? {
            let v_res = Value::try_from(slice);
            let v = v_res?;
            Ok(Some(v))
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

        let mut wtx = self.keyspace.write_tx();
        mem::take(&mut self.puts)
            .into_iter()
            .for_each(|(key, value)| wtx.insert(&self.part, key, value));
        self.meta.export(self.len(), height)?;
        wtx.commit()?;

        self.keyspace.persist(PersistMode::SyncAll)?;

        self.rtx = self.keyspace.read_tx();

        Ok(())
    }

    pub fn height(&self) -> Option<&Height> {
        self.meta.height()
    }
}
