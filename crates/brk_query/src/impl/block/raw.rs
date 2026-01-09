use brk_error::{Error, Result};
use brk_types::{BlockHash, Height};
use vecdb::{AnyVec, GenericStoredVec};

use crate::Query;

impl Query {
    pub fn block_raw(&self, hash: &BlockHash) -> Result<Vec<u8>> {
        let height = self.height_by_hash(hash)?;
        self.block_raw_by_height(height)
    }

    fn block_raw_by_height(&self, height: Height) -> Result<Vec<u8>> {
        let indexer = self.indexer();
        let computer = self.computer();
        let reader = self.reader();

        let max_height = Height::from(indexer.vecs.blocks.blockhash.len().saturating_sub(1));
        if height > max_height {
            return Err(Error::OutOfRange("Block height out of range".into()));
        }

        let position = computer.positions.block_position.read_once(height)?;
        let size = indexer.vecs.blocks.total_size.read_once(height)?;

        reader.read_raw_bytes(position, *size as usize)
    }
}
