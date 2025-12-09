use std::{borrow::Cow, cmp::Ordering, fmt::Debug, fs, hash::Hash, mem, path::Path};

use brk_error::Result;
use brk_types::{Height, Version};
use byteview_f3::ByteView;
use fjall3::{Database, Keyspace, KeyspaceCreateOptions, config::*};
use rustc_hash::{FxHashMap, FxHashSet};

mod meta;
use meta::*;

use crate::any::AnyStore;

const MAJOR_FJALL_VERSION: Version = Version::new(3);

pub fn open_fjall3_database(path: &Path) -> fjall3::Result<Database> {
    Database::builder(path.join("fjall"))
        .cache_size(2 * 1024 * 1024 * 1024)
        .open()
}

#[derive(Clone)]
pub struct StoreFjallV3<K, V> {
    meta: StoreMeta,
    name: &'static str,
    keyspace: Keyspace,
    puts: FxHashMap<K, V>,
    dels: FxHashSet<K>,
    caches: Vec<FxHashMap<K, V>>,
}

impl<K, V> StoreFjallV3<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
    Self: Send + Sync,
{
    pub fn import(
        db: &Database,
        path: &Path,
        name: &str,
        version: Version,
        mode: Mode3,
        kind: Kind3,
    ) -> Result<Self> {
        Self::import_inner(db, path, name, version, mode, kind, 0)
    }

    pub fn import_cached(
        db: &Database,
        path: &Path,
        name: &str,
        version: Version,
        mode: Mode3,
        kind: Kind3,
        max_batches: u8,
    ) -> Result<Self> {
        Self::import_inner(db, path, name, version, mode, kind, max_batches)
    }

    fn import_inner(
        db: &Database,
        path: &Path,
        name: &str,
        version: Version,
        mode: Mode3,
        kind: Kind3,
        max_batches: u8,
    ) -> Result<Self> {
        fs::create_dir_all(path)?;

        let (meta, keyspace) = StoreMeta::checked_open(
            db,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_keyspace(db, name, mode, kind).inspect_err(|e| {
                    eprintln!("{e}");
                    eprintln!("Delete {path:?} and try again");
                })
            },
        )?;

        let mut caches = vec![];
        for _ in 0..max_batches {
            caches.push(FxHashMap::default());
        }

        Ok(Self {
            meta,
            name: Box::leak(Box::new(name.to_string())),
            keyspace,
            puts: FxHashMap::default(),
            dels: FxHashSet::default(),
            caches,
        })
    }

    fn open_keyspace(
        database: &Database,
        name: &str,
        _mode: Mode3,
        kind: Kind3,
    ) -> Result<Keyspace> {
        let mut options = KeyspaceCreateOptions::default()
            .manual_journal_persist(true)
            .expect_point_read_hits(true)
            .filter_block_partitioning_policy(PartitioningPolicy::new([false, false, true]))
            .index_block_partitioning_policy(PartitioningPolicy::new([false, false, true]));

        if kind.is_not_vec() {
            options = options.filter_policy(FilterPolicy::new([
                FilterPolicyEntry::Bloom(BloomConstructionPolicy::BitsPerKey(10.0)),
                FilterPolicyEntry::Bloom(BloomConstructionPolicy::BitsPerKey(10.0)),
                FilterPolicyEntry::Bloom(BloomConstructionPolicy::BitsPerKey(8.0)),
                FilterPolicyEntry::Bloom(BloomConstructionPolicy::BitsPerKey(7.0)),
            ]));
        } else {
            options = options
                .max_memtable_size(8 * 1024 * 1024)
                .filter_policy(FilterPolicy::disabled());
        }

        if kind.is_sequential() {
            options = options
                .filter_block_partitioning_policy(PartitioningPolicy::all(true))
                .index_block_partitioning_policy(PartitioningPolicy::all(true))
                .filter_block_pinning_policy(PinningPolicy::all(false))
                .index_block_pinning_policy(PinningPolicy::all(false));
        }

        database.keyspace(name, || options).map_err(|e| e.into())
    }

    #[inline]
    pub fn get<'a>(&'a self, key: &'a K) -> Result<Option<Cow<'a, V>>>
    where
        ByteView: From<&'a K>,
    {
        if let Some(v) = self.puts.get(key) {
            return Ok(Some(Cow::Borrowed(v)));
        }

        for cache in &self.caches {
            if let Some(v) = cache.get(key) {
                return Ok(Some(Cow::Borrowed(v)));
            }
        }

        if let Some(slice) = self.keyspace.get(ByteView::from(key))? {
            Ok(Some(Cow::Owned(V::from(ByteView::from(slice)))))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn is_empty(&self) -> Result<bool> {
        self.keyspace.is_empty().map_err(|e| e.into())
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        let _ = self.dels.is_empty() || self.dels.remove(&key);
        self.puts.insert(key, value);
    }

    #[inline]
    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.needs(height) {
            self.insert(key, value);
        }
    }

    #[inline]
    pub fn remove(&mut self, key: K) {
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

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        self.keyspace
            .iter()
            .map(|res| res.into_inner().unwrap())
            .map(|(k, v)| (K::from(ByteView::from(&*k)), V::from(ByteView::from(&*v))))
    }

    #[inline]
    fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }

    #[inline]
    pub fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }

    fn export_meta_if_needed(&mut self, height: Height) -> Result<()> {
        if !self.has(height) {
            self.meta.export(height)?;
        }
        Ok(())
    }

    fn ingest<'a>(
        keyspace: &Keyspace,
        puts: impl Iterator<Item = (&'a K, &'a V)>,
        dels: impl Iterator<Item = &'a K>,
    ) -> Result<()>
    where
        ByteView: From<&'a K> + From<&'a V>,
        K: 'a,
        V: 'a,
    {
        let mut items: Vec<Item<&'a K, &'a V>> = puts
            .map(|(key, value)| Item::Value { key, value })
            .chain(dels.map(Item::Tomb))
            .collect();

        items.sort_unstable();

        let mut ingestion = keyspace.start_ingestion()?;
        for item in items {
            match item {
                Item::Value { key, value } => {
                    ingestion.write(ByteView::from(key), ByteView::from(value))?;
                }
                Item::Tomb(key) => {
                    ingestion.write_tombstone(ByteView::from(key))?;
                }
            }
        }
        ingestion.finish()?;

        Ok(())
    }
}

impl<K, V> AnyStore for StoreFjallV3<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    for<'a> ByteView: From<K> + From<V> + From<&'a K> + From<&'a V>,
    Self: Send + Sync,
{
    fn keyspace(&self) -> &Keyspace {
        &self.keyspace
    }

    fn take_all_f2(&mut self) -> Vec<fjall2::InnerItem> {
        vec![]
    }

    fn partition(&self) -> &fjall2::PartitionHandle {
        panic!()
    }

    fn export_meta_if_needed(&mut self, height: Height) -> Result<()> {
        self.export_meta_if_needed(height)
    }

    fn name(&self) -> &'static str {
        self.name
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

    fn commit_f3(&mut self, height: Height) -> Result<()> {
        self.export_meta_if_needed(height)?;

        let puts = mem::take(&mut self.puts);
        let dels = mem::take(&mut self.dels);

        if puts.is_empty() && dels.is_empty() {
            return Ok(());
        }

        Self::ingest(&self.keyspace, puts.iter(), dels.iter())?;

        if !self.caches.is_empty() {
            self.caches.pop();
            self.caches.insert(0, puts);
        }

        Ok(())
    }
}

enum Item<K, V> {
    Value { key: K, value: V },
    Tomb(K),
}
impl<K, V> Item<K, V> {
    #[inline]
    fn key(&self) -> &K {
        match self {
            Self::Value { key, .. } | Self::Tomb(key) => key,
        }
    }
}
impl<K: Ord, V> Ord for Item<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(other.key())
    }
}
impl<K: Ord, V> PartialOrd for Item<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<K: Eq, V> PartialEq for Item<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}
impl<K: Eq, V> Eq for Item<K, V> {}

#[derive(Debug, Clone, Copy)]
pub enum Mode3 {
    Any,
    PushOnly,
}
impl Mode3 {
    pub fn is_any(&self) -> bool {
        matches!(*self, Self::Any)
    }

    pub fn is_push_only(&self) -> bool {
        matches!(*self, Self::PushOnly)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Kind3 {
    Random,
    Sequential,
    Vec,
}
impl Kind3 {
    pub fn is_sequential(&self) -> bool {
        matches!(*self, Self::Sequential)
    }

    pub fn is_random(&self) -> bool {
        matches!(*self, Self::Random)
    }

    pub fn is_not_vec(&self) -> bool {
        !matches!(*self, Self::Vec)
    }
}
