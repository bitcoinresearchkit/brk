use std::{borrow::Cow, fmt::Debug, fs, hash::Hash, path::Path, sync::Arc};

use brk_error::Result;
use brk_structs::{Height, Version};
use byteview_v8::ByteView;
use fjall_v3::{KeyspaceCreateOptions, PersistMode, ReadTransaction, TxDatabase, TxKeyspace};

mod meta;

use log::info;
use meta::*;
use parking_lot::RwLock;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::any::AnyStore;

#[derive(Clone)]
pub struct StoreV3<Key, Value> {
    meta: StoreMeta,
    name: &'static str,
    database: TxDatabase,
    keyspace: Arc<RwLock<Option<TxKeyspace>>>,
    rtx: Arc<RwLock<Option<ReadTransaction>>>,
    puts: FxHashMap<Key, Value>,
    dels: FxHashSet<Key>,
}

const MAJOR_FJALL_VERSION: Version = Version::new(3);

pub fn open_database(path: &Path) -> fjall_v3::Result<TxDatabase> {
    TxDatabase::builder(path.join("fjall"))
        .cache_size(4 * 1024 * 1024 * 1024)
        .open()
}

impl<K, V> StoreV3<K, V>
where
    K: Debug + Clone + From<ByteView> + Ord + Eq + Hash,
    V: Debug + Clone + From<ByteView>,
    ByteView: From<K> + From<V>,
{
    fn open_keyspace(database: &TxDatabase, name: &str) -> Result<TxKeyspace> {
        database
            .keyspace(
                name,
                KeyspaceCreateOptions::default().manual_journal_persist(true),
            )
            .map_err(|e| e.into())
    }

    pub fn import(
        database: &TxDatabase,
        path: &Path,
        name: &str,
        version: Version,
        _bloom_filters: Option<bool>,
    ) -> Result<Self> {
        fs::create_dir_all(path)?;

        let (meta, keyspace) = StoreMeta::checked_open(
            database,
            &path.join(format!("meta/{name}")),
            MAJOR_FJALL_VERSION + version,
            || {
                Self::open_keyspace(database, name).inspect_err(|e| {
                    eprintln!("{e}");
                    eprintln!("Delete {path:?} and try again");
                })
            },
        )?;

        let rtx = database.read_tx();

        Ok(Self {
            meta,
            name: Box::leak(Box::new(name.to_string())),
            database: database.clone(),
            keyspace: Arc::new(RwLock::new(Some(keyspace))),
            rtx: Arc::new(RwLock::new(Some(rtx))),
            puts: FxHashMap::default(),
            dels: FxHashSet::default(),
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
            .get(self.keyspace.read().as_ref().unwrap(), ByteView::from(key))?
        {
            Ok(Some(Cow::Owned(V::from(ByteView::from(slice)))))
        } else {
            Ok(None)
        }
    }

    pub fn is_empty(&self) -> Result<bool> {
        self.rtx
            .read()
            .as_ref()
            .unwrap()
            .is_empty(self.keyspace.read().as_ref().unwrap())
            .map_err(|e| e.into())
    }

    // pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
    //     let keyspace = self.keyspace.read().as_ref().unwrap();

    //     self.rtx
    //         .read()
    //         .as_ref()
    //         .unwrap()
    //         .iter(keyspace)
    //         .map(|res| res.into_inner().unwrap())
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

impl<K, V> AnyStore for StoreV3<K, V>
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
        let bad_rtx = rtx.take();
        drop(bad_rtx);

        let mut wtx = self.database.write_tx();

        let keyspace = self.keyspace.read();

        let partition = keyspace.as_ref().unwrap();

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

        rtx.replace(self.database.read_tx());

        Ok(())
    }

    fn persist(&self) -> Result<()> {
        self.database
            .persist(PersistMode::SyncAll)
            .map_err(|e| e.into())
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn reset(&mut self) -> Result<()> {
        info!("Resetting {}...", self.name);

        todo!();

        let mut opt = self.keyspace.write();

        let keyspace = opt.take().unwrap();

        // Doesn't exist yet
        // self.database.remove_keyspace(keyspace)?;

        self.meta.reset();

        let keyspace = Self::open_keyspace(&self.database, self.name)?;

        opt.replace(keyspace);

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
