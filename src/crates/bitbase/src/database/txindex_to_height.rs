use derive_deref::{Deref, DerefMut};

use crate::structs::{Database, DatabaseTrait, Height, Txindex, Version};

#[derive(Deref, DerefMut)]
pub struct TxindexToHeight(Database);

impl TxindexToHeight {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "txindex_to_height",
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, txindex: Txindex, height: Height) {
        self.0.insert(txindex.into(), height.into(), height)
    }

    pub fn remove(&mut self, txindex: Txindex) {
        self.0.remove(txindex.into())
    }
}

impl DatabaseTrait for TxindexToHeight {
    fn version() -> Version {
        Version::from(1)
    }
}
