use std::{borrow::Cow, fmt::Debug, fs, hash::Hash, mem, path::Path, sync::Arc};

use brk_error::Result;
use brk_structs::{Height, Version};
use byteview6::ByteView;
use fjall2::{
    PartitionCreateOptions, PersistMode, ReadTransaction, TransactionalKeyspace,
    TransactionalPartitionHandle,
};
use parking_lot::RwLock;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

mod meta;

use log::info;
use meta::*;

#[derive(Clone)]
pub struct StoreV2<Key, Value> {
    meta: StoreMeta,
    name: &'static str,
    keyspace: TransactionalKeyspace,
    partition: Arc<RwLock<Option<TransactionalPartitionHandle>>>,
    rtx: Arc<RwLock<Option<ReadTransaction>>>,
    puts: FxHashMap<Key, Value>,
    dels: FxHashSet<Key>,
    bloom_filters: Option<bool>,
}

// const CHECK_COLLISIONS: bool = true;
const MAJOR_FJALL_VERSION: Version = Version::TWO;

pub fn open_keyspace(path: &Path) -> fjall2::Result<TransactionalKeyspace> {
    fjall2::Config::new(path.join("fjall"))
        // .cache_size(1024 * 1024 * 1024) // for tests only
        .max_write_buffer_size(32 * 1024 * 1024)
        .open_transactional()
}

impl<K, V> StoreV2<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
{
    fn open_partition_handle(
        keyspace: &TransactionalKeyspace,
        name: &str,
        bloom_filters: Option<bool>,
    ) -> Result<TransactionalPartitionHandle> {
        let mut options = PartitionCreateOptions::default()
            // .max_memtable_size(64 * 1024 * 1024) // for tests only
            .max_memtable_size(8 * 1024 * 1024)
            .manual_journal_persist(true);

        if bloom_filters.is_some_and(|b| !b) {
            options = options.bloom_filter_bits(None);
        }

        keyspace.open_partition(name, options).map_err(|e| e.into())
    }

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
            partition: Arc::new(RwLock::new(Some(partition))),
            rtx: Arc::new(RwLock::new(Some(rtx))),
            puts: FxHashMap::default(),
            dels: FxHashSet::default(),
            bloom_filters,
        })
    }

    #[inline]
    pub fn get<'a>(&'a self, key: &'a K) -> Result<Option<Cow<'a, V>>>
    where
        ByteView: From<&'a K>,
    {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Cow::Borrowed(v)))
        } else if let Some(slice) = self
            .rtx
            .read()
            .as_ref()
            .unwrap()
            .get(self.partition.read().as_ref().unwrap(), ByteView::from(key))?
        {
            Ok(Some(Cow::Owned(V::from(ByteView::from(&*slice)))))
        } else {
            Ok(None)
        }
    }

    pub fn is_empty(&self) -> Result<bool> {
        self.rtx
            .read()
            .as_ref()
            .unwrap()
            .is_empty(self.partition.read().as_ref().unwrap())
            .map_err(|e| e.into())
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        self.rtx
            .read()
            .as_ref()
            .unwrap()
            .iter(self.partition.read().as_ref().unwrap())
            .map(|res| res.unwrap())
            .map(|(k, v)| (K::from(ByteView::from(&*k)), V::from(ByteView::from(&*v))))
    }

    #[inline]
    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.needs(height) {
            let _ = self.dels.is_empty() || self.dels.remove(&key);
            self.puts.insert(key, value);
        }
    }

    #[inline]
    pub fn remove(&mut self, key: K) {
        // Hot path: key was recently inserted
        if self.puts.remove(&key).is_some() {
            return;
        }

        let newly_inserted = self.dels.insert(key);
        debug_assert!(newly_inserted, "Double deletion at {:?}", self.meta.path());
    }

    #[inline]
    pub fn remove_if_needed(&mut self, key: K, height: Height) {
        if self.needs(height) {
            self.remove(key)
        }
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

    #[inline]
    fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }

    #[inline]
    fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }
}

impl<K, V> AnyStore for StoreV2<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
    Self: Send + Sync,
{
    fn commit(&mut self, height: Height) -> Result<()> {
        if self.has(height) {
            return Ok(());
        }

        self.meta.export(height)?;

        if self.puts.is_empty() && self.dels.is_empty() {
            return Ok(());
        }

        let mut rtx = self.rtx.write();
        let _ = rtx.take();

        let mut wtx = self.keyspace.write_tx();

        let partition = self.partition.read();

        let partition = partition.as_ref().unwrap();

        wtx.remove_batch(partition, self.dels.drain().map(ByteView::from));

        wtx.insert_batch(
            partition,
            self.puts
                .drain()
                .map(|(k, v)| (ByteView::from(k), ByteView::from(v))),
        );

        wtx.commit()?;

        rtx.replace(self.keyspace.read_tx());

        Ok(())
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

        let mut opt = self.partition.write();

        let partition = opt.take().unwrap();

        self.keyspace.delete_partition(partition)?;

        self.meta.reset();

        let partition = Self::open_partition_handle(&self.keyspace, self.name, self.bloom_filters)?;

        opt.replace(partition);

        Ok(())
    }

    fn height(&self) -> Option<Height> {
        self.meta.height()
    }

    fn has(&self, height: Height) -> bool {
        self.has(height)
    }

    fn needs(&self, height: Height) -> bool {
        self.needs(height)
    }

    fn version(&self) -> Version {
        self.meta.version()
    }
}
