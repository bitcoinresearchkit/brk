use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs, mem,
    path::PathBuf,
};

use allocative::Allocative;

// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/
//
// Seems indeed much faster than ReDB and LMDB (heed)
// But a lot has changed code wise between them so a retest wouldn't hurt
//
// Possible compression: https://pijul.org/posts/sanakirja-zstd/
use sanakirja::{
    btree::{self, page, Db_, Iter},
    Commit, Env, Error, MutTxn, RootDb, Storable,
};

#[derive(Allocative)]
#[allocative(bound = "Key: Allocative, Value: Allocative")]
/// There is no `cached_gets` since it's much cheaper and faster to do a parallel search first using `unsafe_get` than caching gets along the way.
pub struct Database<Key, Value>
where
    Key: Ord + Clone + Debug + Storable,
    Value: Storable + PartialEq,
{
    pub cached_puts: BTreeMap<Key, Value>,
    pub cached_dels: BTreeSet<Key>,
    path: PathBuf,
    #[allocative(skip)]
    db: Db_<Key, Value, page::Page<Key, Value>>,
    #[allocative(skip)]
    txn: MutTxn<Env, ()>,
}

const ROOT_DB: usize = 0;
const PAGE_SIZE: u64 = 4096;

impl<Key, Value> Database<Key, Value>
where
    Key: Ord + Clone + Debug + Storable,
    Value: Storable + PartialEq,
{
    pub fn open(path: PathBuf) -> color_eyre::Result<Self> {
        let env = unsafe { Env::new_nolock(&path, PAGE_SIZE, 1)? };

        let mut txn = Env::mut_txn_begin(env)?;

        let db = txn
            .root_db(ROOT_DB)
            .unwrap_or_else(|| unsafe { btree::create_db_(&mut txn).unwrap() });

        Ok(Self {
            path,
            cached_puts: BTreeMap::default(),
            cached_dels: BTreeSet::default(),
            db,
            txn,
        })
    }

    pub fn iter(&self) -> Iter<'_, MutTxn<Env, ()>, Key, Value, page::Page<Key, Value>> {
        btree::iter(&self.txn, &self.db, None).unwrap()
    }

    pub fn iter_collect(&self) -> BTreeMap<Key, Value>
    where
        Value: Clone,
    {
        self.iter()
            .map(|r| r.unwrap())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<_>()
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        if let Some(cached_put) = self.get_from_puts(key) {
            return Some(cached_put);
        }

        self.db_get(key)
    }

    pub fn db_get(&self, key: &Key) -> Option<&Value> {
        let option = btree::get(&self.txn, &self.db, key, None).unwrap();

        if let Some((key_found, v)) = option {
            if key == key_found {
                return Some(v);
            }
        }

        None
    }

    #[inline(always)]
    pub fn get_from_puts(&self, key: &Key) -> Option<&Value> {
        self.cached_puts.get(key)
    }

    #[inline(always)]
    pub fn get_mut_from_puts(&mut self, key: &Key) -> Option<&mut Value> {
        self.cached_puts.get_mut(key)
    }

    #[inline(always)]
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.remove_from_puts(key).or_else(|| {
            self.db_remove(key);

            None
        })
    }

    #[inline]
    pub fn db_remove(&mut self, key: &Key) {
        self.cached_dels.insert(key.clone());
    }

    #[inline]
    pub fn update(&mut self, key: Key, value: Value) -> Option<Value> {
        self.cached_dels.insert(key.clone());
        self.cached_puts.insert(key, value)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    #[inline]
    pub fn remove_from_puts(&mut self, key: &Key) -> Option<Value> {
        self.cached_puts.remove(key)
    }

    #[inline]
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.cached_dels.remove(&key);

        self.unsafe_insert(key, value)
    }

    #[inline]
    pub fn unsafe_insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.cached_puts.insert(key, value)
    }

    fn db_multi_put(&mut self, tree: BTreeMap<Key, Value>) -> Result<(), Error> {
        tree.into_iter()
            .try_for_each(|(key, value)| -> Result<(), Error> {
                btree::put(&mut self.txn, &mut self.db, &key, &value)?;
                Ok(())
            })
    }

    fn db_multi_del(&mut self, tree: BTreeSet<Key>) -> Result<(), Error> {
        tree.into_iter().try_for_each(|key| -> Result<(), Error> {
            btree::del(&mut self.txn, &mut self.db, &key, None)?;
            Ok(())
        })
    }
}

pub trait AnyDatabase {
    fn export(self) -> color_eyre::Result<(), Error>;
    fn boxed_export(self: Box<Self>) -> color_eyre::Result<(), Error>;
    #[allow(unused)]
    fn defragment(self);
    fn boxed_defragment(self: Box<Self>);
    fn destroy(self);
}

impl<Key, Value> AnyDatabase for Database<Key, Value>
where
    Key: Ord + Clone + Debug + Storable,
    Value: Storable + PartialEq + Clone,
{
    fn export(self) -> color_eyre::Result<(), Error> {
        Box::new(self).boxed_export()
    }

    fn boxed_export(mut self: Box<Self>) -> color_eyre::Result<(), Error> {
        if self.cached_dels.is_empty() && self.cached_puts.is_empty() {
            return Ok(());
        }

        let cached_dels = mem::take(&mut self.cached_dels);
        self.db_multi_del(cached_dels)?;

        let cached_puts = mem::take(&mut self.cached_puts);
        self.db_multi_put(cached_puts)?;

        self.txn.set_root(ROOT_DB, self.db.db.into());

        self.txn.commit()
    }

    fn defragment(self) {
        Box::new(self).boxed_defragment()
    }

    fn boxed_defragment(self: Box<Self>) {
        let btree = self.iter_collect();

        let path = self.path.to_owned();

        self.destroy();

        let mut db = Self::open(path).unwrap();

        if !db.is_empty() {
            panic!()
        }

        db.cached_puts = btree;
        db.export().unwrap();
    }

    fn destroy(self) {
        let path = self.path.to_owned();

        drop(self);

        fs::remove_file(&path).unwrap_or_else(|_| {
            dbg!(path);
            panic!("Error");
        });
    }
}
