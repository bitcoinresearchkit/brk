use std::sync::Arc;

use brk_error::Result;
use brk_reader::Reader;
use brk_rpc::Client;
use brk_types::{BlockHash, Height};

mod iterator;
mod range;
mod source;
mod state;

use iterator::*;
use range::*;
use source::*;
use state::*;

///
/// Block iterator factory
///
/// Creates iterators over Bitcoin blocks from various sources (RPC/Reader).
/// Iterators may end earlier than expected if a chain reorganization occurs.
///
/// Thread-safe and free to clone.
///
#[derive(Clone)]
pub struct Blocks(Arc<Source>);

impl Blocks {
    /// Create with smart mode (auto-select source based on range size)
    pub fn new(client: Client, reader: Reader) -> Self {
        Self::new_inner(Source::Smart { client, reader })
    }

    /// Create with RPC-only mode
    pub fn new_rpc(client: Client) -> Self {
        Self::new_inner(Source::Rpc { client })
    }

    /// Create with Reader-only mode
    pub fn new_reader(reader: Reader) -> Self {
        Self::new_inner(Source::Reader { reader })
    }

    fn new_inner(source: Source) -> Self {
        Self(Arc::new(source))
    }

    /// Iterate over a specific range (start..=end)
    pub fn range(&self, start: Height, end: Height) -> Result<BlockIterator> {
        self.iter(BlockRange::Span { start, end })
    }

    /// Iterate from start (inclusive) to chain tip
    pub fn start(&self, start: Height) -> Result<BlockIterator> {
        self.iter(BlockRange::Start { start })
    }

    /// Iterate from genesis to end (inclusive)
    pub fn end(&self, end: Height) -> Result<BlockIterator> {
        self.iter(BlockRange::End { end })
    }

    /// Iterate over last n blocks
    pub fn last(&self, n: u32) -> Result<BlockIterator> {
        self.iter(BlockRange::Last { n })
    }

    /// Iterate after hash
    pub fn after(&self, hash: Option<BlockHash>) -> Result<BlockIterator> {
        self.iter(BlockRange::After { hash })
    }

    fn iter(&self, range: BlockRange) -> Result<BlockIterator> {
        let (start, end, hash_opt) = self.resolve_range(range)?;

        let count = end.saturating_sub(*start) + 1;

        let state = match &*self.0 {
            Source::Smart { client, reader } => {
                if count <= 10 {
                    State::new_rpc(client.clone(), start, end, hash_opt)
                } else {
                    State::new_reader(reader.clone(), start, end, hash_opt)
                }
            }
            Source::Rpc { client } => State::new_rpc(client.clone(), start, end, hash_opt),
            Source::Reader { reader, .. } => {
                State::new_reader(reader.clone(), start, end, hash_opt)
            }
        };

        Ok(BlockIterator::new(state))
    }

    fn resolve_range(&self, range: BlockRange) -> Result<(Height, Height, Option<BlockHash>)> {
        let client = self.0.client();

        match range {
            BlockRange::Span { start, end } => Ok((start, end, None)),
            BlockRange::Start { start } => {
                let end = client.get_last_height()?;
                Ok((start, end, None))
            }
            BlockRange::End { end } => Ok((Height::ZERO, end, None)),
            BlockRange::Last { n } => {
                let end = client.get_last_height()?;
                let start = Height::new((*end).saturating_sub(n - 1));
                Ok((start, end, None))
            }
            BlockRange::After { hash } => {
                let start = if let Some(hash) = hash.as_ref() {
                    let block_info = client.get_block_header_info(hash)?;
                    (block_info.height + 1).into()
                } else {
                    Height::ZERO
                };
                let end = client.get_last_height()?;
                Ok((start, end, hash))
            }
        }
    }
}
