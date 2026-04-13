//! Canonical-hash pipeline for `Reader::after`.
//!
//! Three pieces, each with one job:
//!
//! * **`CanonicalRange::walk`** is the only place bitcoind is consulted
//!   about the main chain. It batch-fetches every canonical hash in the
//!   target window once, up front, via `getblockhash` JSON-RPC batching.
//! * **`parse_canonical_block`** is a pure function of raw blk bytes.
//!   It XOR-decodes only the 80-byte header, looks the hash up in the
//!   pre-fetched `CanonicalRange`, and short-circuits orphans before
//!   touching the (expensive) transaction body. No RPC, no `confirmations`
//!   filter, no chain logic.
//! * **`pipeline_forward` / `pipeline_tail`** wire the scan loop to a
//!   parser pool. The forward pipeline runs 1 reader + N parser threads
//!   (default `N = 1`, configurable via `after_canonical_with`); the
//!   tail pipeline (≤10 blocks) stays inline on a single thread because
//!   channel/lock overhead would dominate.
//!
//! Coexists with the original `read`/`read_rev`/`after` so the two can be
//! A/B-tested from the indexer.

use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek, SeekFrom},
    ops::ControlFlow,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
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

use crate::{
    BlkIndexToBlkPath, ReaderInner, XORBytes, XORIndex,
    scan::{ScanResult, scan_bytes},
};

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

/// Forward-ordered canonical hashes for `start..=end`, resolved once up front.
///
/// `hashes[i]` is the canonical block hash at height `start + i`.
/// `by_prefix` maps the 8-byte `BlockHashPrefix` of every canonical hash to
/// its offset — same prefix-keyed scheme brk already uses in `stores`.
/// Lookups verify the full hash via `hashes[offset]`, neutralising the
/// (astronomically small) prefix collision risk at zero extra cost.
pub struct CanonicalRange {
    pub start: Height,
    pub end: Height,
    hashes: Vec<BlockHash>,
    by_prefix: FxHashMap<BlockHashPrefix, u32>,
}

impl CanonicalRange {
    /// Resolves canonical hashes for every height strictly after `anchor`
    /// up to `tip` inclusive. If `anchor` is `None`, starts at genesis.
    ///
    /// Uses `get_block_hash(h)` which is a deterministic height → canonical
    /// hash lookup — no race window against in-progress reorgs.
    pub fn walk(client: &Client, anchor: Option<BlockHash>, tip: Height) -> Result<Self> {
        let start = match anchor {
            Some(hash) => {
                let info = client.get_block_header_info(&hash)?;
                Height::from(info.height + 1)
            }
            None => Height::ZERO,
        };

        if start > tip {
            return Ok(Self::empty(start));
        }

        let len = (*tip - *start + 1) as usize;
        let hashes = client.get_block_hashes_range(*start, *tip)?;

        let mut by_prefix = FxHashMap::with_capacity_and_hasher(len, Default::default());
        for (offset, hash) in hashes.iter().enumerate() {
            by_prefix.insert(BlockHashPrefix::from(hash), offset as u32);
        }

        Ok(Self {
            start,
            end: tip,
            hashes,
            by_prefix,
        })
    }

    fn empty(start: Height) -> Self {
        Self {
            start,
            end: start,
            hashes: Vec::new(),
            by_prefix: FxHashMap::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.hashes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }

    /// Returns the offset-from-start of `hash` iff it matches a canonical
    /// block in this range. A prefix hit is verified against the stored
    /// full hash to rule out the (vanishing) chance of prefix collisions
    /// from unrelated orphans in blk files.
    #[inline]
    fn offset_of(&self, hash: &BlockHash) -> Option<u32> {
        let offset = *self.by_prefix.get(&BlockHashPrefix::from(hash))?;
        (self.hashes[offset as usize] == *hash).then_some(offset)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pure block parser — no client, no confirmations, no Ok(None) on RPC errors.
// ─────────────────────────────────────────────────────────────────────────────

const HEADER_LEN: usize = 80;

/// XOR-decode just the 80-byte header, compute the block hash, look it
/// up in `canonical`, and only proceed to parse the body and transactions
/// when the block is on the canonical chain. Returning early before the
/// body decode is what lets a single parser thread keep up with the
/// 4-thread `read()` pool on sparse ranges.
///
/// Returns `Ok(None)` for orphans / out-of-range blocks. Deterministic —
/// never touches RPC.
fn parse_canonical_block(
    mut bytes: Vec<u8>,
    metadata: BlkMetadata,
    mut xor_i: XORIndex,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
) -> Result<Option<(u32, ReadBlock)>> {
    if bytes.len() < HEADER_LEN {
        return Err(Error::Internal("Block bytes shorter than header"));
    }

    // Decode just the header and look the hash up before paying for the
    // body. `xor_i` advances `HEADER_LEN` here so it stays in lock-step
    // with the decoded prefix.
    xor_i.bytes(&mut bytes[..HEADER_LEN], xor_bytes);
    let header = Header::consensus_decode(&mut &bytes[..HEADER_LEN])?;
    let bitcoin_hash = header.block_hash();

    let Some(offset) = canonical.offset_of(&BlockHash::from(bitcoin_hash)) else {
        return Ok(None);
    };

    // Canonical: XOR-decode the body and parse transactions.
    xor_i.bytes(&mut bytes[HEADER_LEN..], xor_bytes);
    let mut cursor = Cursor::new(bytes);
    cursor.set_position(HEADER_LEN as u64);
    let tx_count = VarInt::consensus_decode(&mut cursor)?.0 as usize;
    let mut txdata = Vec::with_capacity(tx_count);
    let mut tx_metadata = Vec::with_capacity(tx_count);
    let mut tx_offsets = Vec::with_capacity(tx_count);
    for _ in 0..tx_count {
        let off = cursor.position() as u32;
        tx_offsets.push(off);
        let position = metadata.position() + off;
        let tx = Transaction::consensus_decode(&mut cursor)?;
        txdata.push(tx);
        let len = cursor.position() as u32 - off;
        tx_metadata.push(BlkMetadata::new(position, len));
    }

    let raw_bytes = cursor.into_inner();
    let height = Height::from(*canonical.start + offset);
    let mut block = Block::from((height, bitcoin_hash, bitcoin::Block { header, txdata }));
    block.set_raw_data(raw_bytes, tx_offsets);
    Ok(Some((offset, ReadBlock::from((block, metadata, tx_metadata)))))
}

// ─────────────────────────────────────────────────────────────────────────────
// Public entry — drop-in replacement for `Reader::after`.
// ─────────────────────────────────────────────────────────────────────────────

impl ReaderInner {
    /// Stream every canonical block strictly after `hash` (or from
    /// genesis if `None`) up to the current chain tip, in canonical
    /// order, via the canonical-hash pipeline.
    ///
    /// Uses the default parser-thread count (`1`); see
    /// `after_canonical_with` to override.
    pub fn after_canonical(&self, hash: Option<BlockHash>) -> Result<Receiver<ReadBlock>> {
        self.after_canonical_with(hash, DEFAULT_PARSER_THREADS)
    }

    /// Same as `after_canonical` but with a configurable number of parser
    /// threads. `parser_threads = 1` is the minimal-thread default
    /// (1 reader + 1 parser, uncontended mutex hot path). Higher values
    /// trade extra cores for throughput on dense ranges where the parser
    /// is the bottleneck.
    pub fn after_canonical_with(
        &self,
        hash: Option<BlockHash>,
        parser_threads: usize,
    ) -> Result<Receiver<ReadBlock>> {
        let parser_threads = parser_threads.max(1);
        let tip = self.client.get_last_height()?;
        let canonical = Arc::new(CanonicalRange::walk(&self.client, hash, tip)?);

        if canonical.is_empty() {
            return Ok(bounded(0).1);
        }

        // Refresh the blk path cache once, on the caller's thread, so the
        // worker thread below has a stable view.
        let paths = BlkIndexToBlkPath::scan(&self.blocks_dir);
        *self.blk_index_to_blk_path.write() = paths.clone();

        let (send, recv) = bounded(BOUND_CAP);
        let xor_bytes = self.xor_bytes;

        if canonical.len() <= TAIL_THRESHOLD {
            thread::spawn(move || {
                if let Err(e) = pipeline_tail(&paths, xor_bytes, &canonical, &send) {
                    error!("after_canonical tail pipeline failed: {e}");
                }
            });
        } else {
            let first_blk_index = self
                .find_start_blk_index(Some(canonical.start), &paths, xor_bytes)
                .unwrap_or_default();
            thread::spawn(move || {
                if let Err(e) = pipeline_forward(
                    &paths,
                    first_blk_index,
                    xor_bytes,
                    canonical,
                    &send,
                    parser_threads,
                ) {
                    error!("after_canonical forward pipeline failed: {e}");
                }
            });
        }

        Ok(recv)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Forward pipeline — 1 reader + N parsers + shared in-order emission.
// ─────────────────────────────────────────────────────────────────────────────

/// Item shipped from the reader thread to the parser pool: raw block
/// bytes, blk-file metadata, and the XOR state at the byte the bytes
/// start at.
type ScannedItem = (BlkMetadata, Vec<u8>, XORIndex);

/// Shared in-order emission buffer used by N parser threads. The mutex
/// is uncontended at `parser_threads = 1` (still acquired, never blocks).
struct ReorderState {
    next_offset: u32,
    target_len: u32,
    /// Ahead-of-line matches keyed by canonical offset; drained
    /// contiguously each time `next_offset` advances.
    pending: FxHashMap<u32, ReadBlock>,
    send_to_consumer: Sender<ReadBlock>,
}

impl ReorderState {
    fn new(send_to_consumer: Sender<ReadBlock>, target_len: u32) -> Self {
        Self {
            next_offset: 0,
            target_len,
            pending: FxHashMap::default(),
            send_to_consumer,
        }
    }

    /// Insert a parsed canonical block. Returns `false` once the pipeline
    /// is done — either the consumer dropped the receiver, every canonical
    /// block has been emitted, or a parser somehow produced a duplicate
    /// offset — so the caller should stop processing and exit.
    fn try_emit(&mut self, offset: u32, block: ReadBlock) -> bool {
        use std::cmp::Ordering::*;
        match offset.cmp(&self.next_offset) {
            Equal => {
                if self.send_to_consumer.send(block).is_err() {
                    return false;
                }
                self.next_offset += 1;
                while let Some(b) = self.pending.remove(&self.next_offset) {
                    if self.send_to_consumer.send(b).is_err() {
                        return false;
                    }
                    self.next_offset += 1;
                }
                self.next_offset < self.target_len
            }
            Greater => {
                self.pending.insert(offset, block);
                true
            }
            // Each canonical hash appears at exactly one offset, and
            // each block is parsed once, so a parser should never
            // produce an offset below `next_offset`. Treat as done.
            Less => false,
        }
    }
}

/// Two-stage pipeline:
///
/// 1. **Reader (this thread)** — walks blk files from `first_blk_index`,
///    `fs::read`s each one, runs `scan_bytes` to locate every block, and
///    ships `ScannedItem`s over an mpmc channel to the parser pool.
/// 2. **Parser pool** — `parser_threads` workers draining the same
///    channel. Each worker runs `parse_canonical_block` (header first,
///    body only on canonical match) and acquires the shared `ReorderState`
///    mutex to insert into the in-order emission buffer.
///
/// Canonical blocks can arrive out of order across blk files (bitcoind
/// doesn't write in strict chain order during initial sync, headers-first
/// body fetch, or reindex), so the reorder buffer is required even with
/// a single parser thread.
fn pipeline_forward(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    canonical: Arc<CanonicalRange>,
    send: &Sender<ReadBlock>,
    parser_threads: usize,
) -> Result<()> {
    let (parser_send, parser_recv) = bounded::<ScannedItem>(BOUND_CAP);
    let reorder = Arc::new(Mutex::new(ReorderState::new(
        send.clone(),
        canonical.len() as u32,
    )));
    // Set when the pipeline is finished (consumer dropped or all canonical
    // blocks emitted) so parsers can short-circuit instead of burning CPU
    // on doomed work while the reader drains the queue.
    let done = Arc::new(AtomicBool::new(false));

    let parsers = spawn_parser_pool(
        parser_threads,
        &parser_recv,
        &reorder,
        &done,
        &canonical,
        xor_bytes,
    );
    drop(parser_recv); // parsers own clones; this would otherwise keep the channel open

    let read_result = read_and_dispatch(paths, first_blk_index, xor_bytes, &parser_send, &done);
    drop(parser_send); // signal end-of-input to parsers

    for parser in parsers {
        parser
            .join()
            .map_err(|_| Error::Internal("parser thread panicked"))??;
    }
    read_result?;

    let state = reorder.lock();
    if (state.next_offset as usize) < canonical.len() && !done.load(Ordering::Relaxed) {
        return Err(Error::Internal(
            "after_canonical forward pipeline: blk files missing canonical blocks",
        ));
    }
    Ok(())
}

/// Spawn `n` parser threads that drain `parser_recv`, parse each scanned
/// item via `parse_canonical_block`, and emit canonical matches to
/// `reorder`. Parsers exit when the channel closes or `done` is set.
fn spawn_parser_pool(
    n: usize,
    parser_recv: &Receiver<ScannedItem>,
    reorder: &Arc<Mutex<ReorderState>>,
    done: &Arc<AtomicBool>,
    canonical: &Arc<CanonicalRange>,
    xor_bytes: XORBytes,
) -> Vec<thread::JoinHandle<Result<()>>> {
    (0..n)
        .map(|_| {
            let parser_recv = parser_recv.clone();
            let reorder = reorder.clone();
            let done = done.clone();
            let canonical = canonical.clone();
            thread::spawn(move || -> Result<()> {
                for (metadata, bytes, xor_i) in parser_recv {
                    if done.load(Ordering::Relaxed) {
                        continue; // drain quietly
                    }

                    let (offset, block) = match parse_canonical_block(
                        bytes, metadata, xor_i, xor_bytes, &canonical,
                    ) {
                        Ok(Some(item)) => item,
                        Ok(None) => continue, // orphan / out of range
                        Err(e) => {
                            warn!("parse_canonical_block failed: {e}");
                            continue;
                        }
                    };

                    if !reorder.lock().try_emit(offset, block) {
                        done.store(true, Ordering::Relaxed);
                    }
                }
                Ok(())
            })
        })
        .collect()
}

/// Walk blk files from `first_blk_index`, scan each one, and ship every
/// raw block found to the parser pool. Stops early if `done` flips or
/// the parser channel closes.
fn read_and_dispatch(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    parser_send: &Sender<ScannedItem>,
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
            |metadata, block_bytes, xor_i| {
                if done.load(Ordering::Relaxed)
                    || parser_send.send((metadata, block_bytes, xor_i)).is_err()
                {
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
// Tail pipeline — reverse 5MB chunks of the last blk files until we've
// collected every canonical hash, then emit forward.
// ─────────────────────────────────────────────────────────────────────────────

fn pipeline_tail(
    paths: &BlkIndexToBlkPath,
    xor_bytes: XORBytes,
    canonical: &Arc<CanonicalRange>,
    send: &Sender<ReadBlock>,
) -> Result<()> {
    // Collected matches, keyed by canonical offset. Tail ranges are ≤10 so
    // a Vec<Option<_>> is the simplest representation.
    let mut collected: Vec<Option<ReadBlock>> = (0..canonical.len()).map(|_| None).collect();
    let mut remaining = canonical.len();

    'files: for (&blk_index, path) in paths.iter().rev() {
        let file_len = fs::metadata(path).map(|m| m.len() as usize).unwrap_or(0);
        if file_len == 0 {
            continue;
        }
        let Ok(mut file) = File::open(path) else {
            return Err(Error::Internal("Failed to open blk file"));
        };

        let mut read_end = file_len;
        let mut head: Vec<u8> = Vec::new();

        while read_end > 0 && remaining > 0 {
            let read_start = read_end.saturating_sub(TAIL_CHUNK);
            let chunk_len = read_end - read_start;
            read_end = read_start;

            if file.seek(SeekFrom::Start(read_start as u64)).is_err() {
                return Err(Error::Internal("Failed to seek blk file"));
            }
            let mut buf = vec![0u8; chunk_len + head.len()];
            if file.read_exact(&mut buf[..chunk_len]).is_err() {
                return Err(Error::Internal("Failed to read blk chunk"));
            }
            buf[chunk_len..].copy_from_slice(&head);
            head.clear();

            let result: ScanResult = scan_bytes(
                &mut buf,
                blk_index,
                read_start,
                xor_bytes,
                |metadata, block_bytes, xor_i| {
                    match parse_canonical_block(block_bytes, metadata, xor_i, xor_bytes, canonical)
                    {
                        Ok(Some((offset, block))) => {
                            let slot = &mut collected[offset as usize];
                            if slot.is_none() {
                                *slot = Some(block);
                                remaining -= 1;
                            }
                        }
                        Ok(None) => {} // orphan / out of range
                        Err(e) => warn!("parse_canonical_block failed in tail pipeline: {e}"),
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
                head = buf[..result.first_magic.unwrap_or(buf.len())].to_vec();
            }
        }
    }

    if remaining > 0 {
        return Err(Error::Internal(
            "after_canonical tail pipeline: blk files missing canonical blocks",
        ));
    }

    // `remaining == 0` above guarantees every slot is `Some`; `flatten`
    // is just the natural way to write the emit loop.
    for block in collected.into_iter().flatten() {
        if send.send(block).is_err() {
            return Ok(());
        }
    }
    Ok(())
}
