mod metadata;
mod tx;
mod txin;
mod txout;
mod types;

pub use types::*;

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_types::{AddressHash, Block, Height, OutPoint, TxInIndex, TxIndex, TxOutIndex, TypeIndex};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{Indexes, Readers, Stores, Vecs};

/// Processes a single block, extracting and storing all indexed data.
pub struct BlockProcessor<'a> {
    pub block: &'a Block,
    pub height: Height,
    pub check_collisions: bool,
    pub indexes: &'a mut Indexes,
    pub vecs: &'a mut Vecs,
    pub stores: &'a mut Stores,
    pub readers: &'a Readers,
}

impl BlockProcessor<'_> {
    /// Update global indexes after processing a block.
    pub fn update_indexes(&mut self, tx_count: usize, input_count: usize, output_count: usize) {
        self.indexes.txindex += TxIndex::from(tx_count);
        self.indexes.txinindex += TxInIndex::from(input_count);
        self.indexes.txoutindex += TxOutIndex::from(output_count);
    }

    /// Finalizes outputs/inputs in parallel with storing tx metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn finalize_and_store_metadata(
        &mut self,
        txs: Vec<ComputedTx>,
        txouts: Vec<ProcessedOutput>,
        txins: Vec<(TxInIndex, InputSource)>,
        same_block_spent_outpoints: &FxHashSet<OutPoint>,
        already_added: &mut ByAddressType<FxHashMap<AddressHash, TypeIndex>>,
        same_block_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
    ) -> Result<()> {
        let height = self.height;
        let indexes = &mut *self.indexes;

        // Split transactions vecs: finalize needs first_txoutindex/first_txinindex, metadata needs the rest
        let (first_txoutindex, first_txinindex, mut tx_metadata) =
            self.vecs.transactions.split_for_finalize();

        let outputs = &mut self.vecs.outputs;
        let inputs = &mut self.vecs.inputs;
        let addresses = &mut self.vecs.addresses;
        let scripts = &mut self.vecs.scripts;

        let addr_hash_stores = &mut self.stores.addresstype_to_addresshash_to_addressindex;
        let addr_txindex_stores = &mut self.stores.addresstype_to_addressindex_and_txindex;
        let addr_outpoint_stores = &mut self.stores.addresstype_to_addressindex_and_unspentoutpoint;
        let txidprefix_store = &mut self.stores.txidprefix_to_txindex;

        let (finalize_result, metadata_result) = rayon::join(
            || -> Result<()> {
                txout::finalize_outputs(
                    indexes,
                    first_txoutindex,
                    outputs,
                    addresses,
                    scripts,
                    addr_hash_stores,
                    addr_txindex_stores,
                    addr_outpoint_stores,
                    txouts,
                    same_block_spent_outpoints,
                    already_added,
                    same_block_info,
                )?;
                txin::finalize_inputs(
                    first_txinindex,
                    inputs,
                    addr_txindex_stores,
                    addr_outpoint_stores,
                    txins,
                    same_block_info,
                )
            },
            || tx::store_tx_metadata(height, txs, txidprefix_store, &mut tx_metadata),
        );

        finalize_result?;
        metadata_result?;
        Ok(())
    }
}
