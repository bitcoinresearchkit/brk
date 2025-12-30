use brk_error::{Error, Result};
use brk_types::{BlockHashPrefix, Timestamp};
use log::error;
use vecdb::GenericStoredVec;

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
            .insert_if_needed(blockhash_prefix, height, height);

        self.stores.height_to_coinbase_tag.insert_if_needed(
            height,
            self.block.coinbase_tag().into(),
            height,
        );

        self.vecs
            .block
            .height_to_blockhash
            .checked_push(height, blockhash.clone())?;
        self.vecs
            .block
            .height_to_difficulty
            .checked_push(height, self.block.header.difficulty_float().into())?;
        self.vecs
            .block
            .height_to_timestamp
            .checked_push(height, Timestamp::from(self.block.header.time))?;
        self.vecs
            .block
            .height_to_total_size
            .checked_push(height, self.block.total_size().into())?;
        self.vecs
            .block
            .height_to_weight
            .checked_push(height, self.block.weight().into())?;

        Ok(())
    }
}
