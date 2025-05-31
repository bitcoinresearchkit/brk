use std::{
    collections::{BTreeMap, BTreeSet},
    error,
    fmt::Debug,
    mem,
    path::Path,
};

use brk_core::{Height, Result, Value, Version};
use byteview::ByteView;
use fjall::{
    PartitionCreateOptions, PersistMode, ReadTransaction, TransactionalKeyspace,
    TransactionalPartitionHandle,
};
use zerocopy::{Immutable, IntoBytes};

mod meta;
use meta::*;

pub struct Store<Key, Value> {
    meta: StoreMeta,
    name: String,
    keyspace: TransactionalKeyspace,
    partition: TransactionalPartitionHandle,
    rtx: ReadTransaction,
    puts: BTreeMap<Key, Value>,
    dels: BTreeSet<Key>,
    bloom_filter_bits: Option<Option<u8>>,
}

/// Use default if will read
const DEFAULT_BLOOM_FILTER_BITS: Option<u8> = Some(5);
const CHECK_COLLISISONS: bool = true;
const MAJOR_FJALL_VERSION: Version = Version::TWO;

impl<K, V> Store<K, V>
where
    K: Debug + Clone + Into<ByteView> + TryFrom<ByteView> + Ord + Immutable + IntoBytes,
    V: Debug + Clone + Into<ByteView> + TryFrom<ByteView>,
    <K as TryFrom<ByteView>>::Error: error::Error + Send + Sync + 'static,
    <V as TryFrom<ByteView>>::Error: error::Error + Send + Sync + 'static,
{
    pub fn import(
        keyspace: &TransactionalKeyspace,
        path: &Path,
        name: &str,
        version: Version,
        bloom_filter_bits: Option<Option<u8>>,
    ) -> Result<Self> {
        let (meta, partition) = StoreMeta::checked_open(
            keyspace,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_partition_handle(keyspace, name, bloom_filter_bits).inspect_err(|e| {
                    eprintln!("{e}");
                    eprintln!("Delete {path:?} and try again");
                })
            },
        )?;

        let rtx = keyspace.read_tx();

        Ok(Self {
            meta,
            name: name.to_owned(),
            keyspace: keyspace.clone(),
            partition,
            rtx,
            puts: BTreeMap::new(),
            dels: BTreeSet::new(),
            bloom_filter_bits,
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

    pub fn get_mut_or_default(&mut self, key: &K) -> &mut V
    where
        V: Default,
    {
        self.puts.entry(key.clone()).or_insert_with(|| {
            if let Some(slice) = self.rtx.get(&self.partition, key.as_bytes()).unwrap() {
                V::try_from(slice.as_bytes().into()).unwrap()
            } else {
                V::default()
            }
        })
    }

    pub fn unordered_clone_iter(&self) -> impl Iterator<Item = (K, V)> {
        self.rtx
            .iter(&self.partition)
            .map(|res| res.unwrap())
            .map(|(k, v)| (K::try_from(ByteView::from(k)).unwrap(), v))
            .filter(|(k, _)| !self.puts.contains_key(k) && !self.dels.contains(k))
            .map(|(k, v)| (k, V::try_from(ByteView::from(v)).unwrap()))
            .chain(self.puts.iter().map(|(k, v)| (k.clone(), v.clone())))
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

    pub fn retain_or_del<F>(&mut self, retain: F)
    where
        F: Fn(&K, &mut V) -> bool,
    {
        self.puts.retain(|k, v| {
            let ret = retain(k, v);
            if !ret {
                self.dels.insert(k.clone());
            }
            ret
        });
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
        bloom_filter_bits: Option<Option<u8>>,
    ) -> Result<TransactionalPartitionHandle> {
        keyspace
            .open_partition(
                name,
                PartitionCreateOptions::default()
                    .bloom_filter_bits(bloom_filter_bits.unwrap_or(DEFAULT_BLOOM_FILTER_BITS))
                    .max_memtable_size(8 * 1024 * 1024)
                    .manual_journal_persist(true),
            )
            .map_err(|e| e.into())
    }

    pub fn reset_partition(&mut self) -> Result<()> {
        self.keyspace.delete_partition(self.partition.clone())?;
        self.keyspace.persist(PersistMode::SyncAll)?;
        self.partition =
            Self::open_partition_handle(&self.keyspace, &self.name, self.bloom_filter_bits)?;
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
            bloom_filter_bits: self.bloom_filter_bits,
        }
    }
}
