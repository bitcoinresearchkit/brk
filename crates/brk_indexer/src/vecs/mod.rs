use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{AddressHash, Height, OutputType, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Rw, Stamp, StorageMode};

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

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    db: Database,
    pub blocks: BlocksVecs<M>,
    #[traversable(wrap = "transactions", rename = "raw")]
    pub transactions: TransactionsVecs<M>,
    #[traversable(wrap = "inputs", rename = "raw")]
    pub inputs: InputsVecs<M>,
    #[traversable(wrap = "outputs", rename = "raw")]
    pub outputs: OutputsVecs<M>,
    #[traversable(wrap = "addresses", rename = "raw")]
    pub addresses: AddressesVecs<M>,
    #[traversable(wrap = "scripts", rename = "raw")]
    pub scripts: ScriptsVecs<M>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        tracing::debug!("Opening vecs database...");
        let db = Database::open(&parent.join("vecs"))?;
        tracing::debug!("Setting min len...");
        db.set_min_len(PAGE_SIZE * 50_000_000)?;

        let (blocks, transactions, inputs, outputs, addresses, scripts) = parallel_import! {
            blocks = BlocksVecs::forced_import(&db, version),
            transactions = TransactionsVecs::forced_import(&db, version),
            inputs = InputsVecs::forced_import(&db, version),
            outputs = OutputsVecs::forced_import(&db, version),
            addresses = AddressesVecs::forced_import(&db, version),
            scripts = ScriptsVecs::forced_import(&db, version),
        };

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
            .truncate(starting_indexes.height, starting_indexes.tx_index, stamp)?;

        self.inputs
            .truncate(starting_indexes.height, starting_indexes.txin_index, stamp)?;

        self.outputs
            .truncate(starting_indexes.height, starting_indexes.txout_index, stamp)?;

        self.addresses.truncate(
            starting_indexes.height,
            starting_indexes.p2pk65_address_index,
            starting_indexes.p2pk33_address_index,
            starting_indexes.p2pkh_address_index,
            starting_indexes.p2sh_address_index,
            starting_indexes.p2wpkh_address_index,
            starting_indexes.p2wsh_address_index,
            starting_indexes.p2tr_address_index,
            starting_indexes.p2a_address_index,
            stamp,
        )?;

        self.scripts.truncate(
            starting_indexes.height,
            starting_indexes.empty_output_index,
            starting_indexes.op_return_index,
            starting_indexes.p2ms_output_index,
            starting_indexes.unknown_output_index,
            stamp,
        )?;

        Ok(())
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
