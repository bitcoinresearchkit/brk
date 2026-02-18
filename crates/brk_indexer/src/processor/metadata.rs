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
            .blockhashprefix_to_height
            .get(&blockhash_prefix)?
            .is_some_and(|prev_height| *prev_height != height)
        {
            error!("BlockHash: {blockhash}");
            return Err(Error::Internal("BlockHash prefix collision"));
        }

        self.indexes.checked_push(self.vecs)?;

        self.stores
            .blockhashprefix_to_height
            .insert(blockhash_prefix, height);

        self.stores.height_to_coinbase_tag.insert(
            height,
            self.block.coinbase_tag().into(),
        );

        self.vecs
            .blocks
            .blockhash
            .checked_push(height, blockhash.clone())?;
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
        let overhead =
            bitcoin::block::Header::SIZE + bitcoin::VarInt::from(txs.len()).size();
        let mut total_size = overhead;
        let mut weight_wu = overhead * 4;
        for ct in txs {
            let base = ct.base_size as usize;
            let total = ct.total_size as usize;
            total_size += total;
            weight_wu += base * 3 + total;
        }
        self.vecs
            .blocks
            .total_size
            .checked_push(self.height, total_size.into())?;
        self.vecs
            .blocks
            .weight
            .checked_push(self.height, weight_wu.into())?;
        Ok(())
    }
}
