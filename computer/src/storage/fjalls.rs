use std::path::Path;

use indexer::Store;
use storable_vec::Version;

use crate::structs::{AddressindexTxoutindex, Unit};

pub struct Fjalls {
    pub address_to_utxos_received: Store<AddressindexTxoutindex, Unit>,
    pub address_to_utxos_spent: Store<AddressindexTxoutindex, Unit>,
}

impl Fjalls {
    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        let address_to_utxos_received = Store::import(&path.join("address_to_utxos_received"), Version::from(1))?;
        let address_to_utxos_spent = Store::import(&path.join("address_to_utxos_spent"), Version::from(1))?;

        Ok(Self {
            address_to_utxos_received,
            address_to_utxos_spent,
        })
    }
}
