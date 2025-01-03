use biter::bitcoin::Txid;
use derive_deref::{Deref, DerefMut};

use crate::structs::{Database, DatabaseTrait, Height, Txindex, Version};

#[derive(Deref, DerefMut)]
pub struct TxindexToTxid(Database);

impl TxindexToTxid {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import("txindex_to_txid", Self::version())?))
    }

    pub fn insert(&mut self, txindex: Txindex, txid: &Txid, height: Height) {
        self.0.insert(txindex.into(), txid[..].into(), height)
    }

    pub fn remove(&mut self, txindex: Txindex) {
        self.0.remove(txindex.into())
    }
}

impl DatabaseTrait for TxindexToTxid {
    fn version() -> Version {
        Version::from(1)
    }
}
