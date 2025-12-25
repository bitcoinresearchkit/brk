//! Block processing for indexing.
//!
//! This module handles the extraction and storage of all indexed data from blocks.
//! Processing is split into phases that can be parallelized where possible.

mod metadata;
mod tx;
mod txin;
mod txout;
mod types;

pub use types::*;

use brk_types::{Block, Height, TxInIndex, TxIndex, TxOutIndex};

use crate::{Indexes, RangeMap, Readers, Stores, Vecs};

/// Processes a single block, extracting and storing all indexed data.
pub struct BlockProcessor<'a> {
    pub block: &'a Block,
    pub height: Height,
    pub check_collisions: bool,
    pub indexes: &'a mut Indexes,
    pub vecs: &'a mut Vecs,
    pub stores: &'a mut Stores,
    pub readers: &'a Readers,
    pub txindex_to_height: &'a RangeMap<TxIndex, Height>,
}

impl BlockProcessor<'_> {
    /// Update global indexes after processing a block.
    pub fn update_indexes(&mut self, tx_count: usize, input_count: usize, output_count: usize) {
        self.indexes.txindex += TxIndex::from(tx_count);
        self.indexes.txinindex += TxInIndex::from(input_count);
        self.indexes.txoutindex += TxOutIndex::from(output_count);
    }
}
