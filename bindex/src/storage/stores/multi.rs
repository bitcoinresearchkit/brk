use std::{array, path::Path, sync::OnceLock};

use rayon::prelude::*;
use snkrj::{DatabaseKey, DatabaseMulti, DatabaseValue};

use super::{Height, StoreMeta, Version};

pub struct StoreMulti<K, V>
where
    K: DatabaseKey,
    V: DatabaseValue,
{
    meta: StoreMeta,
    pub parts: [OnceLock<Box<DatabaseMulti<K, V>>>; 256],
}

impl<K, V> StoreMulti<K, V>
where
    K: DatabaseKey,
    V: DatabaseValue,
{
    pub fn open(path: &Path, version: Version) -> Result<Self, snkrj::Error> {
        let meta = StoreMeta::checked_open(path, version)?;

        Ok(Self {
            meta,
            parts: array::from_fn(|_| OnceLock::new()),
        })
    }

    // pub fn len(&self) -> usize {
    //     self.meta.len()
    // }

    fn get_or_init_store(&self, key: &K) -> &DatabaseMulti<K, V> {
        self.get_or_init_store_(key.as_ne_byte() as usize)
    }

    fn get_or_init_store_(&self, storeindex: usize) -> &DatabaseMulti<K, V> {
        self.parts[storeindex]
            .get_or_init(|| Box::new(DatabaseMulti::open(self.meta.path_parts().join(storeindex.to_string())).unwrap()))
    }

    fn get_or_init_mut_store(&mut self, key: &K) -> &mut DatabaseMulti<K, V> {
        self.get_or_init_store(key);

        self.parts
            .get_mut(key.as_ne_byte() as usize)
            .unwrap()
            .get_mut()
            .unwrap()
    }

    #[allow(unused)]
    pub fn open_all(&self) {
        (0..=(u8::MAX) as usize).for_each(|storeindex| {
            self.get_or_init_store_(storeindex);
        });
    }

    #[allow(unused)]
    pub fn get(&self, key: &K) -> Result<Option<&V>, snkrj::Error> {
        self.get_or_init_store(key).get(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.meta.len += 1;
        self.get_or_init_mut_store(&key).insert(key, value)
    }

    pub fn insert_if_needed(&mut self, key: K, value: V, height: Height) {
        if self.meta.needs(height) {
            self.insert(key, value);
        }
    }

    pub fn export(self, height: Height) -> Result<(), snkrj::Error> {
        if self.has(height) {
            return Ok(());
        }

        self.meta.export(height)?;

        self.parts.into_par_iter().try_for_each(|s| {
            if let Some(db) = s.into_inner() {
                db.export()
            } else {
                Ok(())
            }
        })
    }

    pub fn height(&self) -> Option<&Height> {
        self.meta.height()
    }
    #[allow(unused)]
    pub fn needs(&self, height: Height) -> bool {
        self.meta.needs(height)
    }
    pub fn has(&self, height: Height) -> bool {
        self.meta.has(height)
    }
}
