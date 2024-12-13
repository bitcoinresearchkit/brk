use std::{
    collections::BTreeMap,
    fs, mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use allocative::Allocative;
use itertools::Itertools;

use crate::structs::{Config, EmptyAddressData};

use super::{
    AnyDatabase, AnyDatabaseGroup, Database as _Database, Metadata, ADDRESS_INDEX_DB_MAX_SIZE,
};

type Key = u32;
type Value = EmptyAddressData;
type Database = _Database<Key, Value>;

#[derive(Allocative)]
pub struct AddressIndexToEmptyAddressData {
    path: PathBuf,
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
        let path = self.path.to_owned();

        self.entry(db_index).or_insert_with(|| {
            let db_name = format!(
                "{}..{}",
                db_index * ADDRESS_INDEX_DB_MAX_SIZE,
                (db_index + 1) * ADDRESS_INDEX_DB_MAX_SIZE
            );

            let path = path.join(db_name);

            Database::open(path).unwrap()
        })
    }

    fn db_index(key: &Key) -> usize {
        *key as usize / ADDRESS_INDEX_DB_MAX_SIZE
    }
}

impl AnyDatabaseGroup for AddressIndexToEmptyAddressData {
    fn import(config: &Config) -> Self {
        let path = config
            .path_databases()
            .join("address_index_to_empty_address_data");
        Self {
            metadata: Metadata::import(&path, 1),
            path,
            map: BTreeMap::default(),
        }
    }

    fn reset_metadata(&mut self) {
        self.metadata.reset();
    }

    fn open_all(&mut self) {
        let folder = fs::read_dir(&self.path);

        if folder.is_err() {
            return;
        }

        folder
            .unwrap()
            .map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .filter(|file_name| file_name.contains(".."))
            .for_each(|path| {
                self.open_db(&path.split("..").next().unwrap().parse::<u32>().unwrap());
            });
    }

    fn drain_to_vec(&mut self) -> Vec<Box<dyn AnyDatabase + Send>> {
        mem::take(&mut self.map)
            .into_values()
            .map(|db| Box::new(db) as Box<dyn AnyDatabase + Send>)
            .collect_vec()
    }

    fn metadata(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    fn path(&self) -> &Path {
        &self.path
    }
}
