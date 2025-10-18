use std::{borrow::Cow, fmt::Debug, fs, hash::Hash, path::Path, sync::Arc};

use brk_error::Result;
use brk_structs::{Height, Version};
use byteview_v6::ByteView;
use fjall_v2::{
    PartitionCreateOptions, PersistMode, ReadTransaction, TransactionalKeyspace,
    TransactionalPartitionHandle,
};

mod meta;

use log::info;
use meta::*;
use parking_lot::RwLock;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

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

pub fn open_keyspace(path: &Path) -> fjall_v2::Result<TransactionalKeyspace> {
    fjall_v2::Config::new(path.join("fjall"))
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

        // if !self.puts.is_empty() {
        //     unreachable!("Shouldn't reach this");
        // }

        if (self.puts.is_empty() || self.puts.remove(&key).is_none()) && !self.dels.insert(key) {
            dbg!(&self.meta.path());
            unreachable!();
        }

        // Ok(())
    }

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

    fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }

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

        let mut dels = self.dels.drain().collect::<Vec<_>>();
        dels.sort_unstable();
        dels.into_iter()
            .for_each(|key| wtx.remove(partition, ByteView::from(key)));

        let mut puts = self.puts.drain().collect::<Vec<_>>();
        puts.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        puts.into_iter().for_each(|(key, value)| {
            wtx.insert(partition, ByteView::from(key), ByteView::from(value))
        });

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
