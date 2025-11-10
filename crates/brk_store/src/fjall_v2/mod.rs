use std::{borrow::Cow, cmp, fmt::Debug, fs, hash::Hash, mem, path::Path};

use brk_error::Result;
use brk_types::{Height, Version};
use byteview6::ByteView;
use fjall2::{
    CompressionType, InnerItem, PartitionCreateOptions, TransactionalKeyspace,
    TransactionalPartitionHandle, ValueType,
};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

mod meta;

use meta::*;

#[derive(Clone)]
pub struct StoreFjallV2<Key, Value> {
    meta: StoreMeta,
    name: &'static str,
    keyspace: TransactionalKeyspace,
    partition: TransactionalPartitionHandle,
    puts: FxHashMap<Key, Value>,
    dels: FxHashSet<Key>,
}

const MAJOR_FJALL_VERSION: Version = Version::TWO;

pub fn open_keyspace(path: &Path) -> fjall2::Result<TransactionalKeyspace> {
    fjall2::Config::new(path.join("fjall"))
        .manual_journal_persist(true)
        .open_transactional()
}

impl<K, V> StoreFjallV2<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
{
    fn open_partition_handle(
        keyspace: &TransactionalKeyspace,
        name: &str,
        mode: Mode,
        compression: CompressionType,
    ) -> Result<TransactionalPartitionHandle> {
        let mut options = PartitionCreateOptions::default()
            .compression(compression)
            .manual_journal_persist(true);

        if mode.is_unique_push_only() {
            options = options.bloom_filter_bits(Some(7));
        } else {
            options = options
                .max_memtable_size(8 * 1024 * 1024)
                .bloom_filter_bits(None);
        }

        keyspace.open_partition(name, options).map_err(|e| e.into())
    }

    pub fn import(
        keyspace: &TransactionalKeyspace,
        path: &Path,
        name: &str,
        version: Version,
        mode: Mode,
        compression: CompressionType,
    ) -> Result<Self> {
        fs::create_dir_all(path)?;

        let (meta, partition) = StoreMeta::checked_open(
            keyspace,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_partition_handle(keyspace, name, mode, compression).inspect_err(|e| {
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
        self.keyspace
            .read_tx()
            .is_empty(&self.partition)
            .map_err(|e| e.into())
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        self.keyspace
            .read_tx()
            .iter(&self.partition)
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
    pub fn approximate_len(&self) -> usize {
        self.partition.approximate_len()
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

impl<K, V> AnyStore for StoreFjallV2<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash + 'static,
    V: Debug + Clone + From<ByteView> + 'static,
    ByteView: From<K> + From<V>,
    Self: Send + Sync,
{
    fn partition(&self) -> &fjall2::PartitionHandle {
        self.partition.inner()
    }

    fn take_all_f2(&mut self) -> Vec<InnerItem> {
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
        items.into_iter().map(InnerItem::from).collect()
    }

    // fn take_all_f3(&mut self) -> Box<dyn Iterator<Item = Item>> {
    //     Box::new([].into_iter())
    // }

    fn export_meta_if_needed(&mut self, height: Height) -> Result<()> {
        if self.has(height) {
            return Ok(());
        }
        self.meta.export(height)?;
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        if self.puts.is_empty() && self.dels.is_empty() {
            return Ok(());
        }

        // if self.mode.is_unique_push_only() {
        //     if !self.dels.is_empty() {
        //         unreachable!();
        //     }
        //     let mut puts = mem::take(&mut self.puts).into_iter().collect::<Vec<_>>();
        //     puts.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
        //     dbg!(&puts);
        //     self.partition.ingest(
        //         puts.into_iter()
        //             .map(|(k, v)| (ByteView::from(k), ByteView::from(v))),
        //     )?;
        // } else {
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

        // self.keyspace.inner().batch().commit_partition(
        //     self.partition.inner(),
        //     items.into_iter().map(InnerItem::from).collect::<Vec<_>>(),
        // )?;
        // }

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

enum Item<K, V> {
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
    #[inline]
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
                value_type: ValueType::Tombstone,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    VecLike,
    UniquePushOnly(Type),
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Random,
    Sequential,
}

impl Mode {
    pub fn is_vec_like(&self) -> bool {
        matches!(*self, Self::VecLike)
    }

    pub fn is_unique_push_only(&self) -> bool {
        matches!(*self, Self::UniquePushOnly(_))
    }
}
