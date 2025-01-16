// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs, io, mem,
    path::PathBuf,
    result::Result,
};

use sanakirja::btree::{page, Db_};
pub use sanakirja::*;

///
/// A simple wrapper around Sanakirja aatabase that acts as a very fast on disk BTreeMap.
///
/// The state of the tree is uncommited until `.export()` is called during which it is unsafe to stop the program.
///
pub struct Database<Key, Value>
where
    Key: Ord + Clone + Debug + Storable,
    Value: Storable + PartialEq,
{
    path: PathBuf,
    puts: BTreeMap<Key, Value>,
    dels: BTreeSet<Key>,
    db: Db_<Key, Value, page::Page<Key, Value>>,
    txn: MutTxn<Env, ()>,
}

const ROOT_DB: usize = 0;
const PAGE_SIZE: u64 = 4096;

impl<Key, Value> Database<Key, Value>
where
    Key: Ord + Clone + Debug + Storable,
    Value: Storable + PartialEq,
{
    /// Open a database without a lock file where only one instance is safe to open.
    pub fn open(path: PathBuf) -> Result<Self, Error> {
        let env = unsafe { Env::new_nolock(&path, PAGE_SIZE, 1)? };

        let mut txn = Env::mut_txn_begin(env)?;

        let db = txn
            .root_db(ROOT_DB)
            .unwrap_or_else(|| unsafe { btree::create_db_(&mut txn).unwrap() });

        Ok(Self {
            path,
            puts: BTreeMap::default(),
            dels: BTreeSet::default(),
            db,
            txn,
        })
    }

    #[inline]
    pub fn get(&self, key: &Key) -> Option<&Value> {
        if let Some(cached_put) = self.get_from_ram(key) {
            return Some(cached_put);
        }

        self.get_from_disk(key)
    }

    /// Get only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn get_from_ram(&self, key: &Key) -> Option<&Value> {
        self.puts.get(key)
    }

    /// Get mut only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn get_mut_from_ram(&mut self, key: &Key) -> Option<&mut Value> {
        self.puts.get_mut(key)
    }

    /// Get only from the database (disk) without checking the uncommited tree (ram)
    #[inline]
    pub fn get_from_disk(&self, key: &Key) -> Option<&Value> {
        let option = btree::get(&self.txn, &self.db, key, None).unwrap();

        if let Some((key_found, v)) = option {
            if key == key_found {
                return Some(v);
            }
        }

        None
    }

    #[inline]
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.dels.remove(&key);
        self.insert_to_ram(key, value)
    }

    /// Insert without removing the key to the dels tree, so be sure that it hasn't added to the delete set
    #[inline]
    pub fn insert_to_ram(&mut self, key: Key, value: Value) -> Option<Value> {
        self.puts.insert(key, value)
    }

    #[inline]
    pub fn update(&mut self, key: Key, value: Value) -> Option<Value> {
        self.dels.insert(key.clone());
        self.puts.insert(key, value)
    }

    #[inline]
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.remove_from_ram(key).or_else(|| {
            self.remove_later_from_disk(key);

            None
        })
    }

    /// Get only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn remove_from_ram(&mut self, key: &Key) -> Option<Value> {
        self.puts.remove(key)
    }

    /// Add the key only to the dels tree without checking if it's present in the puts tree, only use if you are positive that you neither added nor updated an entry with this key
    #[inline]
    pub fn remove_later_from_disk(&mut self, key: &Key) {
        self.dels.insert(key.clone());
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.iter_disk().next().is_none()
    }

    /// Iterate over key/value pairs from the uncommited tree (ram)
    #[inline]
    pub fn iter_ram(&self) -> std::collections::btree_map::Iter<'_, Key, Value> {
        self.puts.iter()
    }

    /// Iterate over key/value pairs from the database (disk)
    #[inline]
    pub fn iter_disk(
        &self,
    ) -> btree::Iter<'_, MutTxn<Env, ()>, Key, Value, page::Page<Key, Value>> {
        btree::iter(&self.txn, &self.db, None).unwrap()
    }

    /// Iterate over key/value pairs
    #[inline]
    pub fn iter_ram_then_disk(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.iter_ram().chain(self.iter_disk().map(|r| r.unwrap()))
    }

    /// Collect a **clone** of all uncommited key/value pairs (ram)
    pub fn collect_ram(&self) -> BTreeMap<Key, Value>
    where
        Value: Clone,
    {
        self.puts.clone()
    }

    /// Collect a **clone** of all key/value pairs from the database (disk)
    pub fn collect_disk(&self) -> BTreeMap<Key, Value>
    where
        Value: Clone,
    {
        self.iter_disk()
            .map(|r| r.unwrap())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<_>()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.iter_ram_then_disk().count()
    }
}

pub trait AnyDatabase {
    #[allow(unused)]
    fn export(self, defragment: bool) -> Result<(), Error>;
    fn boxed_export(self: Box<Self>, defragment: bool) -> Result<(), Error>;
    #[allow(unused)]
    fn destroy(self) -> io::Result<()>;
}

impl<Key, Value> AnyDatabase for Database<Key, Value>
where
    Key: Ord + Clone + Debug + Storable,
    Value: Storable + PartialEq + Clone,
{
    /// Flush all puts and dels from the ram to disk with an option to defragment the database to save some disk space
    ///
    /// /!\ Do not kill the program while this function is runnning  /!\
    fn export(self, defragment: bool) -> Result<(), Error> {
        Box::new(self).boxed_export(defragment)
    }

    /// Flush all puts and dels from the ram to disk with an option to defragment the database to save some disk space
    ///
    /// /!\ Do not kill the program while this function is runnning  /!\
    fn boxed_export(mut self: Box<Self>, defragment: bool) -> Result<(), Error> {
        if defragment {
            let mut btree = self.as_ref().collect_disk();

            let path = self.path.to_owned();
            self.dels.iter().for_each(|key| {
                btree.remove(key);
            });
            btree.append(&mut self.puts);

            self.destroy()?;

            *self = Self::open(path).unwrap();

            if !self.is_empty() {
                panic!()
            }

            self.puts = btree;
        }

        if self.dels.is_empty() && self.puts.is_empty() {
            return Ok(());
        }

        mem::take(&mut self.dels)
            .into_iter()
            .try_for_each(|key| -> Result<(), Error> {
                btree::del(&mut self.txn, &mut self.db, &key, None)?;
                Ok(())
            })?;

        mem::take(&mut self.puts).into_iter().try_for_each(
            |(key, value)| -> Result<(), Error> {
                btree::put(&mut self.txn, &mut self.db, &key, &value)?;
                Ok(())
            },
        )?;

        self.txn.set_root(ROOT_DB, self.db.db.into());

        self.txn.commit()
    }

    fn destroy(self) -> io::Result<()> {
        let path = self.path.to_owned();

        drop(self);

        fs::remove_file(&path)
    }
}
