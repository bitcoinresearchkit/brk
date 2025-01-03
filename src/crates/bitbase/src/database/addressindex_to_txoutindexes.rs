use derive_deref::{Deref, DerefMut};
use fjall::Slice;

use crate::structs::{
    Addressindex, Addresstxoutindex, Database, DatabaseTrait, Height, SliceExtended, Txoutindex,
    Version,
};

#[derive(Deref, DerefMut)]
pub struct AddressindexToTxoutindexes(Database);

impl AddressindexToTxoutindexes {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "addressindex_to_txoutindexes",
            Self::version(),
        )?))
    }

    pub fn insert(&mut self, addressindex: Addressindex, txoutindex: Txoutindex, height: Height) {
        self.0.insert(
            Addresstxoutindex::from((addressindex, txoutindex)).into(),
            Slice::default(),
            height,
        )
    }

    pub fn remove(&mut self, addressindex: Addressindex, txoutindex: Txoutindex) {
        self.0
            .remove(Addresstxoutindex::from((addressindex, txoutindex)).into());
    }

    pub fn is_empty(&self, addressindex: Addressindex) -> bool {
        self.prefix(Slice::from(addressindex)).next().is_none()
    }
}

impl DatabaseTrait for AddressindexToTxoutindexes {
    fn version() -> Version {
        Version::from(1)
    }
}
