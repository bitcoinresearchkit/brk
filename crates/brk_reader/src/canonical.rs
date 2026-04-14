//! `CanonicalRange`: every canonical block hash in a height window,
//! pre-fetched once via [`brk_rpc::Client::get_block_hashes_range`].
//! Used as the authoritative "is this block on the main chain?"
//! filter so the scan pipeline never needs a per-block RPC call.

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{BlockHash, Height};
use rustc_hash::FxHashMap;

/// Keyed on the full 32-byte hash because a prefix collision would
/// silently drop both blocks; the ~24 MB extra RAM is negligible
/// against the 128 MB blk reads happening in parallel.
pub struct CanonicalRange {
    pub start: Height,
    by_hash: FxHashMap<BlockHash, u32>,
}

impl CanonicalRange {
    /// Resolves canonical hashes for every height strictly after
    /// `anchor` up to `tip` inclusive. `anchor = None` starts at
    /// genesis.
    pub fn walk(client: &Client, anchor: Option<&BlockHash>, tip: Height) -> Result<Self> {
        let start = match anchor {
            Some(hash) => Height::from(client.get_block_header_info(hash)?.height + 1),
            None => Height::ZERO,
        };
        Self::between(client, start, tip)
    }

    /// Resolves canonical hashes for every height in `start..=end`.
    pub fn between(client: &Client, start: Height, end: Height) -> Result<Self> {
        if start > end {
            return Ok(Self {
                start,
                by_hash: FxHashMap::default(),
            });
        }
        let by_hash = client
            .get_block_hashes_range(*start, *end)?
            .into_iter()
            .enumerate()
            .map(|(i, h)| (h, i as u32))
            .collect();
        Ok(Self { start, by_hash })
    }

    pub fn len(&self) -> usize {
        self.by_hash.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_hash.is_empty()
    }

    /// Offset-from-`start` of `hash` iff it's on the canonical chain.
    #[inline]
    pub(crate) fn offset_of(&self, hash: &BlockHash) -> Option<u32> {
        self.by_hash.get(hash).copied()
    }
}
