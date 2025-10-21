use brk_reader::Reader;
use brk_structs::{Block, Height};

mod builder;
mod range;
mod source;

use builder::*;
use range::*;
use source::*;

/// Block iterator that can use either RPC or Reader
pub struct BlockIterator {
    source: Source,
}

impl From<Source> for BlockIterator {
    fn from(source: Source) -> Self {
        Self { source }
    }
}

impl Iterator for BlockIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.source {
            Source::Rpc {
                client,
                heights,
                prev_hash,
            } => {
                let height = heights.next()?;

                let Ok(hash) = client.get_block_hash(height) else {
                    return None;
                };

                let Ok(block) = client.get_block(&hash) else {
                    return None;
                };

                if prev_hash
                    .as_ref()
                    .is_some_and(|prev_hash| block.header.prev_blockhash != prev_hash.into())
                {
                    return None;
                }

                Some(Block::from((height, hash, block)))
            }
            Source::Reader { receiver } => receiver.recv().ok().map(|b| b.unwrap()),
        }
    }
}

impl BlockIterator {
    pub fn range(start: Height, end: Height) -> BlockIteratorBuilder {
        BlockIteratorBuilder::from(BlockRange::Span { start, end })
    }

    pub fn start(start: Height) -> BlockIteratorBuilder {
        BlockIteratorBuilder::from(BlockRange::Start { start })
    }

    pub fn end(end: Height) -> BlockIteratorBuilder {
        BlockIteratorBuilder::from(BlockRange::End { end })
    }

    pub fn last(n: u32) -> BlockIteratorBuilder {
        BlockIteratorBuilder::from(BlockRange::Last { n })
    }
}
