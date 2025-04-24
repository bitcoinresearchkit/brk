use std::path::Path;

use brk_core::{AddressindexTxoutindex, Unit};
use brk_indexer::Store;
use brk_vec::Version;
use fjall::TransactionalKeyspace;

#[derive(Clone)]
pub struct Stores {
    pub address_to_utxos_received: Store<AddressindexTxoutindex, Unit>,
    pub address_to_utxos_spent: Store<AddressindexTxoutindex, Unit>,
}

impl Stores {
    pub fn import(path: &Path, keyspace: &TransactionalKeyspace) -> color_eyre::Result<Self> {
        let address_to_utxos_received = Store::import(
            keyspace.clone(),
            path,
            "address_to_utxos_received",
            Version::ZERO,
        )?;
        let address_to_utxos_spent = Store::import(
            keyspace.clone(),
            path,
            "address_to_utxos_spent",
            Version::ZERO,
        )?;

        Ok(Self {
            address_to_utxos_received,
            address_to_utxos_spent,
        })
    }
}
