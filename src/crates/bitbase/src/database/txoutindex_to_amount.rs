use derive_deref::{Deref, DerefMut};

use crate::structs::{Amount, Database, DatabaseTrait, Height, Txoutindex, Version};

#[derive(Deref, DerefMut)]
pub struct TxoutindexToAmount(Database);

impl TxoutindexToAmount {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "txoutindex_to_amount",
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, txoutindex: Txoutindex, amount: Amount, height: Height) {
        self.0.insert(txoutindex.into(), amount.into(), height)
    }

    pub fn remove(&mut self, txoutindex: Txoutindex) {
        self.0.remove(txoutindex.into())
    }
}

impl DatabaseTrait for TxoutindexToAmount {
    fn version() -> Version {
        Version::from(1)
    }
}
