use biter::bitcoin::Txid;
use color_eyre::eyre::eyre;
use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use crate::structs::{Database, DatabaseTrait, Height, Txindex, Version};

#[derive(Deref, DerefMut)]
pub struct TxidPrefixToTxindex(Database);

impl TxidPrefixToTxindex {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "txid_prefix_to_txindex",
            Self::version(),
        )?))
    }

    pub fn insert(
        &mut self,
        txid: &Txid,
        txindex: Txindex,
        height: Height,
    ) -> color_eyre::Result<()> {
        if let Some(_txindex) =
            self.fetch_update(Self::txid_to_key(txid), txindex.into(), height)?
        {
            return Err(eyre!("TxidPrefixToTxindex: key collision"));
        }
        Ok(())
    }

    pub fn remove(&mut self, txid: &Txid) {
        self.0.remove(Self::txid_to_key(txid))
    }

    fn txid_to_key(txid: &Txid) -> Slice {
        txid[0..8].into()
    }
}

impl DatabaseTrait for TxidPrefixToTxindex {
    fn version() -> Version {
        Version::from(1)
    }
}
