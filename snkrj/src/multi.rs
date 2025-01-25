// https://docs.rs/sanakirja/latest/sanakirja/index.html
// https://pijul.org/posts/2021-02-06-rethinking-sanakirja/

use core::panic;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, mem,
    path::PathBuf,
    result::Result,
};

use sanakirja::btree::page;
pub use sanakirja::*;

use crate::{AnyDatabase, Base, DatabaseKey, DatabaseValue};

///
/// A simple wrapper around Sanakirja aatabase that acts as a very fast on disk BTreeMap.
///
/// The state of the tree is uncommited until `.export()` is called during which it is unsafe to stop the program.
///
pub struct DatabaseMulti<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    puts: BTreeMap<Key, Vec<Value>>,
    dels: BTreeSet<Key>,
    len: usize,
    db: Base<Key, Value>,
}

impl<Key, Value> DatabaseMulti<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    /// Open a database without a lock file where only one instance is safe to open.
    pub fn open(pathbuf: PathBuf) -> Result<Self, Error> {
        let db = Base::open(pathbuf)?;
        Ok(Self {
            len: db.read_length(),
            puts: BTreeMap::default(),
            dels: BTreeSet::default(),
            db,
        })
    }

    #[inline]
    pub fn get(&self, key: &Key) -> Result<Option<&Value>, Error> {
        if let Some(cached_put) = self.get_uncommited(key) {
            return Ok(Some(cached_put));
        }

        self.db.get(key)
    }

    /// Get only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn get_uncommited(&self, key: &Key) -> Option<&Value> {
        self.puts.get(key).and_then(|v| v.first())
    }

    /// Get mut only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn get_mut_uncommited(&mut self, key: &Key) -> Option<&mut Value> {
        self.puts.get_mut(key).and_then(|v| v.first_mut())
    }

    #[inline]
    pub fn insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.dels.remove(&key);
        self.unchecked_insert(key, value)
    }

    /// Insert without removing the key to the dels tree, so be sure that it hasn't added to the delete set
    #[inline]
    pub fn unchecked_insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.len += 1;
        self.puts.entry(key).or_default().push(value);
        None
    }

    #[inline]
    pub fn update(&mut self, key: Key, value: Value) -> Option<Value> {
        todo!()
        // self.dels.insert(key.clone());
        // self.puts.insert(key, value)
    }

    #[inline]
    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        todo!()
        // self.len -= 1;
        // self.puts.remove(key).or_else(|| {
        //     self.dels.insert(key.clone());
        //     None
        // })
    }

    /// Remove only from the uncommited tree (ram) without checking the database (disk)
    #[inline]
    pub fn remove_from_uncommited(&mut self, key: &Key) -> Option<Value> {
        todo!()
        // self.len -= 1;
        // self.puts.remove(key)
    }

    /// Add the key only to the dels tree without checking if it's present in the puts tree, only use if you are positive that you neither added nor updated an entry with this key
    #[inline]
    pub fn remove_later_from_disk(&mut self, key: &Key) {
        todo!()
        // self.len -= 1;
        // self.dels.insert(key.clone());
    }

    /// Iterate over key/value pairs from the uncommited tree (ram)
    #[inline]
    pub fn iter_ram(&self) -> std::collections::btree_map::Iter<'_, Key, Vec<Value>> {
        self.puts.iter()
    }

    /// Iterate over key/value pairs from the database (disk)
    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn iter_disk(
        &self,
    ) -> Result<btree::Iter<'_, MutTxn<Env, ()>, Key, Value, page::Page<Key, Value>>, Error> {
        self.db.iter()
    }

    /// Iterate over key/value pairs
    // #[inline]
    // pub fn iter_ram_then_disk(&self) -> Result<impl Iterator<Item = (&Key, &Value)>, Error> {
    //     todo!();
    //     // Ok(self.iter_ram().chain(self.iter_disk()?.map(|r| r.unwrap())))
    // }

    /// Collect a **clone** of all uncommited key/value pairs (ram)
    pub fn collect_ram(&self) -> BTreeMap<Key, Value> {
        todo!()
        // self.puts.clone()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Flush all puts and dels from the ram to disk with an option to defragment the database to save some disk space
    ///
    /// /!\ Do not kill the program while this function is runnning  /!\
    pub fn export(mut self) -> Result<(), Error> {
        if self.dels.is_empty() && self.puts.is_empty() {
            return Ok(());
        }

        if self.db.should_defragment(self.len)? {
            let mut btree = self.db.iter_collect_multi()?;
            // TODO:
            // self.dels.iter().for_each(|key| {
            //     btree.remove(key);
            // });
            self.puts.iter().for_each(|(key, values)| {
                // btree.insert(key, value);
                let vec = btree.entry(key).or_default();
                vec.extend(values.iter());
            });

            let path_self_original = self.db.path().to_owned();
            let path_self_defragmented = self.db.path_self_defragmented();

            let len = btree.values().map(|v| v.len()).sum::<usize>();

            if len != self.len {
                dbg!(len, self.len, path_self_defragmented);
                panic!("Len should be the same");
            }

            {
                let mut defragmented = Self::open(path_self_defragmented.clone()).unwrap();

                btree
                    .into_iter()
                    .try_for_each(|(key, values)| -> Result<(), Error> {
                        values
                            .into_iter()
                            .try_for_each(|value| -> Result<(), Error> {
                                defragmented.db.put(key, value)?;
                                Ok(())
                            })?;
                        Ok(())
                    })?;

                defragmented.len = len;
                defragmented.db.commit(self.len)?;
            }

            drop(self);

            fs::remove_dir_all(&path_self_original)?;
            fs::rename(&path_self_defragmented, &path_self_original)?;

            Ok(())
        } else {
            mem::take(&mut self.dels)
                .into_iter()
                .try_for_each(|key| -> Result<(), Error> {
                    self.db.del(&key, None)?;
                    Ok(())
                })?;

            mem::take(&mut self.puts).into_iter().try_for_each(
                |(key, vec): (Key, Vec<Value>)| -> Result<(), Error> {
                    vec.into_iter().try_for_each(|value| -> Result<(), Error> {
                        self.db.put(&key, &value)?;
                        Ok(())
                    })
                },
            )?;

            self.db.commit(self.len)
        }
    }
}

impl<Key, Value> AnyDatabase for DatabaseMulti<Key, Value>
where
    Key: DatabaseKey,
    Value: DatabaseValue,
{
    fn export(self) -> Result<(), Error> {
        self.export()
    }
}
