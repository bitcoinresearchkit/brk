//! In-order emission buffer + chain-continuity check used by the
//! forward pipeline. Parsers complete blocks out of order, so this
//! parks ahead-of-line matches in `pending` until `next_offset`
//! catches up.

use std::cmp::Ordering;

use brk_error::{Error, Result};
use brk_types::{BlockHash, ReadBlock};
use crossbeam::channel::Sender;
use rustc_hash::FxHashMap;
use tracing::warn;

/// Accessed by the parser pool under a `parking_lot::Mutex` owned by
/// `pipeline_forward`; at `parser_threads = 1` the lock is always
/// uncontended.
pub(super) struct ReorderState {
    pub(super) next_offset: u32,
    pending: FxHashMap<u32, ReadBlock>,
    send_to_consumer: Sender<Result<ReadBlock>>,
    /// Seeded with the user-supplied anchor so the first emit is
    /// also verified against it.
    last_emitted_hash: Option<BlockHash>,
    /// A `prev_blockhash` mismatch fires this; converted into a
    /// final `Err` by `finalize`.
    chain_broken: bool,
    /// Distinguishes "consumer cancelled" from "ran out of work
    /// early" in the missing-blocks check inside `finalize`.
    consumer_dropped: bool,
}

impl ReorderState {
    pub(super) fn new(send_to_consumer: Sender<Result<ReadBlock>>, anchor: Option<BlockHash>) -> Self {
        Self {
            next_offset: 0,
            pending: FxHashMap::default(),
            send_to_consumer,
            last_emitted_hash: anchor,
            chain_broken: false,
            consumer_dropped: false,
        }
    }

    /// Resolves the pipeline's exit state. Called by
    /// `pipeline_forward` after the read loop has finished and all
    /// parser threads have joined.
    pub(super) fn finalize(self, expected_count: usize) -> Result<()> {
        if self.chain_broken {
            return Err(Error::Internal(
                "forward pipeline: canonical batch stitched across a reorg",
            ));
        }
        if !self.consumer_dropped && (self.next_offset as usize) < expected_count {
            return Err(Error::Internal(
                "forward pipeline: blk files missing canonical blocks",
            ));
        }
        Ok(())
    }

    /// Emits `block` if it's the next expected offset (and drains
    /// any contiguous pending matches), otherwise parks it. Returns
    /// `false` once the pipeline should stop (consumer drop or chain
    /// break).
    pub(super) fn try_emit(&mut self, offset: u32, block: ReadBlock) -> bool {
        match offset.cmp(&self.next_offset) {
            Ordering::Equal => {
                if !self.send_in_order(block) {
                    return false;
                }
                while let Some(next) = self.pending.remove(&self.next_offset) {
                    if !self.send_in_order(next) {
                        return false;
                    }
                }
                true
            }
            Ordering::Greater => {
                self.pending.insert(offset, block);
                true
            }
            // Each canonical hash appears at exactly one offset and
            // each block is parsed once, so this is unreachable in
            // practice.
            Ordering::Less => true,
        }
    }

    fn send_in_order(&mut self, block: ReadBlock) -> bool {
        if let Some(last) = &self.last_emitted_hash {
            let prev = BlockHash::from(block.header.prev_blockhash);
            if prev != *last {
                warn!(
                    "canonical chain broken at offset {}: expected prev={} got {}",
                    self.next_offset, last, prev,
                );
                self.chain_broken = true;
                return false;
            }
        }
        let hash = block.hash().clone();
        if self.send_to_consumer.send(Ok(block)).is_err() {
            self.consumer_dropped = true;
            return false;
        }
        self.last_emitted_hash = Some(hash);
        self.next_offset += 1;
        true
    }
}
