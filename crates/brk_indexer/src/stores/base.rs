use std::{
    collections::{BTreeMap, BTreeSet},
    error,
    fmt::Debug,
    mem,
    path::Path,
};

use brk_core::Height;
use brk_vec::{Value, Version};
use byteview::ByteView;
use fjall::{
    PartitionCreateOptions, PersistMode, ReadTransaction, Result, TransactionalKeyspace, TransactionalPartitionHandle,
};
use zerocopy::{Immutable, IntoBytes};

use super::StoreMeta;

pub struct Store<Key, Value> {
    meta: StoreMeta,
    keyspace: TransactionalKeyspace,
    part: TransactionalPartitionHandle,
    rtx: ReadTransaction,
    puts: BTreeMap<Key, Value>,
    dels: BTreeSet<Key>,
}

const CHECK_COLLISISONS: bool = true;

impl<K, V> Store<K, V>
where
    K: Debug + Clone + Into<ByteView> + Ord + Immutable + IntoBytes,
    V: Debug + Clone + Into<ByteView> + TryFrom<ByteView>,
    <V as TryFrom<ByteView>>::Error: error::Error + Send + Sync + 'static,
{
    pub fn import(path: &Path, version: Version) -> color_eyre::Result<Self> {
        let meta = StoreMeta::checked_open(path, version)?;

        let keyspace = match Self::open_keyspace(path) {
            Ok(keyspace) => keyspace,
            Err(e) => {
                dbg!(e);
                meta.reset()?;
                return Self::import(path, version);
            }
        };

        let part = match Self::open_partition_handle(&keyspace) {
            Ok(part) => part,
            Err(e) => {
                dbg!(e);
                drop(keyspace);
                meta.reset()?;
                return Self::import(path, version);
            }
        };

        let rtx = keyspace.read_tx();

        Ok(Self {
            meta,
            keyspace,
            part,
            rtx,
            puts: BTreeMap::new(),
            dels: BTreeSet::new(),
        })
    }

    pub fn get(&self, key: &K) -> color_eyre::Result<Option<Value<V>>> {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Value::Ref(v)))
        } else if let Some(slice) = self.rtx.get(&self.part, key.as_bytes())? {
            Ok(Some(Value::Owned(V::try_from(slice.into())?)))
        } else {
            Ok(None)
        }
    }

    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.needs(height) {
            if !self.dels.is_empty() {
                // self.dels.remove(&key);
                unreachable!("Shouldn't reach this");
            }
            self.puts.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: K) {
        if !self.puts.is_empty() {
            unreachable!("Shouldn't reach this");
            // self.puts.remove(&key);
        }
        // dbg!(&key);
        if !self.dels.insert(key) {
            unreachable!();
        }
    }

    pub fn commit(&mut self, height: Height) -> Result<()> {
        if self.has(height) && self.puts.is_empty() && self.dels.is_empty() {
            return Ok(());
        }

        self.meta.export(self.len(), height)?;

        let mut wtx = self.keyspace.write_tx();

        mem::take(&mut self.dels)
            .into_iter()
            .for_each(|key| wtx.remove(&self.part, key.into()));

        mem::take(&mut self.puts).into_iter().for_each(|(key, value)| {
            if CHECK_COLLISISONS {
                #[allow(unused_must_use)]
                if let Ok(Some(value)) = wtx.get(&self.part, key.as_bytes()) {
                    dbg!(
                        &key,
                        V::try_from(value.into()).unwrap(),
                        &self.meta,
                        self.rtx.get(&self.part, key.as_bytes())
                    );
                    unreachable!();
                }
            }
            wtx.insert(&self.part, key.into(), value.into())
        });

        wtx.commit()?;

        self.keyspace.persist(PersistMode::SyncAll)?;

        self.rtx = self.keyspace.read_tx();

        Ok(())
    }

    pub fn height(&self) -> Option<Height> {
        self.meta.height()
    }

    pub fn len(&self) -> usize {
        self.meta.len() + self.puts.len() - self.dels.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
            PartitionCreateOptions::default()
                .bloom_filter_bits(Some(5))
                .manual_journal_persist(true),
        )
    }
}

impl<Key, Value> Clone for Store<Key, Value>
where
    Key: Clone,
    Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            meta: self.meta.clone(),
            keyspace: self.keyspace.clone(),
            part: self.part.clone(),
            rtx: self.keyspace.read_tx(),
            puts: self.puts.clone(),
            dels: self.dels.clone(),
        }
    }
}
