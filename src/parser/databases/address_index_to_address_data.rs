use std::{
    collections::BTreeMap,
    fs, mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use allocative::Allocative;
use itertools::Itertools;
use rayon::prelude::*;
use snkrj::{AnyDatabase, Database as _Database};

use crate::{
    parser::states::AddressCohortsDurableStates,
    structs::{AddressData, Config},
    utils::time,
};

use super::{AnyDatabaseGroup, Metadata};

type Key = u32;
type Value = AddressData;
type Database = _Database<Key, Value>;

#[derive(Allocative)]
pub struct AddressIndexToAddressData {
    path: PathBuf,
    pub metadata: Metadata,
    #[allocative(skip)]
    pub map: BTreeMap<usize, Database>,
}

impl Deref for AddressIndexToAddressData {
    type Target = BTreeMap<usize, Database>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for AddressIndexToAddressData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

pub const ADDRESS_INDEX_DB_MAX_SIZE: usize = 250_000;

impl AddressIndexToAddressData {
    pub fn insert_to_ram(&mut self, key: Key, value: Value) -> Option<Value> {
        self.metadata.called_insert();

        self.open_db(&key).insert_to_ram(key, value)
    }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.metadata.called_remove();

        self.open_db(key).remove(key)
    }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed
    /// Though it makes it easy to use with rayon.
    pub fn get_from_ram(&self, key: &Key) -> Option<&Value> {
        let db_index = Self::db_index(key);

        self.get(&db_index).unwrap().get_from_ram(key)
    }

    pub fn get_from_disk(&self, key: &Key) -> Option<&Value> {
        let db_index = Self::db_index(key);

        self.get(&db_index).unwrap().get_from_disk(key)
    }

    pub fn open_db(&mut self, key: &Key) -> &mut Database {
        let db_index = Self::db_index(key);
        let path = self.path().to_owned();

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

    pub fn compute_addres_cohorts_durable_states(&mut self) -> AddressCohortsDurableStates {
        time("Iter through address_index_to_address_data", || {
            self.open_all();

            // MUST CLEAR MAP, otherwise some weird things are happening later in the export I think
            mem::take(&mut self.map)
                .par_iter()
                .map(|(_, database)| {
                    let mut s = AddressCohortsDurableStates::default();

                    database
                        .iter_disk()
                        .map(|r| r.unwrap().1)
                        .for_each(|address_data| s.increment(address_data).unwrap());

                    s
                })
                .sum()
        })
    }

    fn db_index(key: &Key) -> usize {
        *key as usize / ADDRESS_INDEX_DB_MAX_SIZE
    }
}

impl AnyDatabaseGroup for AddressIndexToAddressData {
    fn import(config: &Config) -> Self {
        let path = config
            .path_databases()
            .join("address_index_to_address_data");
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
