use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{AddressBytes, AddressHash, Height, OutputType, TypeIndex, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Stamp};

use crate::AddressReaders;

const PAGE_SIZE: usize = 4096;

use crate::parallel_import;

mod addresses;
mod blocks;
mod inputs;
mod macros;
mod outputs;
mod scripts;
mod transactions;

pub use addresses::*;
pub use blocks::*;
pub use inputs::*;
pub use outputs::*;
pub use scripts::*;
pub use transactions::*;

use crate::Indexes;

#[derive(Clone, Traversable)]
pub struct Vecs {
    db: Database,
    pub blocks: BlocksVecs,
    pub transactions: TransactionsVecs,
    pub inputs: InputsVecs,
    pub outputs: OutputsVecs,
    pub addresses: AddressesVecs,
    pub scripts: ScriptsVecs,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        tracing::debug!("Opening vecs database...");
        let db = Database::open(&parent.join("vecs"))?;
        tracing::debug!("Setting min len...");
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        tracing::debug!("Importing sub-vecs in parallel...");
        let (blocks, transactions, inputs, outputs, addresses, scripts) = parallel_import! {
            blocks = {
                tracing::debug!("Importing BlocksVecs...");
                let r = BlocksVecs::forced_import(&db, version);
                tracing::debug!("BlocksVecs imported.");
                r
            },
            transactions = {
                tracing::debug!("Importing TransactionsVecs...");
                let r = TransactionsVecs::forced_import(&db, version);
                tracing::debug!("TransactionsVecs imported.");
                r
            },
            inputs = {
                tracing::debug!("Importing InputsVecs...");
                let r = InputsVecs::forced_import(&db, version);
                tracing::debug!("InputsVecs imported.");
                r
            },
            outputs = {
                tracing::debug!("Importing OutputsVecs...");
                let r = OutputsVecs::forced_import(&db, version);
                tracing::debug!("OutputsVecs imported.");
                r
            },
            addresses = {
                tracing::debug!("Importing AddressesVecs...");
                let r = AddressesVecs::forced_import(&db, version);
                tracing::debug!("AddressesVecs imported.");
                r
            },
            scripts = {
                tracing::debug!("Importing ScriptsVecs...");
                let r = ScriptsVecs::forced_import(&db, version);
                tracing::debug!("ScriptsVecs imported.");
                r
            },
        };
        tracing::debug!("Sub-vecs imported.");

        let this = Self {
            db,
            blocks,
            transactions,
            inputs,
            outputs,
            addresses,
            scripts,
        };

        this.db.retain_regions(
            this.iter_any_exportable()
                .flat_map(|v| v.region_names())
                .collect(),
        )?;
        this.db.compact()?;

        Ok(this)
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();
        let stamp = Stamp::from(u64::from(saved_height));

        self.blocks.truncate(starting_indexes.height, stamp)?;

        self.transactions
            .truncate(starting_indexes.height, starting_indexes.txindex, stamp)?;

        self.inputs
            .truncate(starting_indexes.height, starting_indexes.txinindex, stamp)?;

        self.outputs
            .truncate(starting_indexes.height, starting_indexes.txoutindex, stamp)?;

        self.addresses.truncate(
            starting_indexes.height,
            starting_indexes.p2pk65addressindex,
            starting_indexes.p2pk33addressindex,
            starting_indexes.p2pkhaddressindex,
            starting_indexes.p2shaddressindex,
            starting_indexes.p2wpkhaddressindex,
            starting_indexes.p2wshaddressindex,
            starting_indexes.p2traddressindex,
            starting_indexes.p2aaddressindex,
            stamp,
        )?;

        self.scripts.truncate(
            starting_indexes.height,
            starting_indexes.emptyoutputindex,
            starting_indexes.opreturnindex,
            starting_indexes.p2msoutputindex,
            starting_indexes.unknownoutputindex,
            stamp,
        )?;

        Ok(())
    }

    pub fn get_addressbytes_by_type(
        &self,
        addresstype: OutputType,
        typeindex: TypeIndex,
        readers: &AddressReaders,
    ) -> Option<AddressBytes> {
        self.addresses
            .get_bytes_by_type(addresstype, typeindex, readers)
    }

    pub fn push_bytes_if_needed(&mut self, index: TypeIndex, bytes: AddressBytes) -> Result<()> {
        self.addresses.push_bytes_if_needed(index, bytes)
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.par_iter_mut_any_stored_vec()
            .try_for_each(|vec| vec.stamped_write(Stamp::from(height)))?;
        self.db.flush()?;
        Ok(())
    }

    pub fn starting_height(&mut self) -> Height {
        self.par_iter_mut_any_stored_vec()
            .map(|vec| {
                let h = Height::from(vec.stamp());
                if h > Height::ZERO { h.incremented() } else { h }
            })
            .min()
            .unwrap()
    }

    pub fn compact(&self) -> Result<()> {
        self.db.compact()?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.par_iter_mut_any_stored_vec()
            .try_for_each(|vec| vec.any_reset())?;
        Ok(())
    }

    pub fn iter_address_hashes_from(
        &self,
        address_type: OutputType,
        height: Height,
    ) -> Result<Box<dyn Iterator<Item = AddressHash> + '_>> {
        self.addresses.iter_hashes_from(address_type, height)
    }

    fn par_iter_mut_any_stored_vec(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.blocks
            .par_iter_mut_any()
            .chain(self.transactions.par_iter_mut_any())
            .chain(self.inputs.par_iter_mut_any())
            .chain(self.outputs.par_iter_mut_any())
            .chain(self.addresses.par_iter_mut_any())
            .chain(self.scripts.par_iter_mut_any())
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}
