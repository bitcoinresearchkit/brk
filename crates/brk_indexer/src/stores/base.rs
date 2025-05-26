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
    PartitionCreateOptions, PersistMode, ReadTransaction, Result, TransactionalKeyspace,
    TransactionalPartitionHandle,
};
use zerocopy::{Immutable, IntoBytes};

use super::StoreMeta;

pub struct Store<Key, Value> {
    meta: StoreMeta,
    name: String,
    keyspace: TransactionalKeyspace,
    partition: TransactionalPartitionHandle,
    rtx: ReadTransaction,
    puts: BTreeMap<Key, Value>,
    dels: BTreeSet<Key>,
}

const CHECK_COLLISISONS: bool = true;
const MAJOR_FJALL_VERSION: Version = Version::TWO;

impl<K, V> Store<K, V>
where
    K: Debug + Clone + Into<ByteView> + Ord + Immutable + IntoBytes,
    V: Debug + Clone + Into<ByteView> + TryFrom<ByteView>,
    <V as TryFrom<ByteView>>::Error: error::Error + Send + Sync + 'static,
{
    pub fn import(
        keyspace: TransactionalKeyspace,
        path: &Path,
        name: &str,
        version: Version,
    ) -> color_eyre::Result<Self> {
        let (meta, partition) = StoreMeta::checked_open(
            &keyspace,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_partition_handle(&keyspace, name).inspect_err(|_| {
                    eprintln!("Delete {path:?} and try again");
                })
            },
        )?;

        let rtx = keyspace.read_tx();

        Ok(Self {
            meta,
            name: name.to_owned(),
            keyspace,
            partition,
            rtx,
            puts: BTreeMap::new(),
            dels: BTreeSet::new(),
        })
    }

    pub fn get(&self, key: &K) -> color_eyre::Result<Option<Value<V>>> {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Value::Ref(v)))
        } else if let Some(slice) = self.rtx.get(&self.partition, key.as_bytes())? {
            Ok(Some(Value::Owned(V::try_from(slice.as_bytes().into())?)))
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
        if self.is_empty() {
            return;
        }

        if !self.puts.is_empty() {
            unreachable!("Shouldn't reach this");
        }

        if !self.dels.insert(key.clone()) {
            dbg!(key, &self.meta.path());
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
            .for_each(|key| wtx.remove(&self.partition, key.as_bytes()));

        mem::take(&mut self.puts)
            .into_iter()
            .for_each(|(key, value)| {
                if CHECK_COLLISISONS {
                    #[allow(unused_must_use)]
                    if let Ok(Some(value)) = wtx.get(&self.partition, key.as_bytes()) {
                        dbg!(
                            &key,
                            V::try_from(value.as_bytes().into()).unwrap(),
                            &self.meta,
                            self.rtx.get(&self.partition, key.as_bytes())
                        );
                        unreachable!();
                    }
                }
                wtx.insert(
                    &self.partition,
                    key.as_bytes(),
                    &*ByteView::try_from(value).unwrap(),
                )
            });

        wtx.commit()?;

        self.rtx = self.keyspace.read_tx();

        Ok(())
    }

    pub fn rotate_memtable(&self) {
        let _ = self.partition.inner().rotate_memtable();
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

    fn open_partition_handle(
        keyspace: &TransactionalKeyspace,
        name: &str,
    ) -> Result<TransactionalPartitionHandle> {
        keyspace.open_partition(
            name,
            PartitionCreateOptions::default()
                .bloom_filter_bits(Some(5))
                .max_memtable_size(8 * 1024 * 1024)
                .manual_journal_persist(true),
        )
    }

    pub fn reset_partition(&mut self) -> Result<()> {
        self.keyspace.delete_partition(self.partition.clone())?;
        self.keyspace.persist(PersistMode::SyncAll)?;
        self.partition = Self::open_partition_handle(&self.keyspace, &self.name)?;
        Ok(())
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
            name: self.name.clone(),
            keyspace: self.keyspace.clone(),
            partition: self.partition.clone(),
            rtx: self.keyspace.read_tx(),
            puts: self.puts.clone(),
            dels: self.dels.clone(),
        }
    }
}
