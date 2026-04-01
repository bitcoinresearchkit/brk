use brk_error::{Error, Result};
use brk_types::{BlockHashPrefix, Timestamp};
use tracing::error;
use vecdb::WritableVec;

use super::{BlockProcessor, ComputedTx};
use crate::IndexesExt;

impl BlockProcessor<'_> {
    pub fn process_block_metadata(&mut self) -> Result<()> {
        let height = self.height;
        let blockhash = self.block.hash();
        let blockhash_prefix = BlockHashPrefix::from(blockhash);

        if self
            .stores
            .blockhash_prefix_to_height
            .get(&blockhash_prefix)?
            .is_some_and(|prev_height| *prev_height != height)
        {
            error!("BlockHash: {blockhash}");
            return Err(Error::Internal("BlockHash prefix collision"));
        }

        self.indexes.checked_push(self.vecs)?;

        self.stores
            .blockhash_prefix_to_height
            .insert(blockhash_prefix, height);

        self.vecs
            .blocks
            .blockhash
            .checked_push(height, blockhash.clone())?;
        self.vecs
            .blocks
            .coinbase_tag
            .checked_push(height, self.block.coinbase_tag())?;
        self.vecs
            .blocks
            .difficulty
            .checked_push(height, self.block.header.difficulty_float().into())?;
        self.vecs
            .blocks
            .timestamp
            .checked_push(height, Timestamp::from(self.block.header.time))?;

        Ok(())
    }

    /// Push block total_size and weight, reusing per-tx sizes already computed in ComputedTx.
    /// This avoids redundant tx serialization (base_size + total_size were already computed).
    pub fn push_block_size_and_weight(&mut self, txs: &[ComputedTx]) -> Result<()> {
        let overhead = bitcoin::block::Header::SIZE + bitcoin::VarInt::from(txs.len()).size();
        let mut total_size = overhead;
        let mut weight = overhead * 4;
        let mut sw_txs = 0u32;
        let mut sw_size = 0usize;
        let mut sw_weight = 0usize;

        for (i, tx) in txs.iter().enumerate() {
            total_size += tx.total_size as usize;
            weight += tx.weight();
            if i > 0 && tx.is_segwit() {
                sw_txs += 1;
                sw_size += tx.total_size as usize;
                sw_weight += tx.weight();
            }
        }

        let h = self.height;
        let blocks = &mut self.vecs.blocks;
        blocks.total.checked_push(h, total_size.into())?;
        blocks.weight.checked_push(h, weight.into())?;
        blocks.segwit_txs.checked_push(h, sw_txs.into())?;
        blocks.segwit_size.checked_push(h, sw_size.into())?;
        blocks.segwit_weight.checked_push(h, sw_weight.into())?;
        Ok(())
    }
}
