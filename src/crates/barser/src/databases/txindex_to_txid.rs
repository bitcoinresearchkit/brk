use biter::bitcoin::Txid;
use derive_deref::{Deref, DerefMut};
use fjall::Slice;

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
        self.0.remove(Slice::from(txindex))
    }
}

impl DatabaseTrait for TxindexToTxid {
    fn version() -> Version {
        Version::from(1)
    }
}
