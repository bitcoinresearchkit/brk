use std::{collections::BTreeMap, mem};

use exit::Exit;
use fjall::{
    PartitionCreateOptions, PersistMode, ReadTransaction, Result, Slice, TransactionalKeyspace,
    TransactionalPartitionHandle, TxKeyspace, WriteTransaction,
};

use crate::structs::{Height, Version};

pub struct Partition<Key, Value> {
    version: Version,
    data: TransactionalPartitionHandle,
    meta: TransactionalPartitionHandle,
    height: Option<Height>,
    puts: BTreeMap<Key, Value>,
}

impl<Key, Value> Partition<Key, Value>
where
    Key: Into<Slice> + Ord,
    Value: Into<Slice> + TryFrom<Slice> + Clone,
{
    pub const VERSION: &str = "version";
    pub const HEIGHT: &str = "height";

    pub fn import(
        keyspace: &TransactionalKeyspace,
        name: &str,
        version: Version,
        exit: &Exit,
    ) -> color_eyre::Result<Self> {
        let data = Self::open_data(keyspace, name)?;
        let meta = Self::open_meta(keyspace, name)?;

        let height = if let Some(slice) = meta.get(Self::HEIGHT)? {
            Some(Height::try_from(slice)?)
        } else {
            None
        };

        let mut this = Self {
            version,
            height,
            data,
            meta,
            puts: BTreeMap::new(),
        };

        if let Some(slice) = this.meta.get(Self::VERSION)? {
            if version != Version::try_from(slice)? {
                this = this.reset(keyspace, name, exit)?;
            }
        }

        Ok(this)
    }

    fn open_data(keyspace: &TransactionalKeyspace, name: &str) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(&format!("{name}-data"), Self::create_options())
    }

    fn open_meta(keyspace: &TransactionalKeyspace, name: &str) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(&format!("{name}-meta"), Self::create_options())
    }

    fn create_options() -> PartitionCreateOptions {
        PartitionCreateOptions::default().manual_journal_persist(true)
    }

    pub fn has(&self, height: Height) -> bool {
        self.height.is_some_and(|self_height| self_height >= height)
    }
    pub fn needs(&self, height: Height) -> bool {
        !self.has(height)
    }

    pub fn get<'a>(&self, rtx: &ReadTransaction, key: &'a Key) -> color_eyre::Result<Option<Value>>
    where
        fjall::Slice: std::convert::From<&'a Key>,
        <Value as std::convert::TryFrom<fjall::Slice>>::Error: std::error::Error + Send + Sync,
        <Value as std::convert::TryFrom<fjall::Slice>>::Error: 'static,
    {
        if let Some(v) = self.puts.get(key) {
            return Ok(Some(v.clone()));
        }

        if let Some(slice) = rtx.get(&self.data, Slice::from(key))? {
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

    fn update_meta(&self, wtx: &mut WriteTransaction, height: Height) {
        wtx.insert(&self.meta, Self::VERSION, self.version());
        wtx.insert(&self.meta, Self::HEIGHT, height);
    }

    pub fn write(&mut self, keyspace: &TxKeyspace, height: Height) -> Result<()> {
        if self.has(height) && self.puts.is_empty() {
            return Ok(());
        }

        let mut wtx = keyspace.write_tx();
        mem::take(&mut self.puts)
            .into_iter()
            .for_each(|(key, value)| wtx.insert(&self.data, key, value));
        self.update_meta(&mut wtx, height);
        wtx.commit()
    }

    pub fn version(&self) -> Version {
        self.version
    }

    fn reset(mut self, keyspace: &TransactionalKeyspace, name: &str, exit: &Exit) -> Result<Self> {
        exit.block();

        keyspace.delete_partition(self.data)?;
        keyspace.delete_partition(self.meta)?;

        keyspace.persist(PersistMode::SyncAll)?;

        self.data = Self::open_data(keyspace, name)?;
        self.meta = Self::open_meta(keyspace, name)?;
        self.height = None;

        exit.unblock();

        Ok(self)
    }

    pub fn height(&self) -> Option<&Height> {
        self.height.as_ref()
    }
}
