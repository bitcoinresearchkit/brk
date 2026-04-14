//! Two-strategy block-streaming pipeline. [`spawn`] picks between:
//!
//! * **forward** — one reader thread walks blk files in order from a
//!   bisection lower bound; canonical hits ship to a parser pool that
//!   emits in-order through [`reorder::ReorderState`].
//! * **tail** — single-threaded reverse scan of the newest blk files,
//!   buffering matches in offset slots, then emitting forward with
//!   an inline chain check.
//!
//! Both strategies verify `block.header.prev_blockhash` against the
//! previously emitted block — and against the user-supplied `anchor`
//! for the very first block — and propagate a final `Err` to the
//! consumer on chain breaks, parse failures, or missing blocks.

use std::{sync::Arc, thread};

use brk_error::Result;
use brk_rpc::Client;
use brk_types::{BlockHash, Height, ReadBlock};
use crossbeam::channel::{Receiver, bounded};

use crate::{
    BlkIndexToBlkPath, ReaderInner, XORBytes, bisect,
    canonical::CanonicalRange,
};

mod forward;
mod reorder;
mod tail;

pub(crate) const CHANNEL_CAPACITY: usize = 50;

/// If `canonical.start` lives within this many files of the chain
/// tip, use the reverse-scan pipeline. The forward pipeline pays the
/// bisection + 21-file backoff (~2.7 GB of reads) regardless of how
/// few canonical blocks live in the window, so for any tip-clustered
/// catchup the tail wins until the window grows past this many files.
const TAIL_DISTANCE_FILES: usize = 8;

/// The indexer is CPU-bound on the consumer side, so 1 reader + 1
/// parser leaves the rest of the cores for it. Bench tools that
/// drain the channel cheaply can override.
pub(crate) const DEFAULT_PARSER_THREADS: usize = 1;

enum Strategy {
    Tail,
    Forward { first_blk_index: u16 },
}

/// `anchor`, when supplied, is the hash the consumer expects to be
/// the **parent** of the first emitted block. Seeded into the chain
/// check so a stale `Reader::after` anchor (e.g. the tip of a
/// reorged-out chain) cannot silently produce a stitched stream.
/// `None` skips the check (genesis or `range`-style calls have no
/// anchor to verify against).
pub(crate) fn spawn(
    reader: Arc<ReaderInner>,
    canonical: CanonicalRange,
    anchor: Option<BlockHash>,
    parser_threads: usize,
) -> Result<Receiver<Result<ReadBlock>>> {
    // Cap at the parser channel capacity: beyond that, extra parsers
    // are idle (they all contend for the same buffered items) and
    // absurd inputs would otherwise OOM the scoped spawn.
    let parser_threads = parser_threads.clamp(1, CHANNEL_CAPACITY);

    if canonical.is_empty() {
        return Ok(bounded(0).1);
    }

    let paths = reader.refresh_paths()?;
    let xor_bytes = reader.xor_bytes;
    let strategy = pick_strategy(&reader.client, &paths, xor_bytes, canonical.start);

    let (send, recv) = bounded(CHANNEL_CAPACITY);

    thread::spawn(move || {
        let result = match strategy {
            Strategy::Tail => {
                tail::pipeline_tail(&reader.client, &paths, xor_bytes, &canonical, anchor, &send)
            }
            Strategy::Forward { first_blk_index } => forward::pipeline_forward(
                &paths,
                first_blk_index,
                xor_bytes,
                &canonical,
                anchor,
                &send,
                parser_threads,
            ),
        };
        if let Err(e) = result {
            // No-op if the consumer already dropped the receiver.
            let _ = send.send(Err(e));
        }
    });

    Ok(recv)
}

/// Tail iff one of the last `TAIL_DISTANCE_FILES` files starts at a
/// height ≤ `canonical_start`; that file is where tail iteration
/// would land. Otherwise bisect for the forward start. Genesis-rooted
/// catchups skip the tail probes since no file's first block is ≤
/// genesis.
fn pick_strategy(
    client: &Client,
    paths: &BlkIndexToBlkPath,
    xor_bytes: XORBytes,
    canonical_start: Height,
) -> Strategy {
    if canonical_start != Height::ZERO
        && paths
            .iter()
            .rev()
            .take(TAIL_DISTANCE_FILES)
            .any(|(_, path)| {
                bisect::first_block_height(client, path, xor_bytes)
                    .is_ok_and(|h| h <= canonical_start)
            })
    {
        return Strategy::Tail;
    }
    Strategy::Forward {
        first_blk_index: bisect::find_start_blk_index(client, canonical_start, paths, xor_bytes),
    }
}
