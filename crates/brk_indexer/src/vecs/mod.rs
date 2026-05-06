use std::path::Path;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{AddrHash, Height, OutputType, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Rw, Stamp, StorageMode};

const PAGE_SIZE: usize = 4096;

use crate::parallel_import;

mod addrs;
mod blocks;
mod inputs;
mod macros;
mod outputs;
mod scripts;
mod transactions;

pub use addrs::*;
pub use blocks::*;
pub use inputs::*;
pub use outputs::*;
pub use scripts::*;
pub use transactions::*;

use crate::Lengths;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub db: Database,
    pub blocks: BlocksVecs<M>,
    #[traversable(wrap = "transactions", rename = "raw")]
    pub transactions: TransactionsVecs<M>,
    #[traversable(wrap = "inputs", rename = "raw")]
    pub inputs: InputsVecs<M>,
    #[traversable(wrap = "outputs", rename = "raw")]
    pub outputs: OutputsVecs<M>,
    #[traversable(wrap = "addrs", rename = "raw")]
    pub addrs: AddrsVecs<M>,
    #[traversable(wrap = "scripts", rename = "raw")]
    pub scripts: ScriptsVecs<M>,
}

impl Vecs {
    pub fn forced_import(parent: &Path, version: Version) -> Result<Self> {
        tracing::debug!("Opening vecs database...");
        let db = Database::open(&parent.join("vecs"))?;
        tracing::debug!("Setting min len...");
        db.set_min_len(PAGE_SIZE * 60_000_000)?;

        let (blocks, transactions, inputs, outputs, addrs, scripts) = parallel_import! {
            blocks = BlocksVecs::forced_import(&db, version),
            transactions = TransactionsVecs::forced_import(&db, version),
            inputs = InputsVecs::forced_import(&db, version),
            outputs = OutputsVecs::forced_import(&db, version),
            addrs = AddrsVecs::forced_import(&db, version),
            scripts = ScriptsVecs::forced_import(&db, version),
        };

        let this = Self {
            db,
            blocks,
            transactions,
            inputs,
            outputs,
            addrs,
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

    pub fn rollback_if_needed(&mut self, starting_lengths: &Lengths) -> Result<()> {
        let saved_height = starting_lengths.height.decremented().unwrap_or_default();
        let stamp = Stamp::from(u64::from(saved_height));

        self.blocks.truncate(starting_lengths.height, stamp)?;

        self.transactions
            .truncate(starting_lengths.height, starting_lengths.tx_index, stamp)?;

        self.inputs
            .truncate(starting_lengths.height, starting_lengths.txin_index, stamp)?;

        self.outputs
            .truncate(starting_lengths.height, starting_lengths.txout_index, stamp)?;

        self.addrs.truncate(
            starting_lengths.height,
            starting_lengths.p2pk65_addr_index,
            starting_lengths.p2pk33_addr_index,
            starting_lengths.p2pkh_addr_index,
            starting_lengths.p2sh_addr_index,
            starting_lengths.p2wpkh_addr_index,
            starting_lengths.p2wsh_addr_index,
            starting_lengths.p2tr_addr_index,
            starting_lengths.p2a_addr_index,
            stamp,
        )?;

        self.scripts.truncate(
            starting_lengths.height,
            starting_lengths.empty_output_index,
            starting_lengths.op_return_index,
            starting_lengths.p2ms_output_index,
            starting_lengths.unknown_output_index,
            stamp,
        )?;

        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.stamped_write(height)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn next_height(&mut self) -> Height {
        self.par_iter_mut_any_stored_vec()
            .map(|vec| {
                let h = Height::from(vec.stamp());
                if h > Height::ZERO { h.incremented() } else { h }
            })
            .min()
            .unwrap()
    }

    pub fn stamped_write(&mut self, height: Height) -> Result<()> {
        self.par_iter_mut_any_stored_vec()
            .try_for_each(|vec| vec.stamped_write(Stamp::from(height)))?;
        Ok(())
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

    pub fn iter_addr_hashes_from(
        &self,
        addr_type: OutputType,
        height: Height,
    ) -> Result<Box<dyn Iterator<Item = AddrHash> + '_>> {
        self.addrs.iter_hashes_from(addr_type, height)
    }

    fn par_iter_mut_any_stored_vec(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.blocks
            .par_iter_mut_any()
            .chain(self.transactions.par_iter_mut_any())
            .chain(self.inputs.par_iter_mut_any())
            .chain(self.outputs.par_iter_mut_any())
            .chain(self.addrs.par_iter_mut_any())
            .chain(self.scripts.par_iter_mut_any())
    }
}
