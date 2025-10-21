use brk_error::Result;
use brk_reader::Reader;
use brk_rpc::Client;
use brk_structs::Height;

use crate::{BlockIterator, BlockRange, Source};

pub struct BlockIteratorBuilder {
    range: BlockRange,
}

impl BlockIteratorBuilder {
    pub fn new(range: BlockRange) -> Self {
        Self { range }
    }

    /// Build with automatic source selection (≤10 blocks = RPC, >10 = Reader)
    pub fn smart(self, reader: &Reader, client: Client) -> Result<BlockIterator> {
        let (start, end) = self.resolve_range(&client)?;
        let count = end.saturating_sub(*start) + 1;

        let source = if count <= 10 {
            Source::new_rpc(client, start, end)
        } else {
            Source::Reader {
                receiver: reader.read(Some(start), Some(end)),
            }
        };

        Ok(BlockIterator { source })
    }

    /// Build using RPC source
    pub fn rpc(self, client: Client) -> Result<BlockIterator> {
        let (start, end) = self.resolve_range(&client)?;
        Ok(BlockIterator::from(Source::new_rpc(client, start, end)))
    }

    /// Build using Reader source
    pub fn reader(self, reader: &crate::Reader, client: Client) -> Result<BlockIterator> {
        let (start, end) = self.resolve_range(&client)?;
        Ok(BlockIterator::from(Source::Reader {
            receiver: reader.read(Some(start), Some(end)),
        }))
    }

    /// Resolve the range to concrete start/end heights
    fn resolve_range(&self, client: &Client) -> Result<(Height, Height)> {
        match self.range {
            BlockRange::Span { start, end } => Ok((start, end)),
            BlockRange::Start { start } => {
                let end = Height::new(client.get_block_count()? as u32);
                Ok((start, end))
            }
            BlockRange::End { end } => Ok((Height::ZERO, end)),
            BlockRange::Last { n } => {
                let end = Height::new(client.get_block_count()? as u32);
                let start = Height::new((*end).saturating_sub(n - 1));
                Ok((start, end))
            }
        }
    }
}

impl From<BlockRange> for BlockIteratorBuilder {
    fn from(range: BlockRange) -> Self {
        Self { range }
    }
}
