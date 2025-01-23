// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/

use core::panic;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    fs::{self, File},
    io, mem,
    path::{Path, PathBuf},
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
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    pathbuf: PathBuf,
    puts: BTreeMap<Key, Value>,
    dels: BTreeSet<Key>,
    len: usize,
    db: Db_<Key, Value, page::Page<Key, Value>>,
    txn: MutTxn<Env, ()>,
}

const ROOT_DB: usize = 0;
const PAGE_SIZE: u64 = 4096;

pub type UnitDatabase = Database<(), ()>;

const DEFRAGMENT_RATIO_THRESHOLD: f64 = 0.5;

impl<Key, Value> Database<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    const KEY_SIZE: usize = size_of::<Key>();
    const VALUE_SIZE: usize = size_of::<Value>();
    const KEY_AND_VALUE_SIZE: usize = Self::KEY_SIZE + Self::VALUE_SIZE;

    /// Open a database without a lock file where only one instance is safe to open.
    pub fn open(pathbuf: PathBuf) -> Result<Self, Error> {
        fs::create_dir_all(&pathbuf)?;

        let env = unsafe { Env::new_nolock(Self::path_sanakirja_(&pathbuf), PAGE_SIZE, 1)? };

        let mut txn = Env::mut_txn_begin(env)?;

        let db = txn
            .root_db(ROOT_DB)
            .unwrap_or_else(|| unsafe { btree::create_db_(&mut txn).unwrap() });

        Ok(Self {
            len: Self::read_length_(&pathbuf),
            pathbuf,
            puts: BTreeMap::default(),
            dels: BTreeSet::default(),
            db,
            txn,
        })
    }

    pub fn path_sanakirja(&self) -> PathBuf {
        Self::path_sanakirja_(&self.pathbuf)
    }
    fn path_sanakirja_(path: &Path) -> PathBuf {
        path.join("sanakirja")
    }

    pub fn read_length(&self) -> usize {
        Self::read_length_(&self.pathbuf)
    }
    pub fn read_length_(path: &Path) -> usize {
        fs::read(Self::path_length(path))
            .map(|v| {
                let mut buf = [0_u8; 8];
                v.iter().enumerate().take(8).for_each(|(i, b)| {
                    buf[i] = *b;
                });
                usize::from_le_bytes(buf)
            })
            .unwrap_or_default()
    }
    pub fn write_length(&self) -> Result<(), io::Error> {
        Self::write_length_(&self.pathbuf, self.len)
    }
    pub fn write_length_(path: &Path, len: usize) -> Result<(), io::Error> {
        fs::write(Self::path_length(path), len.to_le_bytes())
    }
    fn path_length(path: &Path) -> PathBuf {
        path.join("length")
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
        self.len += 1;
        self.puts.insert(key, value)
    }

    #[inline]
    pub fn update(&mut self, key: Key, value: Value) -> Option<Value> {
        self.dels.insert(key.clone());
        self.puts.insert(key, value)
    }

    #[inline]
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.len -= 1;
        self.puts.remove(key).or_else(|| {
            self.dels.insert(key.clone());
            None
        })
    }

    /// Get only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn remove_from_ram(&mut self, key: &Key) -> Option<Value> {
        self.len -= 1;
        self.puts.remove(key)
    }

    /// Add the key only to the dels tree without checking if it's present in the puts tree, only use if you are positive that you neither added nor updated an entry with this key
    #[inline]
    pub fn remove_later_from_disk(&mut self, key: &Key) {
        self.len -= 1;
        self.dels.insert(key.clone());
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
    pub fn collect_ram(&self) -> BTreeMap<Key, Value> {
        self.puts.clone()
    }

    /// Collect a **clone** of all key/value pairs from the database (disk)
    pub fn collect_disk(&self) -> BTreeMap<Key, Value> {
        self.iter_disk()
            .map(|r| r.unwrap())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<_>()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // pub fn export(self) -> Result<(), Error> {
    //     self.boxed().boxed_export()
    // }

    // pub fn boxed(self) -> Box<Self> {
    //     Box::new(self)
    // }

    pub fn get_file_size_to_data_ratio(&self) -> Result<f64, Error> {
        let data_bytes = (self.len() * Self::KEY_AND_VALUE_SIZE) as f64;
        let file_bytes = File::open(&self.pathbuf)?.metadata()?.len() as f64;
        Ok(file_bytes / data_bytes)
    }

    /// Flush all puts and dels from the ram to disk with an option to defragment the database to save some disk space
    ///
    /// /!\ Do not kill the program while this function is runnning  /!\
    pub fn export(mut self) -> Result<(), Error> {
        let defragment = self.get_file_size_to_data_ratio()? >= DEFRAGMENT_RATIO_THRESHOLD;

        if defragment {
            let mut btree = self.collect_disk();

            let disk_len = btree.len();
            let dels_len = self.dels.len();
            let puts_len = self.puts.len();

            let path = self.pathbuf.to_owned();
            self.dels.iter().for_each(|key| {
                btree.remove(key);
            });
            btree.append(&mut self.puts);

            let len = btree.len();

            if len != self.len {
                dbg!(len, self.len, path, disk_len, dels_len, puts_len);
                panic!("Len should be the same");
            }

            self.destroy()?;

            self = Self::open(path).unwrap();

            if !self.is_empty() {
                panic!()
            }

            self.len = len;
            self.puts = btree;
        }

        self.write_length()?;

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

    pub fn destroy(self) -> io::Result<()> {
        let path = self.pathbuf.to_owned();

        drop(self);

        fs::remove_dir_all(&path)
    }
}

pub trait AnyDatabase {
    fn export(self) -> Result<(), Error>;
    // fn boxed_export(self: Box<Self>) -> Result<(), Error>;
    fn destroy(self) -> io::Result<()>;
}

impl<Key, Value> AnyDatabase for Database<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    fn export(self) -> Result<(), Error> {
        self.export()
    }

    // fn boxed_export(self: Box<Self>) -> Result<(), Error> {
    //     self.boxed_export()
    // }

    fn destroy(self) -> io::Result<()> {
        self.destroy()
    }
}

pub trait DatabaseKey
where
    Self: Ord + Clone + Debug + Storable + Send + Sync,
{
}
impl<T> DatabaseKey for T where T: Ord + Clone + Debug + Storable + Send + Sync {}

pub trait DatabaseValue
where
    Self: Clone + Storable + PartialEq + Send + Sync,
{
}
impl<T> DatabaseValue for T where T: Clone + Storable + PartialEq + Send + Sync {}
