use brk_error::{Error, Result};
use brk_types::{BlockHash, BlockHashPrefix, BlockInfo, Height, TxIndex};
use vecdb::{AnyVec, GenericStoredVec, VecIndex};

use crate::Query;

const DEFAULT_BLOCK_COUNT: u32 = 10;

impl Query {
    pub fn block(&self, hash: &BlockHash) -> Result<BlockInfo> {
        let height = self.height_by_hash(hash)?;
        self.block_by_height(height)
    }

    pub fn block_by_height(&self, height: Height) -> Result<BlockInfo> {
        let indexer = self.indexer();

        let max_height = self.max_height();
        if height > max_height {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }

        let blockhash = indexer.vecs.block.height_to_blockhash.read_once(height)?;
        let difficulty = indexer.vecs.block.height_to_difficulty.read_once(height)?;
        let timestamp = indexer.vecs.block.height_to_timestamp.read_once(height)?;
        let size = indexer.vecs.block.height_to_total_size.read_once(height)?;
        let weight = indexer.vecs.block.height_to_weight.read_once(height)?;
        let tx_count = self.tx_count_at_height(height, max_height)?;

        Ok(BlockInfo {
            id: blockhash,
            height,
            tx_count,
            size: *size,
            weight,
            timestamp,
            difficulty: *difficulty,
        })
    }

    pub fn blocks(&self, start_height: Option<Height>) -> Result<Vec<BlockInfo>> {
        let max_height = self.height();

        let start = start_height.unwrap_or(max_height);
        let start = start.min(max_height);

        let start_u32: u32 = start.into();
        let count = DEFAULT_BLOCK_COUNT.min(start_u32 + 1);

        let mut blocks = Vec::with_capacity(count as usize);
        for i in 0..count {
            let height = Height::from(start_u32 - i);
            blocks.push(self.block_by_height(height)?);
        }

        Ok(blocks)
    }

    // === Helper methods ===

    pub fn height_by_hash(&self, hash: &BlockHash) -> Result<Height> {
        let indexer = self.indexer();

        let prefix = BlockHashPrefix::from(hash);

        indexer
            .stores
            .blockhashprefix_to_height
            .get(&prefix)?
            .map(|h| *h)
            .ok_or(Error::NotFound("Block not found".into()))
    }

    fn max_height(&self) -> Height {
        Height::from(
            self.indexer()
                .vecs
                .block
                .height_to_blockhash
                .len()
                .saturating_sub(1),
        )
    }

    fn tx_count_at_height(&self, height: Height, max_height: Height) -> Result<u32> {
        let indexer = self.indexer();
        let computer = self.computer();

        let first_txindex = indexer.vecs.tx.height_to_first_txindex.read_once(height)?;
        let next_first_txindex = if height < max_height {
            indexer
                .vecs
                .tx
                .height_to_first_txindex
                .read_once(height.incremented())?
        } else {
            TxIndex::from(computer.indexes.txindex_to_txindex.len())
        };

        Ok((next_first_txindex.to_usize() - first_txindex.to_usize()) as u32)
    }
}
