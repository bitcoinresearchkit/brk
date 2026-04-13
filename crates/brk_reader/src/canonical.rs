//! Canonical-hash pipeline for `Reader::after`.
//!
//! Bitcoin Core stores accepted blocks in append-only `blk*.dat` files
//! under the data dir, XOR-encoded with a per-datadir key. A "blk
//! file" contains every block the node ever accepted — including
//! blocks that were later orphaned by a reorg — in acceptance order,
//! not height order. This module turns "give me every block after
//! `hash` up to the tip" into an ordered `ReadBlock` stream drawn from
//! those files while skipping orphans.
//!
//! How it works:
//!
//! 1. [`CanonicalRange::walk`] asks bitcoind once, up front, for the
//!    canonical block hash at every height in the target window. This
//!    is one batched JSON-RPC request — no per-block RPC overhead.
//! 2. The reader walks blk files in order and scans each one for the
//!    block magic prefix. For every block found,
//!    [`peek_canonical_offset`] hashes the 80-byte header and looks
//!    the hash up in the canonical map. Orphans short-circuit here,
//!    before any bytes are cloned.
//! 3. Canonical hits are cloned into [`ScannedBlock`]s and shipped
//!    over a channel to a small pool of parser workers, which run
//!    [`parse_canonical_body`] to fully decode the block.
//! 4. Parsers serialise their output through [`ReorderState`] so that
//!    the consumer receives blocks in canonical-height order even if
//!    the blk files emitted them out of order.
//!
//! Ranges of at most `TAIL_THRESHOLD` blocks take a specialised
//! [`pipeline_tail`] path that reverse-scans the newest blk files in
//! 5 MB chunks — cheaper than walking forward from genesis for a
//! handful of tip blocks.
//!
//! Public entry points: [`ReaderInner::after_canonical`] and
//! [`ReaderInner::after_canonical_with`]. Coexists with the original
//! `read` / `read_rev` / `after` so the two can be A/B-tested.

use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek, SeekFrom},
    ops::ControlFlow,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use bitcoin::{Transaction, VarInt, block::Header, consensus::Decodable};
use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::{BlkMetadata, Block, BlockHash, BlockHashPrefix, Height, ReadBlock};
use crossbeam::channel::{Receiver, Sender, bounded};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use tracing::{error, warn};

use crate::{BlkIndexToBlkPath, ReaderInner, XORBytes, XORIndex, scan::scan_bytes};

const BOUND_CAP: usize = 50;
const TAIL_CHUNK: usize = 5 * 1024 * 1024;
/// Up to this many canonical blocks → tail pipeline. Beyond → forward.
const TAIL_THRESHOLD: usize = 10;
/// Default parser-thread count for `after_canonical`. The indexer is
/// CPU-bound on the consumer side, so 1 parser thread + 1 reader thread
/// (= 2 total) leaves the rest of the cores for the indexer. Bench tools
/// that drain the channel cheaply can override via `after_canonical_with`.
const DEFAULT_PARSER_THREADS: usize = 1;

// ─────────────────────────────────────────────────────────────────────────────
// CanonicalRange — the only RPC-aware piece in this file.
// ─────────────────────────────────────────────────────────────────────────────

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
    /// Resolves canonical hashes for every height strictly after `anchor`
    /// up to `tip` inclusive. `anchor = None` starts at genesis.
    pub fn walk(client: &Client, anchor: Option<BlockHash>, tip: Height) -> Result<Self> {
        let start = match anchor {
            Some(hash) => Height::from(client.get_block_header_info(&hash)?.height + 1),
            None => Height::ZERO,
        };

        if start > tip {
            return Ok(Self {
                start,
                hashes: Vec::new(),
                by_prefix: FxHashMap::default(),
            });
        }

        let hashes = client.get_block_hashes_range(*start, *tip)?;
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
    fn offset_of(&self, hash: &BlockHash) -> Option<u32> {
        let offset = *self.by_prefix.get(&BlockHashPrefix::from(hash))?;
        (self.hashes[offset as usize] == *hash).then_some(offset)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Block parsing — cheap header peek first, full body parse only on a hit.
// ─────────────────────────────────────────────────────────────────────────────

const HEADER_LEN: usize = 80;

/// Returns the canonical offset of `bytes` if its header hashes to a
/// known canonical block, otherwise `None`. Does not allocate and does
/// not mutate `bytes`: the header is copied onto a stack buffer and
/// XOR-decoded there so an orphan short-circuits cleanly and a
/// canonical hit can still be cloned out intact.
fn peek_canonical_offset(
    bytes: &[u8],
    mut xor_state: XORIndex,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
) -> Option<u32> {
    if bytes.len() < HEADER_LEN {
        return None;
    }
    let mut header_buf = [0u8; HEADER_LEN];
    header_buf.copy_from_slice(&bytes[..HEADER_LEN]);
    xor_state.bytes(&mut header_buf, xor_bytes);
    let header = Header::consensus_decode(&mut &header_buf[..]).ok()?;
    canonical.offset_of(&BlockHash::from(header.block_hash()))
}

/// Full XOR-decode + parse for a block that has already been confirmed
/// canonical by `peek_canonical_offset`. Takes owned `bytes` so it can
/// mutate them in place and hand them to the resulting `ReadBlock`.
fn parse_canonical_body(
    mut bytes: Vec<u8>,
    metadata: BlkMetadata,
    mut xor_state: XORIndex,
    xor_bytes: XORBytes,
    height: Height,
) -> Result<ReadBlock> {
    xor_state.bytes(&mut bytes, xor_bytes);
    let mut cursor = Cursor::new(bytes);
    let header = Header::consensus_decode(&mut cursor)?;
    let bitcoin_hash = header.block_hash();
    let tx_count = VarInt::consensus_decode(&mut cursor)?.0 as usize;
    let mut txdata = Vec::with_capacity(tx_count);
    let mut tx_metadata = Vec::with_capacity(tx_count);
    let mut tx_offsets = Vec::with_capacity(tx_count);
    for _ in 0..tx_count {
        let tx_start = cursor.position() as u32;
        tx_offsets.push(tx_start);
        let tx = Transaction::consensus_decode(&mut cursor)?;
        let tx_len = cursor.position() as u32 - tx_start;
        txdata.push(tx);
        tx_metadata.push(BlkMetadata::new(metadata.position() + tx_start, tx_len));
    }

    let raw_bytes = cursor.into_inner();
    let mut block = Block::from((height, bitcoin_hash, bitcoin::Block { header, txdata }));
    block.set_raw_data(raw_bytes, tx_offsets);
    Ok(ReadBlock::from((block, metadata, tx_metadata)))
}

// ─────────────────────────────────────────────────────────────────────────────
// Public entry — drop-in replacement for `Reader::after`.
// ─────────────────────────────────────────────────────────────────────────────

impl ReaderInner {
    /// Streams every canonical block strictly after `hash` (or from
    /// genesis when `None`) up to the current chain tip, in canonical
    /// order. Uses the default parser-thread count; see
    /// [`after_canonical_with`](Self::after_canonical_with) to override.
    pub fn after_canonical(&self, hash: Option<BlockHash>) -> Result<Receiver<ReadBlock>> {
        self.after_canonical_with(hash, DEFAULT_PARSER_THREADS)
    }

    /// Like [`after_canonical`](Self::after_canonical) but with a
    /// configurable number of parser threads. `parser_threads = 1` is
    /// the minimal-thread default (1 reader + 1 parser, uncontended
    /// mutex). Higher values trade extra cores for throughput on dense
    /// ranges where the parser is the bottleneck.
    pub fn after_canonical_with(
        &self,
        hash: Option<BlockHash>,
        parser_threads: usize,
    ) -> Result<Receiver<ReadBlock>> {
        let parser_threads = parser_threads.max(1);
        let tip = self.client.get_last_height()?;
        let canonical = CanonicalRange::walk(&self.client, hash, tip)?;

        if canonical.is_empty() {
            return Ok(bounded(0).1);
        }

        let paths = BlkIndexToBlkPath::scan(&self.blocks_dir);
        *self.blk_index_to_blk_path.write() = paths.clone();

        let (send, recv) = bounded(BOUND_CAP);
        let xor_bytes = self.xor_bytes;
        let use_tail = canonical.len() <= TAIL_THRESHOLD;
        let first_blk_index = if use_tail {
            0
        } else {
            self.find_start_blk_index(Some(canonical.start), &paths, xor_bytes)
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
                error!("after_canonical pipeline failed: {e}");
            }
        });

        Ok(recv)
    }
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
struct ReorderState {
    next_offset: u32,
    /// Ahead-of-line matches keyed by canonical offset; drained
    /// contiguously each time `next_offset` advances. Bounded in
    /// practice by parser-thread scheduling jitter — see module doc.
    pending: FxHashMap<u32, ReadBlock>,
    send_to_consumer: Sender<ReadBlock>,
}

impl ReorderState {
    fn new(send_to_consumer: Sender<ReadBlock>) -> Self {
        Self {
            next_offset: 0,
            pending: FxHashMap::default(),
            send_to_consumer,
        }
    }

    /// Accepts a parsed canonical block; emits it and drains any
    /// contiguous pending matches. Returns `false` iff the consumer
    /// dropped the receiver — a pure liveness signal. Completion is
    /// checked by the caller via `next_offset`.
    fn try_emit(&mut self, offset: u32, block: ReadBlock) -> bool {
        use std::cmp::Ordering::*;
        match offset.cmp(&self.next_offset) {
            Equal => {
                if self.send_to_consumer.send(block).is_err() {
                    return false;
                }
                self.next_offset += 1;
                while let Some(next) = self.pending.remove(&self.next_offset) {
                    if self.send_to_consumer.send(next).is_err() {
                        return false;
                    }
                    self.next_offset += 1;
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
}

/// Forward pipeline: the reader (this thread) scans blk files and
/// ships canonical hits to a scoped parser pool via `parser_send`;
/// parsers decode bodies and serialise emission through `reorder`.
/// Scoped threads let every parser borrow `canonical`, `reorder`, and
/// `done` directly — no `Arc` required.
///
/// A reorder buffer is required even at `parser_threads = 1` because
/// canonical blocks can arrive out of order across blk files (bitcoind
/// doesn't write in strict chain order during initial sync, headers-
/// first body fetch, or reindex).
fn pipeline_forward(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    send: &Sender<ReadBlock>,
    parser_threads: usize,
) -> Result<()> {
    let (parser_send, parser_recv) = bounded::<ScannedBlock>(BOUND_CAP);
    let reorder = Mutex::new(ReorderState::new(send.clone()));
    let target_canonical_count = canonical.len() as u32;
    let done = AtomicBool::new(false);

    thread::scope(|scope| -> Result<()> {
        for _ in 0..parser_threads {
            let parser_recv = parser_recv.clone();
            scope.spawn(|| {
                parser_loop(
                    parser_recv,
                    &reorder,
                    &done,
                    canonical,
                    xor_bytes,
                    target_canonical_count,
                )
            });
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

    let pipeline_cancelled = done.load(Ordering::Relaxed);
    let emitted = reorder.lock().next_offset as usize;
    if !pipeline_cancelled && emitted < canonical.len() {
        return Err(Error::Internal(
            "after_canonical forward pipeline: blk files missing canonical blocks",
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
    target_canonical_count: u32,
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
                || state.next_offset >= target_canonical_count
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
            "after_canonical tail pipeline: blk files missing canonical blocks",
        ));
    }

    for block in slots.into_iter().flatten() {
        if send.send(block).is_err() {
            return Ok(());
        }
    }
    Ok(())
}
