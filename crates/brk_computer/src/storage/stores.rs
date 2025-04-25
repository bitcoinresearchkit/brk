use std::path::Path;

use fjall::TransactionalKeyspace;

#[derive(Clone)]
pub struct Stores {
    // pub address_to_utxos_received: Store<AddressIndexOutputIndex, Unit>,
    // pub address_to_utxos_spent: Store<AddressIndexOutputIndex, Unit>,
}

impl Stores {
    pub fn import(_: &Path, _: &TransactionalKeyspace) -> color_eyre::Result<Self> {
        // let address_to_utxos_received = Store::import(
        //     keyspace.clone(),
        //     path,
        //     "address_to_utxos_received",
        //     Version::ZERO,
        // )?;
        // let address_to_utxos_spent = Store::import(
        //     keyspace.clone(),
        //     path,
        //     "address_to_utxos_spent",
        //     Version::ZERO,
        // )?;

        Ok(Self {
            // address_to_utxos_received,
            // address_to_utxos_spent,
        })
    }
}
