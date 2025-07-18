#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs, mem,
    path::Path,
};

use brk_core::{Height, Result, Version};
use byteview::ByteView;
use fjall::{
    PartitionCreateOptions, PersistMode, ReadTransaction, TransactionalKeyspace,
    TransactionalPartitionHandle,
};

mod meta;
mod r#trait;

use log::info;
use meta::*;
pub use r#trait::*;

pub struct Store<Key, Value> {
    meta: StoreMeta,
    name: &'static str,
    keyspace: TransactionalKeyspace,
    // Arc it
    partition: Option<TransactionalPartitionHandle>,
    rtx: ReadTransaction,
    puts: BTreeMap<Key, Value>,
    dels: BTreeSet<Key>,
    bloom_filters: Option<bool>,
}

// const CHECK_COLLISIONS: bool = true;
const MAJOR_FJALL_VERSION: Version = Version::TWO;

pub fn open_keyspace(path: &Path) -> fjall::Result<TransactionalKeyspace> {
    fjall::Config::new(path.join("fjall"))
        .max_write_buffer_size(32 * 1024 * 1024)
        .open_transactional()
}

impl<'a, K, V> Store<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + 'a,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<&'a K> + From<V>,
{
    pub fn import(
        keyspace: &TransactionalKeyspace,
        path: &Path,
        name: &str,
        version: Version,
        bloom_filters: Option<bool>,
    ) -> Result<Self> {
        fs::create_dir_all(path)?;

        let (meta, partition) = StoreMeta::checked_open(
            keyspace,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_partition_handle(keyspace, name, bloom_filters).inspect_err(|e| {
                    eprintln!("{e}");
                    eprintln!("Delete {path:?} and try again");
                })
            },
        )?;

        let rtx = keyspace.read_tx();

        Ok(Self {
            meta,
            name: Box::leak(Box::new(name.to_string())),
            keyspace: keyspace.clone(),
            partition: Some(partition),
            rtx,
            puts: BTreeMap::new(),
            dels: BTreeSet::new(),
            bloom_filters,
        })
    }

    pub fn get(&self, key: &'a K) -> Result<Option<Cow<V>>> {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Cow::Borrowed(v)))
        } else if let Some(slice) = self
            .rtx
            .get(self.partition.as_ref().unwrap(), ByteView::from(key))?
        {
            Ok(Some(Cow::Owned(V::from(ByteView::from(slice)))))
        } else {
            Ok(None)
        }
    }

    pub fn is_empty(&self) -> Result<bool> {
        self.rtx
            .is_empty(self.partition.as_ref().unwrap())
            .map_err(|e| e.into())
    }

    // pub fn puts_first_key_value(&self) -> Option<(&K, &V)> {
    //     self.puts.first_key_value()
    // }

    // pub fn puts_last_key_value(&self) -> Option<(&K, &V)> {
    //     self.puts.last_key_value()
    // }

    // pub fn rtx_first_key_value(&self) -> Result<Option<(K, V)>> {
    //     Ok(self
    //         .rtx
    //         .first_key_value(&self.partition.load())?
    //         .map(|(k, v)| (K::from(ByteView::from(k)), V::from(ByteView::from(v)))))
    // }

    // pub fn rtx_last_key_value(&self) -> Result<Option<(K, V)>> {
    //     Ok(self
    //         .rtx
    //         .last_key_value(&self.partition.load())?
    //         .map(|(k, v)| (K::from(ByteView::from(k)), V::from(ByteView::from(v)))))
    // }

    // pub fn tx_iter(&self) -> impl Iterator<Item = (K, V)> {
    //     self.rtx
    //         .iter(&self.partition.load())
    //         .map(|res| res.unwrap())
    //         .map(|(k, v)| (K::from(ByteView::from(k)), V::from(ByteView::from(v))))
    // }

    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.needs(height) {
            if !self.dels.is_empty() {
                self.dels.remove(&key);
                // unreachable!("Shouldn't reach this");
            }
            self.puts.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: K) {
        // if self.is_empty()? {
        //     return Ok(());
        // }

        if !self.puts.is_empty() {
            unreachable!("Shouldn't reach this");
        }

        if !self.dels.insert(key.clone()) {
            dbg!(key, &self.meta.path());
            unreachable!();
        }

        // Ok(())
    }

    // pub fn retain_or_del<F>(&mut self, retain: F)
    // where
    //     F: Fn(&K, &mut V) -> bool,
    // {
    //     self.puts.retain(|k, v| {
    //         let ret = retain(k, v);
    //         if !ret {
    //             self.dels.insert(k.clone());
    //         }
    //         ret
    //     });
    // }

    fn open_partition_handle(
        keyspace: &TransactionalKeyspace,
        name: &str,
        bloom_filters: Option<bool>,
    ) -> Result<TransactionalPartitionHandle> {
        let mut options = PartitionCreateOptions::default()
            .max_memtable_size(8 * 1024 * 1024)
            .manual_journal_persist(true);

        if bloom_filters.is_some_and(|b| !b) {
            options = options.bloom_filter_bits(None);
        }

        keyspace.open_partition(name, options).map_err(|e| e.into())
    }

    pub fn commit_(
        &mut self,
        height: Height,
        remove: impl Iterator<Item = K>,
        insert: impl Iterator<Item = (K, V)>,
    ) -> Result<()> {
        if self.has(height) {
            return Ok(());
        }

        self.meta.export(height)?;

        let mut wtx = self.keyspace.write_tx();

        let partition = self.partition.as_ref().unwrap();

        remove.for_each(|key| wtx.remove(partition, ByteView::from(key)));

        insert.for_each(|(key, value)| {
            // if CHECK_COLLISIONS {
            //     #[allow(unused_must_use)]
            //     if let Ok(Some(value)) = wtx.get(&self.partition, key.as_bytes()) {
            //         dbg!(
            //             &key,
            //             V::try_from(value.as_bytes().into()).unwrap(),
            //             &self.meta,
            //             self.rtx.get(&self.partition, key.as_bytes())
            //         );
            //         unreachable!();
            //     }
            // }
            wtx.insert(partition, ByteView::from(key), ByteView::from(value))
        });

        wtx.commit()?;

        self.rtx = self.keyspace.read_tx();

        Ok(())
    }
}

impl<'a, K, V> AnyStore for Store<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + 'a,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<&'a K> + From<V>,
{
    fn commit(&mut self, height: Height) -> Result<()> {
        if self.puts.is_empty() && self.dels.is_empty() {
            self.meta.export(height)?;
            return Ok(());
        }

        let dels = mem::take(&mut self.dels);
        let puts = mem::take(&mut self.puts);

        self.commit_(height, dels.into_iter(), puts.into_iter())
    }

    fn persist(&self) -> Result<()> {
        self.keyspace
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn reset(&mut self) -> Result<()> {
        info!("Resetting {}...", self.name);

        let partition: TransactionalPartitionHandle = self.partition.take().unwrap();

        self.keyspace.delete_partition(partition)?;

        self.meta.reset();

        let partition = Self::open_partition_handle(&self.keyspace, self.name, self.bloom_filters)?;

        self.partition.replace(partition);

        Ok(())
    }

    fn height(&self) -> Option<Height> {
        self.meta.height()
    }

    fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }
    fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }

    fn version(&self) -> Version {
        self.meta.version()
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
            name: self.name,
            keyspace: self.keyspace.clone(),
            partition: None,
            rtx: self.keyspace.read_tx(),
            puts: self.puts.clone(),
            dels: self.dels.clone(),
            bloom_filters: self.bloom_filters,
        }
    }
}
