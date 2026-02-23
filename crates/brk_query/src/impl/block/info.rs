use brk_error::{Error, Result};
use brk_types::{BlockHash, BlockHashPrefix, BlockInfo, Height, TxIndex};
use vecdb::{AnyVec, ReadableVec, VecIndex};

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

        let blockhash = indexer.vecs.blocks.blockhash.read_once(height)?;
        let difficulty = indexer.vecs.blocks.difficulty.collect_one(height).unwrap();
        let timestamp = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();
        let size = indexer.vecs.blocks.total_size.collect_one(height).unwrap();
        let weight = indexer.vecs.blocks.weight.collect_one(height).unwrap();
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
        let count = DEFAULT_BLOCK_COUNT.min(start_u32 + 1) as usize;

        if count == 0 {
            return Ok(Vec::new());
        }

        let indexer = self.indexer();
        let computer = self.computer();

        // Batch-read all PcoVec data for the contiguous range (avoids
        // per-block page decompression — 4 reads instead of 4*count).
        let end = start_u32 as usize + 1;
        let begin = end - count;

        let difficulties = indexer.vecs.blocks.difficulty.collect_range_at(begin, end);
        let timestamps = indexer.vecs.blocks.timestamp.collect_range_at(begin, end);
        let sizes = indexer.vecs.blocks.total_size.collect_range_at(begin, end);
        let weights = indexer.vecs.blocks.weight.collect_range_at(begin, end);

        // Batch-read first_txindex for tx_count computation (need one extra for next boundary)
        let txindex_end = if end <= max_height.to_usize() {
            end + 1
        } else {
            end
        };
        let first_txindexes: Vec<TxIndex> = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_range_at(begin, txindex_end);
        let total_txs = computer.indexes.txindex.identity.len();

        let mut blocks = Vec::with_capacity(count);
        for i in (0..count).rev() {
            let height = Height::from(begin + i);
            let blockhash = indexer.vecs.blocks.blockhash.read_once(height)?;

            let tx_count = if i + 1 < first_txindexes.len() {
                first_txindexes[i + 1].to_usize() - first_txindexes[i].to_usize()
            } else {
                total_txs - first_txindexes[i].to_usize()
            };

            blocks.push(BlockInfo {
                id: blockhash,
                height,
                tx_count: tx_count as u32,
                size: *sizes[i],
                weight: weights[i],
                timestamp: timestamps[i],
                difficulty: *difficulties[i],
            });
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
        Height::from(self.indexer().vecs.blocks.blockhash.len().saturating_sub(1))
    }

    fn tx_count_at_height(&self, height: Height, max_height: Height) -> Result<u32> {
        let indexer = self.indexer();
        let computer = self.computer();

        let first_txindex = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_one(height)
            .unwrap();
        let next_first_txindex = if height < max_height {
            indexer
                .vecs
                .transactions
                .first_txindex
                .collect_one(height.incremented())
                .unwrap()
        } else {
            TxIndex::from(computer.indexes.txindex.identity.len())
        };

        Ok((next_first_txindex.to_usize() - first_txindex.to_usize()) as u32)
    }
}
