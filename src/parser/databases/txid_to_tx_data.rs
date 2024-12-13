use std::{
    collections::BTreeMap,
    fs, mem,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use allocative::Allocative;
use biter::bitcoin::Txid;
use itertools::Itertools;

use crate::structs::{Config, TxData, U8x31};

use super::{AnyDatabase, AnyDatabaseGroup, Database as _Database, Metadata};

type Key = U8x31;
type Value = TxData;
type Database = _Database<Key, Value>;

#[derive(Allocative)]
pub struct TxidToTxData {
    path: PathBuf,
    pub metadata: Metadata,
    map: BTreeMap<u16, Database>,
}

impl Deref for TxidToTxData {
    type Target = BTreeMap<u16, Database>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TxidToTxData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl TxidToTxData {
    pub fn insert(&mut self, txid: &Txid, tx_index: Value) -> Option<Value> {
        self.metadata.called_insert();

        let txid_key = Self::txid_to_key(txid);

        self.open_db(txid).insert(txid_key, tx_index)
    }

    // pub fn safe_get(&mut self, txid: &Txid) -> Option<&Value> {
    //     let txid_key = Self::txid_to_key(txid);
    //     self.open_db(txid).get(&txid_key)
    // }

    /// Doesn't check if the database is open contrary to `safe_get` which does and opens if needed.
    /// Though it makes it easy to use with rayon
    pub fn unsafe_get(&self, txid: &Txid) -> Option<&Value> {
        let txid_key = Self::txid_to_key(txid);

        let db_index = Self::db_index(txid);

        self.get(&db_index).unwrap().get(&txid_key)
    }

    // pub fn unsafe_get_from_puts(&self, txid: &Txid) -> Option<&Value> {
    //     let txid_key = Self::txid_to_key(txid);

    //     let db_index = Self::db_index(txid);

    //     self.get(&db_index).unwrap().get_from_puts(&txid_key)
    // }

    pub fn unsafe_get_mut_from_puts(&mut self, txid: &Txid) -> Option<&mut Value> {
        let txid_key = Self::txid_to_key(txid);

        let db_index = Self::db_index(txid);

        self.get_mut(&db_index)
            .unwrap()
            .get_mut_from_puts(&txid_key)
    }

    pub fn remove_from_db(&mut self, txid: &Txid) {
        self.metadata.called_remove();

        let txid_key = Self::txid_to_key(txid);

        self.open_db(txid).db_remove(&txid_key);
    }

    pub fn remove_from_puts(&mut self, txid: &Txid) {
        self.metadata.called_remove();

        let txid_key = Self::txid_to_key(txid);

        self.open_db(txid).remove_from_puts(&txid_key);
    }

    pub fn update(&mut self, txid: &Txid, tx_data: TxData) {
        let txid_key = Self::txid_to_key(txid);

        self.open_db(txid).update(txid_key, tx_data);
    }

    #[inline(always)]
    pub fn open_db(&mut self, txid: &Txid) -> &mut Database {
        let db_index = Self::db_index(txid);
        self._open_db(db_index)
    }

    #[inline(always)]
    pub fn _open_db(&mut self, db_index: u16) -> &mut Database {
        let path = self.path.to_owned();
        self.entry(db_index).or_insert_with(|| {
            let path = path.join(db_index.to_string());
            Database::open(path).unwrap()
        })
    }

    fn txid_to_key(txid: &Txid) -> U8x31 {
        U8x31::from(&txid[1..])
    }

    fn db_index(txid: &Txid) -> u16 {
        ((txid[0] as u16) << 5) + ((txid[1] as u16) >> 3)
    }
}

impl AnyDatabaseGroup for TxidToTxData {
    fn import(config: &Config) -> Self {
        let path = config.path_databases().join("txid_to_tx_data");
        let metadata = Metadata::import(&path, 2);

        Self {
            path,
            metadata,
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
            .flat_map(|entry| {
                entry
                    .unwrap()
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    .parse::<u16>()
            })
            .for_each(|db_index| {
                self._open_db(db_index);
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
