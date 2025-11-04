use std::{
    borrow::{Borrow, Cow},
    fmt::Debug,
    fs,
    hash::Hash,
    mem::{self, transmute},
    path::Path,
    sync::Arc,
};

use brk_error::Result;
use brk_types::{Height, Version};
use parking_lot::RwLock;
use redb::{
    Builder, Database, Durability, Key, ReadOnlyTable, ReadableDatabase, ReadableTableMetadata,
    TableDefinition, Value,
};

mod meta;

use meta::*;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

#[derive(Clone)]
pub struct StoreRedb<K, V>
where
    K: Key + 'static,
    V: Value + 'static,
{
    meta: StoreMeta,
    name: &'static str,
    db: Arc<Database>,
    table: Arc<RwLock<Option<ReadOnlyTable<K, V>>>>,
    puts: FxHashMap<K, V>,
    dels: FxHashSet<K>,
}

const MAJOR_FJALL_VERSION: Version = Version::new(3);

pub fn open_redb_database(path: &Path) -> redb::Result<Database> {
    let db = Builder::new()
        .set_cache_size(4 * 1024 * 1024 * 1024)
        .create(path.join("store.redb"))
        .unwrap();
    Ok(db)
}

impl<K, V> StoreRedb<K, V>
where
    K: Key + Ord + Eq + Hash + 'static,
    V: Value + Clone + 'static,
{
    pub fn import(
        db: &Arc<Database>,
        path: &Path,
        name: &str,
        version: Version,
        _bloom_filters: Option<bool>,
    ) -> Result<Self> {
        fs::create_dir_all(path)?;

        let meta = StoreMeta::checked_open(
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
        )?;

        {
            let mut wtx = db.begin_write().unwrap();
            wtx.set_durability(Durability::Immediate).unwrap();
            let definition: TableDefinition<K, V> = TableDefinition::new(name);
            let table = wtx.open_table(definition).unwrap();
            drop(table);
            wtx.commit().unwrap();
        }

        let definition: TableDefinition<K, V> = TableDefinition::new(name);
        let table = db.begin_read().unwrap().open_table(definition).unwrap();

        Ok(Self {
            db: db.clone(),
            meta,
            name: Box::leak(Box::new(name.to_string())),
            table: Arc::new(RwLock::new(Some(table))),
            puts: FxHashMap::default(),
            dels: FxHashSet::default(),
        })
    }

    // In case my hack doesn't work:
    // https://github.com/cberner/redb/issues/869
    #[inline]
    pub fn get<'a>(&'a self, key: &'a K) -> Result<Option<Cow<'a, V>>>
    where
        &'a K: Borrow<K::SelfType<'a>>,
        V: From<V::SelfType<'static>>,
    {
        if let Some(v) = self.puts.get(key) {
            Ok(Some(Cow::Borrowed(v)))
        } else if let Some(value) = self.table.read().as_ref().unwrap().get(key).unwrap() {
            let selftype: <V as Value>::SelfType<'static> = unsafe { transmute(value.value()) };
            let owned: V = selftype.into();
            Ok(Some(Cow::Owned(owned)))
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.table.read().as_ref().unwrap().len().unwrap() == 0)
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
    fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }

    #[inline]
    fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }
}

impl<K, V> AnyStore for StoreRedb<K, V>
where
    // K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    // V: Debug + Clone + From<ByteView>,
    K: Debug + Clone + Key + Ord + Eq + Hash + 'static + Borrow<K::SelfType<'static>>,
    V: Debug + Clone + Value + 'static + Borrow<V::SelfType<'static>>,
    // ByteView: From<K> + From<V>,
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

        // let mut _rtx_lock = self._rtx.write();
        // drop(_rtx_lock.take());
        let mut table_lock = self.table.write();
        drop(table_lock.take());

        let mut wtx = self.db.begin_write().unwrap();
        wtx.set_durability(Durability::Immediate).unwrap();

        let definition: TableDefinition<K, V> = TableDefinition::new(self.name);
        let mut table = wtx.open_table(definition).unwrap();

        mem::take(&mut self.puts)
            .into_iter()
            .for_each(|(key, value)| {
                table.insert(key, value).unwrap();
            });

        mem::take(&mut self.dels).into_iter().for_each(|key| {
            table.remove(key).unwrap();
        });

        drop(table);

        wtx.commit().unwrap();

        table_lock.replace(
            self.db
                .begin_read()
                .unwrap()
                .open_table(definition)
                .unwrap(),
        );

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
