use derive_deref::{Deref, DerefMut};

use crate::structs::{Addressindex, Database, DatabaseTrait, Height, Txoutindex, Version};

#[derive(Deref, DerefMut)]
pub struct TxoutindexToAddressindex(Database);

impl TxoutindexToAddressindex {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "txoutindex_to_addressindex",
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, txoutindex: Txoutindex, addressindex: Addressindex, height: Height) {
        self.0
            .insert(txoutindex.into(), addressindex.into(), height)
    }

    pub fn remove(&mut self, txoutindex: Txoutindex) {
        self.0.remove(txoutindex.into())
    }
}

impl DatabaseTrait for TxoutindexToAddressindex {
    fn version() -> Version {
        Version::from(1)
    }
}
