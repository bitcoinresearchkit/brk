use std::path::Path;

use bindex::{Store, Version};

use crate::structs::{AddressindexTxoutindex, Unit};

pub struct Fjalls {
    pub address_txoutindex_in: Store<AddressindexTxoutindex, Unit>,
    pub address_txoutindex_out: Store<AddressindexTxoutindex, Unit>,
}

impl Fjalls {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        let address_txoutindex_in = Store::import(&path.join("address_txoutindex_in"), Version::from(1))?;
        let address_txoutindex_out = Store::import(&path.join("address_txoutindex_out"), Version::from(1))?;

        Ok(Self {
            address_txoutindex_in,
            address_txoutindex_out,
        })
    }
}
