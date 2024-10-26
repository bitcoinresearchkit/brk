use std::{
    collections::BTreeMap,
    mem,
    ops::{Deref, DerefMut},
};

use allocative::Allocative;
use rayon::prelude::*;

use crate::structs::{Date, EmptyAddressData, Height};

use super::{AnyDatabaseGroup, Database as _Database, Metadata, ADDRESS_INDEX_DB_MAX_SIZE};

type Key = u32;
type Value = EmptyAddressData;
type Database = _Database<Key, Value>;

#[derive(Allocative)]
pub struct AddressIndexToEmptyAddressData {
    pub metadata: Metadata,

    map: BTreeMap<usize, Database>,
}

impl Deref for AddressIndexToEmptyAddressData {
    type Target = BTreeMap<usize, Database>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for AddressIndexToEmptyAddressData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl AddressIndexToEmptyAddressData {
    pub fn unsafe_insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.metadata.called_insert();

        self.open_db(&key).unsafe_insert(key, value)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.metadata.called_remove();

        self.open_db(key).remove(key)
    }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed
    /// Though it makes it easy to use with rayon.
    pub fn unsafe_get_from_cache(&self, key: &Key) -> Option<&Value> {
        let db_index = Self::db_index(key);

        self.get(&db_index).and_then(|db| db.get_from_puts(key))
    }

    pub fn unsafe_get_from_db(&self, key: &Key) -> Option<&Value> {
        let db_index = Self::db_index(key);

        self.get(&db_index)
            .unwrap_or_else(|| {
                dbg!(&self.map.keys(), &key, &db_index);
                panic!()
            })
            .db_get(key)
    }

    pub fn open_db(&mut self, key: &Key) -> &mut Database {
        let db_index = Self::db_index(key);

        self.entry(db_index).or_insert_with(|| {
            let db_name = format!(
                "{}..{}",
                db_index * ADDRESS_INDEX_DB_MAX_SIZE,
                (db_index + 1) * ADDRESS_INDEX_DB_MAX_SIZE
            );

            Database::open(Self::folder(), &db_name).unwrap()
        })
    }

    fn db_index(key: &Key) -> usize {
        *key as usize / ADDRESS_INDEX_DB_MAX_SIZE
    }
}

impl AnyDatabaseGroup for AddressIndexToEmptyAddressData {
    fn import() -> Self {
        Self {
            metadata: Metadata::import(&Self::full_path(), 1),

            map: BTreeMap::default(),
        }
    }

    fn export(&mut self, height: Height, date: Date) -> color_eyre::Result<()> {
        mem::take(&mut self.map)
            .into_par_iter()
            .try_for_each(|(_, db)| db.export())?;

        self.metadata.export(height, date)?;

        Ok(())
    }

    fn reset_metadata(&mut self) {
        self.metadata.reset();
    }

    fn folder<'a>() -> &'a str {
        "address_index_to_empty_address_data"
    }
}
