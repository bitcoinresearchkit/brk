//! `CanonicalRange`: a pre-fetched map from canonical block hash to
//! offset-from-`start`. The reader uses this as the authoritative
//! filter for "is this block on the main chain?".
//!
//! Every canonical hash in the target height window is fetched from
//! bitcoind up front via [`get_block_hashes_range`], so the scan
//! pipeline never needs a per-block RPC call (which is what caused the
//! original silent-drop reorg bug).
//!
//! [`get_block_hashes_range`]: brk_rpc::Client::get_block_hashes_range

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{BlockHash, BlockHashPrefix, Height};
use rustc_hash::FxHashMap;

/// Every canonical block hash in a contiguous height window, resolved
/// from bitcoind once up front. `hashes[i]` is the canonical hash at
/// height `start + i`. Lookups by hash go through `by_prefix` (8-byte
/// key, same scheme as `brk_store`) and verify the full hash on hit.
pub struct CanonicalRange {
    pub start: Height,
    hashes: Vec<BlockHash>,
    by_prefix: FxHashMap<BlockHashPrefix, u32>,
}

impl CanonicalRange {
    /// Resolves canonical hashes for every height strictly after
    /// `anchor` up to `tip` inclusive. `anchor = None` starts at
    /// genesis.
    pub fn walk(client: &Client, anchor: Option<BlockHash>, tip: Height) -> Result<Self> {
        let start = match anchor {
            Some(hash) => Height::from(client.get_block_header_info(&hash)?.height + 1),
            None => Height::ZERO,
        };
        Self::between(client, start, tip)
    }

    /// Resolves canonical hashes for every height in `start..=end`.
    pub fn between(client: &Client, start: Height, end: Height) -> Result<Self> {
        if start > end {
            return Ok(Self {
                start,
                hashes: Vec::new(),
                by_prefix: FxHashMap::default(),
            });
        }

        let hashes = client.get_block_hashes_range(*start, *end)?;
        let mut by_prefix =
            FxHashMap::with_capacity_and_hasher(hashes.len(), Default::default());
        by_prefix.extend(
            hashes
                .iter()
                .enumerate()
                .map(|(i, h)| (BlockHashPrefix::from(h), i as u32)),
        );

        Ok(Self {
            start,
            hashes,
            by_prefix,
        })
    }

    pub fn len(&self) -> usize {
        self.hashes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }

    /// Returns the offset-from-`start` of `hash` iff it matches the
    /// canonical chain in this range. A prefix hit is verified against
    /// the full hash so prefix collisions from orphaned blocks are
    /// rejected.
    #[inline]
    pub(crate) fn offset_of(&self, hash: &BlockHash) -> Option<u32> {
        let offset = *self.by_prefix.get(&BlockHashPrefix::from(hash))?;
        (self.hashes[offset as usize] == *hash).then_some(offset)
    }
}
