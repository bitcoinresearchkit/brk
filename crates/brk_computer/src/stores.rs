use std::path::Path;

use brk_vec::Version;
use fjall::TransactionalKeyspace;

const _VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Stores {
    // pub address_to_utxos_received: Store<AddressIndexOutputIndex, Unit>,
    // pub address_to_utxos_spent: Store<AddressIndexOutputIndex, Unit>,
}

impl Stores {
    pub fn import(_: &Path, _: Version, _: &TransactionalKeyspace) -> color_eyre::Result<Self> {
        // let address_to_utxos_received = Store::import(
        //     keyspace.clone(),
        //     path,
        //     "address_to_utxos_received",
        //     version + VERSION + Version::ZERO,
        // )?;
        // let address_to_utxos_spent = Store::import(
        //     keyspace.clone(),
        //     path,
        //     "address_to_utxos_spent",
        //     version + VERSION + Version::ZERO,
        // )?;

        Ok(Self {
            // address_to_utxos_received,
            // address_to_utxos_spent,
        })
    }
}
