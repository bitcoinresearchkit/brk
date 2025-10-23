use std::{borrow::Cow, cmp, fmt::Debug, fs, hash::Hash, path::Path};

use brk_error::Result;
use brk_types::{Height, Version};
use byteview6::ByteView;
use fjall2::{
    InnerItem, Keyspace, PartitionCreateOptions, PartitionHandle, PersistMode, ValueType,
};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

mod meta;

use meta::*;

#[derive(Clone)]
pub struct StoreV2<Key, Value> {
    meta: StoreMeta,
    name: &'static str,
    keyspace: Keyspace,
    partition: PartitionHandle,
    puts: FxHashMap<Key, Value>,
    dels: FxHashSet<Key>,
}

const MAJOR_FJALL_VERSION: Version = Version::TWO;

pub fn open_keyspace(path: &Path) -> fjall2::Result<Keyspace> {
    fjall2::Config::new(path.join("fjall"))
        .max_write_buffer_size(32 * 1024 * 1024)
        .open()
}

impl<K, V> StoreV2<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
{
    fn open_partition_handle(
        keyspace: &Keyspace,
        name: &str,
        bloom_filters: Option<bool>,
    ) -> Result<PartitionHandle> {
        let mut options = PartitionCreateOptions::default()
            .max_memtable_size(8 * 1024 * 1024)
            .manual_journal_persist(true);

        if bloom_filters.is_some_and(|b| !b) {
            options = options.bloom_filter_bits(None);
        }

        keyspace.open_partition(name, options).map_err(|e| e.into())
    }

    pub fn import(
        keyspace: &Keyspace,
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

        Ok(Self {
            meta,
            name: Box::leak(Box::new(name.to_string())),
            keyspace: keyspace.clone(),
            partition,
            puts: FxHashMap::default(),
            dels: FxHashSet::default(),
        })
    }

    #[inline]
    pub fn get<'a>(&'a self, key: &'a K) -> Result<Option<Cow<'a, V>>>
    where
        ByteView: From<&'a K>,
    {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Cow::Borrowed(v)))
        } else if let Some(slice) = self.partition.get(ByteView::from(key))? {
            Ok(Some(Cow::Owned(V::from(ByteView::from(&*slice)))))
        } else {
            Ok(None)
        }
    }

    pub fn is_empty(&self) -> Result<bool> {
        self.partition.is_empty().map_err(|e| e.into())
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        self.partition
            .iter()
            .map(|res| res.unwrap())
            .map(|(k, v)| (K::from(ByteView::from(&*k)), V::from(ByteView::from(&*v))))
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

        let mut items = self
            .puts
            .drain()
            .map(|(key, value)| Item::Value { key, value })
            .chain(self.dels.drain().map(|key| Item::Tomb(key)))
            .collect::<Vec<_>>();
        items.sort_unstable();

        self.keyspace.batch().commit_partition(
            &self.partition,
            items.into_iter().map(InnerItem::from).collect::<Vec<_>>(),
        )?;

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
}

impl<K, V> From<Item<K, V>> for InnerItem
where
    K: Into<ByteView>,
    V: Into<ByteView>,
{
    fn from(value: Item<K, V>) -> Self {
        match value {
            Item::Value { key, value } => Self {
                key: key.into().into(),
                value: value.into().into(),
                value_type: ValueType::Value,
            },
            Item::Tomb(key) => Self {
                key: key.into().into(),
                value: [].into(),
                value_type: ValueType::WeakTombstone,
            },
        }
    }
}
