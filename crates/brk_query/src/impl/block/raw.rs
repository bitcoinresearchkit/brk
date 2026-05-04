use brk_error::{Error, OptionData, Result};
use brk_types::{BlockHash, Height};
use vecdb::ReadableVec;

use crate::Query;

impl Query {
    pub fn block_raw(&self, hash: &BlockHash) -> Result<Vec<u8>> {
        let height = self.height_by_hash(hash)?;
        self.block_raw_by_height(height)
    }

    fn block_raw_by_height(&self, height: Height) -> Result<Vec<u8>> {
        let max_height = self.tip_height();
        if height > max_height {
            return Err(Error::OutOfRange(
                format!("Block height {height} out of range (tip {max_height})").into(),
            ));
        }

        let indexer = self.indexer();
        let position = indexer.vecs.blocks.position.collect_one(height).data()?;
        let size = indexer.vecs.blocks.total.collect_one(height).data()?;

        self.reader().read_raw_bytes(position, *size as usize)
    }
}
