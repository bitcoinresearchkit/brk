use std::{borrow::Cow, cmp, fmt::Debug, fs, hash::Hash, mem, path::Path};

use brk_error::Result;
use brk_types::{Height, Version};
use byteview8::ByteView;
use fjall3::{
    Database, Keyspace, KeyspaceCreateOptions, ValueType,
    config::{BloomConstructionPolicy, FilterPolicy, FilterPolicyEntry, PinningPolicy},
};

mod meta;

use meta::*;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

#[derive(Clone)]
pub struct StoreFjallV3<Key, Value> {
    meta: StoreMeta,
    name: &'static str,
    keyspace: Keyspace,
    puts: FxHashMap<Key, Value>,
    dels: FxHashSet<Key>,
}

const MAJOR_FJALL_VERSION: Version = Version::new(3);

pub fn open_fjall3_database(path: &Path) -> fjall3::Result<Database> {
    Database::builder(path.join("fjall"))
        .cache_size(4 * 1024 * 1024 * 1024)
        .open()
}

impl<K, V> StoreFjallV3<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
{
    pub fn import(
        database: &Database,
        path: &Path,
        name: &str,
        version: Version,
        mode: Mode3,
        kind: Kind3,
    ) -> Result<Self> {
        fs::create_dir_all(path)?;

        let (meta, keyspace) = StoreMeta::checked_open(
            database,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_keyspace(database, name, mode, kind).inspect_err(|e| {
                    eprintln!("{e}");
                    eprintln!("Delete {path:?} and try again");
                })
            },
        )?;

        Ok(Self {
            meta,
            name: Box::leak(Box::new(name.to_string())),
            keyspace,
            puts: FxHashMap::default(),
            dels: FxHashSet::default(),
        })
    }

    fn open_keyspace(
        database: &Database,
        name: &str,
        _mode: Mode3,
        kind: Kind3,
    ) -> Result<Keyspace> {
        let mut options = KeyspaceCreateOptions::default().manual_journal_persist(true);

        if kind.is_not_vec() {
            options = options.filter_policy(FilterPolicy::new(&[
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
                .filter_block_pinning_policy(PinningPolicy::all(false))
                .index_block_pinning_policy(PinningPolicy::all(false));
        }

        database.keyspace(name, options).map_err(|e| e.into())
    }

    #[inline]
    pub fn get<'a>(&'a self, key: &'a K) -> Result<Option<Cow<'a, V>>>
    where
        ByteView: From<&'a K>,
    {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Cow::Borrowed(v)))
        } else if let Some(slice) = self.keyspace.get(ByteView::from(key))? {
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
    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.needs(height) {
            self.insert(key, value);
        }
    }

    #[inline]
    pub fn insert(&mut self, key: K, value: V) {
        let _ = self.dels.is_empty() || self.dels.remove(&key);
        self.puts.insert(key, value);
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
    fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }
}

impl<K, V> AnyStore for StoreFjallV3<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
    Self: Send + Sync,
{
    fn take_all_f2(&mut self) -> Vec<fjall2::InnerItem> {
        vec![]
    }

    fn partition(&self) -> &fjall2::PartitionHandle {
        panic!()
    }

    fn take_all_f3(&mut self) -> Vec<fjall3::Item> {
        let mut items = mem::take(&mut self.puts)
            .into_iter()
            .map(|(key, value)| Item::Value { key, value })
            .chain(
                mem::take(&mut self.dels)
                    .into_iter()
                    .map(|key| Item::Tomb(key)),
            )
            .collect::<Vec<_>>();
        items.sort_unstable();
        items
            .into_iter()
            .map(|v| v.fjalled(&self.keyspace))
            .collect()
    }

    fn export_meta_if_needed(&mut self, height: Height) -> Result<()> {
        if self.has(height) {
            return Ok(());
        }
        self.meta.export(height)?;
        Ok(())
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
}

pub enum Item<K, V> {
    Value { key: K, value: V },
    Tomb(K),
}

impl<K: Ord, V> Ord for Item<K, V> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key().cmp(other.key())
    }
}

impl<K: Ord, V> PartialOrd for Item<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Eq, V> PartialEq for Item<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl<K: Eq, V> Eq for Item<K, V> {}

impl<K, V> Item<K, V> {
    fn key(&self) -> &K {
        match self {
            Self::Value { key, .. } | Self::Tomb(key) => key,
        }
    }

    pub fn fjalled(self, keyspace: &Keyspace) -> fjall3::Item
    where
        K: Into<ByteView>,
        V: Into<ByteView>,
    {
        let keyspace_id = keyspace.id;
        // let keyspace_id = keyspace.inner().id;
        match self {
            Item::Value { key, value } => fjall3::Item {
                keyspace_id,
                key: key.into().into(),
                value: value.into().into(),
                value_type: ValueType::Value,
            },
            Item::Tomb(key) => fjall3::Item {
                keyspace_id,
                key: key.into().into(),
                value: [].into(),
                value_type: ValueType::Tombstone,
            },
        }
    }
}

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
