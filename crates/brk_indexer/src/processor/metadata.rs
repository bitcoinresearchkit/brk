use brk_error::{Error, Result};
use brk_types::{BlockHashPrefix, Timestamp};
use tracing::error;
use vecdb::WritableVec;

use super::BlockProcessor;
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
        let (block_total_size, block_weight) = self.block.total_size_and_weight();
        self.vecs
            .blocks
            .total_size
            .checked_push(height, block_total_size.into())?;
        self.vecs
            .blocks
            .weight
            .checked_push(height, block_weight.into())?;

        Ok(())
    }
}
