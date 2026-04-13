//! The actual pipeline turning a blk-file scan into an ordered
//! `ReadBlock` stream. [`spawn`] picks between two strategies:
//!
//! * **[`pipeline_forward`]** — one reader thread walks blk files in
//!   order, peeks each block's header against the pre-fetched
//!   `CanonicalRange`, and ships canonical hits over an mpmc channel
//!   to a scoped parser pool of `parser_threads` workers, which decode
//!   bodies in parallel and serialise emission through a shared
//!   [`ReorderState`] mutex. Used when the range is larger than
//!   `TAIL_THRESHOLD`.
//! * **[`pipeline_tail`]** — single-threaded reverse scan of the
//!   newest blk files in 5 MB chunks, buffering every canonical match
//!   in offset-indexed slots and then emitting through [`ReorderState`]
//!   in the same order. Used for `canonical.len() <= TAIL_THRESHOLD`,
//!   where the channel + lock overhead of the forward pipeline would
//!   dominate.
//!
//! Both pipelines route emission through [`ReorderState`], which
//! verifies `block.header.prev_blockhash` against the previously
//! emitted block's hash and aborts cleanly if the canonical-hash batch
//! that produced the stream was stitched across a mid-batch reorg.
//!
//! Canonical blocks can also arrive out of order across blk files
//! (bitcoind doesn't write in strict chain order during initial sync,
//! headers-first body fetch, or reindex), so the reorder buffer is
//! required even at `parser_threads = 1`.

use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    ops::ControlFlow,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use brk_error::{Error, Result};
use brk_types::{BlkMetadata, BlockHash, Height, ReadBlock};
use crossbeam::channel::{Receiver, Sender, bounded};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use tracing::{error, warn};

use crate::{
    BlkIndexToBlkPath, ReaderInner, XORBytes, XORIndex,
    canonical::CanonicalRange,
    parse::{parse_canonical_body, peek_canonical_offset},
    scan::scan_bytes,
};

const CHANNEL_CAPACITY: usize = 50;
const TAIL_CHUNK: usize = 5 * 1024 * 1024;
/// Up to this many canonical blocks → tail pipeline. Beyond → forward.
const TAIL_THRESHOLD: usize = 10;

/// Default parser-thread count for [`ReaderInner::after`]. The indexer
/// is CPU-bound on the consumer side, so 1 parser + 1 reader (= 2
/// threads total) leaves the rest of the cores for the indexer. Bench
/// tools that drain the channel cheaply can override via
/// [`ReaderInner::after_with`].
pub(crate) const DEFAULT_PARSER_THREADS: usize = 1;

// ─────────────────────────────────────────────────────────────────────────────
// Shared pipeline entry — called by `Reader::after_with` and `Reader::range_with`.
// ─────────────────────────────────────────────────────────────────────────────

/// Spawns the reader worker and (for non-tail ranges) a scoped parser
/// pool, and returns the consumer receiver. Shared backend for
/// `after_with` and `range_with`.
pub(crate) fn spawn(
    reader: &ReaderInner,
    canonical: CanonicalRange,
    parser_threads: usize,
) -> Result<Receiver<ReadBlock>> {
    let parser_threads = parser_threads.max(1);

    if canonical.is_empty() {
        return Ok(bounded(0).1);
    }

    let paths = BlkIndexToBlkPath::scan(reader.blocks_dir());
    *reader.blk_index_to_blk_path.write() = paths.clone();

    let (send, recv) = bounded(CHANNEL_CAPACITY);
    let xor_bytes = reader.xor_bytes();
    let use_tail = canonical.len() <= TAIL_THRESHOLD;
    let first_blk_index = if use_tail {
        0
    } else {
        reader
            .find_start_blk_index(Some(canonical.start), &paths, xor_bytes)
            .unwrap_or_default()
    };

    thread::spawn(move || {
        let result = if use_tail {
            pipeline_tail(&paths, xor_bytes, &canonical, &send)
        } else {
            pipeline_forward(
                &paths,
                first_blk_index,
                xor_bytes,
                &canonical,
                &send,
                parser_threads,
            )
        };
        if let Err(e) = result {
            error!("Reader canonical pipeline failed: {e}");
        }
    });

    Ok(recv)
}

// ─────────────────────────────────────────────────────────────────────────────
// Forward pipeline — 1 reader + N parsers + shared in-order emission.
// ─────────────────────────────────────────────────────────────────────────────

/// A raw block the reader has already confirmed is on the canonical
/// chain, shipped to the parser pool for full decoding.
struct ScannedBlock {
    metadata: BlkMetadata,
    bytes: Vec<u8>,
    xor_state: XORIndex,
    canonical_offset: u32,
}

/// In-order emission buffer shared between the parser threads. Access
/// is serialised through a `parking_lot::Mutex`; at `parser_threads = 1`
/// the lock is always uncontended.
///
/// Also enforces **chain continuity**: before emitting each block it
/// checks that `block.header.prev_blockhash` matches the previously-
/// emitted block's hash. A mismatch means the canonical-hash batch
/// that produced this stream was stitched across a mid-batch reorg,
/// so we stop emitting cleanly and let the caller retry.
struct ReorderState {
    next_offset: u32,
    /// Ahead-of-line matches keyed by canonical offset; drained
    /// contiguously each time `next_offset` advances. Bounded in
    /// practice by parser-thread scheduling jitter.
    pending: FxHashMap<u32, ReadBlock>,
    send_to_consumer: Sender<ReadBlock>,
    /// Hash of the last block successfully emitted, used to verify
    /// continuity with the next one. `None` before the first emit.
    last_emitted_hash: Option<BlockHash>,
    /// Flipped when a continuity check fails.
    chain_broken: bool,
}

impl ReorderState {
    fn new(send_to_consumer: Sender<ReadBlock>) -> Self {
        Self {
            next_offset: 0,
            pending: FxHashMap::default(),
            send_to_consumer,
            last_emitted_hash: None,
            chain_broken: false,
        }
    }

    /// Accepts a parsed canonical block; emits it and drains any
    /// contiguous pending matches. Returns `false` once the pipeline
    /// should stop — either the consumer dropped the receiver or a
    /// chain-continuity check failed. Completion (all blocks emitted)
    /// is checked by the caller via `next_offset`.
    fn try_emit(&mut self, offset: u32, block: ReadBlock) -> bool {
        use std::cmp::Ordering::*;
        match offset.cmp(&self.next_offset) {
            Equal => {
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
            Greater => {
                self.pending.insert(offset, block);
                true
            }
            // Unreachable in practice: each canonical hash appears at
            // exactly one offset and each block is parsed once.
            Less => true,
        }
    }

    /// Verifies `block.prev_blockhash` against the last emitted hash,
    /// sends the block, and bumps `next_offset`. Returns `false` on
    /// continuity failure or consumer drop.
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
        if self.send_to_consumer.send(block).is_err() {
            return false;
        }
        self.last_emitted_hash = Some(hash);
        self.next_offset += 1;
        true
    }
}

fn pipeline_forward(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    send: &Sender<ReadBlock>,
    parser_threads: usize,
) -> Result<()> {
    let (parser_send, parser_recv) = bounded::<ScannedBlock>(CHANNEL_CAPACITY);
    let reorder = Mutex::new(ReorderState::new(send.clone()));
    let done = AtomicBool::new(false);

    thread::scope(|scope| -> Result<()> {
        for _ in 0..parser_threads {
            let parser_recv = parser_recv.clone();
            scope.spawn(|| parser_loop(parser_recv, &reorder, &done, canonical, xor_bytes));
        }
        // Every parser owns its own clone; ours would otherwise keep
        // the channel "alive" and leak a dangling receiver.
        drop(parser_recv);

        let read_result = read_and_dispatch(
            paths,
            first_blk_index,
            xor_bytes,
            canonical,
            &parser_send,
            &done,
        );
        // Signal end-of-input to the parsers so they exit their `for`
        // loops and the scope can join them.
        drop(parser_send);
        read_result
    })?;

    let state = reorder.lock();
    if state.chain_broken {
        return Err(Error::Internal(
            "forward pipeline: canonical batch stitched across a reorg",
        ));
    }
    let pipeline_cancelled = done.load(Ordering::Relaxed);
    let emitted = state.next_offset as usize;
    if !pipeline_cancelled && emitted < canonical.len() {
        return Err(Error::Internal(
            "forward pipeline: blk files missing canonical blocks",
        ));
    }
    Ok(())
}

/// Full-body parse + in-order emit loop run by every scoped parser
/// worker in `pipeline_forward`. Drains `parser_recv` to exhaustion.
fn parser_loop(
    parser_recv: Receiver<ScannedBlock>,
    reorder: &Mutex<ReorderState>,
    done: &AtomicBool,
    canonical: &CanonicalRange,
    xor_bytes: XORBytes,
) {
    for ScannedBlock { metadata, bytes, xor_state, canonical_offset } in parser_recv {
        if done.load(Ordering::Relaxed) {
            continue;
        }
        let height = Height::from(*canonical.start + canonical_offset);
        let block = match parse_canonical_body(bytes, metadata, xor_state, xor_bytes, height) {
            Ok(block) => block,
            Err(e) => {
                warn!("parse_canonical_body failed: {e}");
                continue;
            }
        };
        let pipeline_finished = {
            let mut state = reorder.lock();
            !state.try_emit(canonical_offset, block)
                || state.next_offset as usize >= canonical.len()
        };
        if pipeline_finished {
            done.store(true, Ordering::Relaxed);
        }
    }
}

/// Walk blk files from `first_blk_index`, scan each one, and ship
/// canonical blocks to the parser pool. Non-canonical blocks are
/// rejected via `peek_canonical_offset` *before* being cloned — the
/// cheap filter is what lets a sparse catchup avoid allocating for the
/// ~99% of blocks outside the window.
fn read_and_dispatch(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    parser_send: &Sender<ScannedBlock>,
    done: &AtomicBool,
) -> Result<()> {
    for (&blk_index, blk_path) in paths.range(first_blk_index..) {
        if done.load(Ordering::Relaxed) {
            return Ok(());
        }

        let mut bytes = fs::read(blk_path).map_err(|e| {
            error!("Failed to read blk file {}: {e}", blk_path.display());
            Error::Internal("Failed to read blk file")
        })?;

        let result = scan_bytes(
            &mut bytes,
            blk_index,
            0,
            xor_bytes,
            |metadata, block_bytes, xor_state| {
                if done.load(Ordering::Relaxed) {
                    return ControlFlow::Break(());
                }
                let Some(canonical_offset) =
                    peek_canonical_offset(block_bytes, xor_state, xor_bytes, canonical)
                else {
                    return ControlFlow::Continue(());
                };
                let scanned = ScannedBlock {
                    metadata,
                    bytes: block_bytes.to_vec(),
                    xor_state,
                    canonical_offset,
                };
                if parser_send.send(scanned).is_err() {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            },
        );

        if result.interrupted {
            return Ok(());
        }
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Tail pipeline — reverse-scan the newest blk files in 5 MB chunks until
// every canonical hash has been matched, then emit them forward.
// ─────────────────────────────────────────────────────────────────────────────

/// Single-threaded tail-range pipeline for small `canonical.len()`.
/// Walks blk files in reverse-index order, reads each one in 5 MB
/// chunks from tail to head, and stuffs every canonical match into an
/// offset-indexed `slots` vec. Once every canonical block is matched,
/// emits them in order through [`ReorderState`] (which doubles as the
/// shared continuity checker). Bails on missing blocks or a chain
/// break just like [`pipeline_forward`].
fn pipeline_tail(
    paths: &BlkIndexToBlkPath,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    send: &Sender<ReadBlock>,
) -> Result<()> {
    let mut slots: Vec<Option<ReadBlock>> = (0..canonical.len()).map(|_| None).collect();
    let mut remaining = canonical.len();
    // Carries the bytes before a chunk's first magic into the next
    // (earlier) chunk so blocks straddling the boundary survive.
    let mut spillover: Vec<u8> = Vec::new();

    'files: for (&blk_index, path) in paths.iter().rev() {
        let mut file = File::open(path).map_err(|_| Error::Internal("Failed to open blk file"))?;
        let file_len = file.metadata().map(|m| m.len() as usize).unwrap_or(0);
        if file_len == 0 {
            continue;
        }

        let mut read_end = file_len;
        spillover.clear();

        while read_end > 0 && remaining > 0 {
            let read_start = read_end.saturating_sub(TAIL_CHUNK);
            let chunk_len = read_end - read_start;
            read_end = read_start;

            file.seek(SeekFrom::Start(read_start as u64))
                .map_err(|_| Error::Internal("Failed to seek blk file"))?;
            let mut buf = vec![0u8; chunk_len + spillover.len()];
            file.read_exact(&mut buf[..chunk_len])
                .map_err(|_| Error::Internal("Failed to read blk chunk"))?;
            buf[chunk_len..].copy_from_slice(&spillover);
            spillover.clear();

            let result = scan_bytes(
                &mut buf,
                blk_index,
                read_start,
                xor_bytes,
                |metadata, block_bytes, xor_state| {
                    let Some(offset) =
                        peek_canonical_offset(block_bytes, xor_state, xor_bytes, canonical)
                    else {
                        return ControlFlow::Continue(());
                    };
                    if slots[offset as usize].is_some() {
                        return ControlFlow::Continue(());
                    }
                    let height = Height::from(*canonical.start + offset);
                    match parse_canonical_body(
                        block_bytes.to_vec(),
                        metadata,
                        xor_state,
                        xor_bytes,
                        height,
                    ) {
                        Ok(block) => {
                            slots[offset as usize] = Some(block);
                            remaining -= 1;
                        }
                        Err(e) => warn!("parse_canonical_body failed in tail pipeline: {e}"),
                    }
                    if remaining == 0 {
                        ControlFlow::Break(())
                    } else {
                        ControlFlow::Continue(())
                    }
                },
            );

            if remaining == 0 {
                break 'files;
            }
            if read_start > 0 {
                spillover.extend_from_slice(&buf[..result.first_magic.unwrap_or(buf.len())]);
            }
        }
    }

    if remaining > 0 {
        return Err(Error::Internal(
            "tail pipeline: blk files missing canonical blocks",
        ));
    }

    // Emit in canonical order via the same `ReorderState` the forward
    // pipeline uses, which verifies `prev_blockhash` continuity between
    // adjacent blocks as a side effect of `try_emit`.
    let mut reorder = ReorderState::new(send.clone());
    for (offset, block) in slots.into_iter().flatten().enumerate() {
        if !reorder.try_emit(offset as u32, block) {
            break;
        }
    }
    if reorder.chain_broken {
        return Err(Error::Internal(
            "tail pipeline: canonical batch stitched across a reorg",
        ));
    }
    Ok(())
}
