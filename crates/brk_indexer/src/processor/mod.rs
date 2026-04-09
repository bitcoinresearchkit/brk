mod metadata;
mod tx;
mod txin;
mod txout;
mod types;

pub use types::*;

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_types::{AddrHash, Block, Height, OutPoint, TxInIndex, TxIndex, TxOutIndex, TypeIndex};
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
        self.indexes.tx_index += TxIndex::from(tx_count);
        self.indexes.txin_index += TxInIndex::from(input_count);
        self.indexes.txout_index += TxOutIndex::from(output_count);
    }

    /// Finalizes outputs/inputs in parallel with storing tx metadata.
    #[allow(clippy::too_many_arguments)]
    pub fn finalize_and_store_metadata(
        &mut self,
        txs: Vec<ComputedTx>,
        txouts: Vec<ProcessedOutput>,
        txins: Vec<(TxInIndex, InputSource)>,
        same_block_spent_outpoints: &FxHashSet<OutPoint>,
        already_added: &mut ByAddrType<FxHashMap<AddrHash, TypeIndex>>,
        same_block_info: &mut FxHashMap<OutPoint, SameBlockOutputInfo>,
    ) -> Result<()> {
        let indexes = &mut *self.indexes;

        // Split transactions vecs: finalize needs first_txout_index/first_txin_index, metadata needs the rest
        let (first_txout_index, first_txin_index, mut tx_metadata) =
            self.vecs.transactions.split_for_finalize();

        let outputs = &mut self.vecs.outputs;
        let inputs = &mut self.vecs.inputs;
        let addrs = &mut self.vecs.addrs;
        let scripts = &mut self.vecs.scripts;

        let addr_hash_stores = &mut self.stores.addr_type_to_addr_hash_to_addr_index;
        let addr_tx_index_stores = &mut self.stores.addr_type_to_addr_index_and_tx_index;
        let addr_outpoint_stores = &mut self.stores.addr_type_to_addr_index_and_unspent_outpoint;
        let txid_prefix_store = &mut self.stores.txid_prefix_to_tx_index;

        let (finalize_result, metadata_result) = rayon::join(
            || -> Result<()> {
                txout::finalize_outputs(
                    indexes,
                    first_txout_index,
                    outputs,
                    addrs,
                    scripts,
                    addr_hash_stores,
                    addr_tx_index_stores,
                    addr_outpoint_stores,
                    txouts,
                    same_block_spent_outpoints,
                    already_added,
                    same_block_info,
                )?;
                txin::finalize_inputs(
                    first_txin_index,
                    inputs,
                    addr_tx_index_stores,
                    addr_outpoint_stores,
                    txins,
                    same_block_info,
                )
            },
            || tx::store_tx_metadata(txs, txid_prefix_store, &mut tx_metadata),
        );

        finalize_result?;
        metadata_result?;
        Ok(())
    }
}
