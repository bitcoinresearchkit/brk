use bitview_plugin::Plugin;
use brk_error::{Error, Result};
use brk_types::{BlockHash, BlockHashPrefix, Height};
use vecdb::ReadableVec;

use crate::Query;

pub mod info;
pub mod range;
pub mod range_v1;
pub mod raw;
pub mod status;
pub mod timestamp;
pub mod txs;

pub use range::ResolvedBlocks;
pub use range_v1::ResolvedBlocksV1;
pub use timestamp::ResolvedBlockTimestamp;

impl Query {
    // A missing row in a temporarily shortened prefix is not yet a 404.
    // Never wait here: callers can hold a prefix pin needed by the writer.
    fn block_unavailable(&self, missing: Error) -> Error {
        if self.indexer().gate().try_read().is_none() {
            Error::StateUpdating
        } else {
            missing
        }
    }

    /// Hash to height, requiring an exact best-chain match at the safe bound.
    pub fn height_by_hash(&self, hash: &BlockHash) -> Result<Height> {
        let height = self
            .indexer()
            .stores()
            .block_height(&BlockHashPrefix::from(hash))?
            .ok_or_else(|| self.block_unavailable(Error::NotFound("Block not found".into())))?;
        self.validate_block_at_height(hash, height)?;
        Ok(height)
    }

    /// Validate between two safe-bound snapshots. Rollback lowers the bound
    /// before mutating vectors, so either snapshot rejects a concurrent change.
    pub fn validate_block_at_height(&self, hash: &BlockHash, height: Height) -> Result<()> {
        if height >= self.safe_lengths().height {
            return Err(self.block_unavailable(Error::NotFound("Block not found".into())));
        }

        // Validate one stored row without materializing the hash-history cache.
        if self
            .indexer()
            .vecs()
            .blocks
            .blockhash
            .inner
            .collect_one(height)
            != Some(*hash)
        {
            return Err(self.block_unavailable(Error::NotFound("Block not found".into())));
        }

        if height >= self.safe_lengths().height {
            return Err(self.block_unavailable(Error::NotFound("Block not found".into())));
        }

        Ok(())
    }
}
