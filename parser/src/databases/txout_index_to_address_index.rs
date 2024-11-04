use std::{
    collections::BTreeMap,
    fs, mem,
    ops::{Deref, DerefMut},
};

use allocative::Allocative;
use itertools::Itertools;

use crate::structs::{Date, Height, TxoutIndex};

use super::{AnyDatabase, AnyDatabaseGroup, Database as _Database, Metadata};

type Key = TxoutIndex;
type Value = u32;
type Database = _Database<Key, Value>;

#[derive(Allocative)]
pub struct TxoutIndexToAddressIndex {
    pub metadata: Metadata,

    map: BTreeMap<usize, Database>,
}

impl Deref for TxoutIndexToAddressIndex {
    type Target = BTreeMap<usize, Database>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TxoutIndexToAddressIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

const DB_MAX_SIZE: usize = 10_000_000_000;

impl TxoutIndexToAddressIndex {
    pub fn unsafe_insert(&mut self, key: Key, value: Value) -> Option<Value> {
        self.metadata.called_insert();

        self.open_db(&key).unsafe_insert(key, value)
    }

    // pub fn undo_insert(&mut self, key: &Key) -> Option<Value> {
    //     self.open_db(key).remove_from_puts(key).map(|v| {
    //         self.metadata.called_remove();

    //         v
    //     })
    // }

    pub fn remove(&mut self, key: &Key) -> Option<Value> {
        self.metadata.called_remove();

        self.open_db(key).remove(key)
    }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed
    /// Though it makes it easy to use with rayon.
    pub fn unsafe_get(&self, key: &Key) -> Option<&Value> {
        let db_index = Self::db_index(key);

        self.get(&db_index).unwrap().get(key)
    }

    pub fn open_db(&mut self, key: &Key) -> &mut Database {
        let db_index = Self::db_index(key);

        self.entry(db_index).or_insert_with(|| {
            let db_name = format!(
                "{}..{}",
                db_index * DB_MAX_SIZE,
                (db_index + 1) * DB_MAX_SIZE
            );

            let path = Self::root().join(db_name);

            Database::open(path).unwrap()
        })
    }

    fn db_index(key: &Key) -> usize {
        key.as_u64() as usize / DB_MAX_SIZE
    }
}

impl AnyDatabaseGroup for TxoutIndexToAddressIndex {
    fn import() -> Self {
        Self {
            metadata: Metadata::import(Self::root(), 1),

            map: BTreeMap::default(),
        }
    }

    fn create_dir_all(&self) -> color_eyre::Result<(), std::io::Error> {
        fs::create_dir_all(Self::root())
    }

    fn reset_metadata(&mut self) {
        self.metadata.reset();
    }

    fn folder<'a>() -> &'a str {
        "txout_index_to_address_index"
    }

    fn open_all(&mut self) {
        let path = Self::root();

        let folder = fs::read_dir(path);

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
                self.open_db(
                    &path
                        .split("..")
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                        .into(),
                );
            });
    }

    fn drain_to_vec(&mut self) -> Vec<Box<dyn AnyDatabase + Send>> {
        mem::take(&mut self.map)
            .into_values()
            .map(|db| Box::new(db) as Box<dyn AnyDatabase + Send>)
            .collect_vec()
    }

    fn export_metadata(&mut self, height: Height, date: Date) -> color_eyre::Result<()> {
        self.metadata.export(height, date)
    }
}
