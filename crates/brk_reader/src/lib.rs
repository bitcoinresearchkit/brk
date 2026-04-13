#![doc = include_str!("../README.md")]

use std::{
    collections::BTreeMap,
    fs::File,
    io::Read,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use bitcoin::{block::Header, consensus::Decodable};
use blk_index_to_blk_path::*;
use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::{BlkPosition, BlockHash, Height, ReadBlock};
pub use crossbeam::channel::Receiver;
use derive_more::Deref;
use parking_lot::{RwLock, RwLockReadGuard};

mod blk_index_to_blk_path;
mod canonical;
mod parse;
mod pipeline;
mod scan;
mod xor_bytes;
mod xor_index;

pub use canonical::CanonicalRange;
use scan::*;
pub use xor_bytes::*;
pub use xor_index::*;

/// How many blk files to step back from the binary-search hit in
/// [`ReaderInner::find_start_blk_index`]. Guards against blocks that
/// bitcoind wrote to the "current" file slightly out of height order
/// (e.g. the tail of a reorg landing in an earlier file index than
/// its successors).
const START_BLK_INDEX_BACKOFF: usize = 21;

/// Handle to a Bitcoin Core blk-file reader.
///
/// Cheap to clone (`Arc`-backed) and thread-safe: all streaming
/// methods take `&self` and the returned `Receiver<ReadBlock>` can be
/// drained from any thread.
#[derive(Debug, Clone, Deref)]
pub struct Reader(Arc<ReaderInner>);

impl Reader {
    pub fn new(blocks_dir: PathBuf, client: &Client) -> Self {
        Self(Arc::new(ReaderInner::new(blocks_dir, client.clone())))
    }
}

#[derive(Debug)]
pub struct ReaderInner {
    blk_index_to_blk_path: Arc<RwLock<BlkIndexToBlkPath>>,
    blk_file_cache: RwLock<BTreeMap<u16, File>>,
    xor_bytes: XORBytes,
    blocks_dir: PathBuf,
    client: Client,
}

impl ReaderInner {
    pub fn new(blocks_dir: PathBuf, client: Client) -> Self {
        let no_file_limit = rlimit::getrlimit(rlimit::Resource::NOFILE).unwrap_or((0, 0));
        let _ = rlimit::setrlimit(
            rlimit::Resource::NOFILE,
            no_file_limit.0.max(15_000),
            no_file_limit.1,
        );

        Self {
            xor_bytes: XORBytes::from(blocks_dir.as_path()),
            blk_index_to_blk_path: Arc::new(RwLock::new(BlkIndexToBlkPath::scan(
                blocks_dir.as_path(),
            ))),
            blk_file_cache: RwLock::new(BTreeMap::new()),
            blocks_dir,
            client,
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn blocks_dir(&self) -> &Path {
        &self.blocks_dir
    }

    pub fn blk_index_to_blk_path(&self) -> RwLockReadGuard<'_, BlkIndexToBlkPath> {
        self.blk_index_to_blk_path.read()
    }

    pub fn xor_bytes(&self) -> XORBytes {
        self.xor_bytes
    }

    /// Ensure the blk file for `blk_index` is in the file handle cache.
    fn ensure_blk_cached(&self, blk_index: u16) -> Result<()> {
        if self.blk_file_cache.read().contains_key(&blk_index) {
            return Ok(());
        }
        let blk_paths = self.blk_index_to_blk_path();
        let blk_path = blk_paths
            .get(&blk_index)
            .ok_or(Error::NotFound("Blk file not found".into()))?;
        let file = File::open(blk_path)?;
        self.blk_file_cache.write().entry(blk_index).or_insert(file);
        Ok(())
    }

    /// Read raw bytes from a blk file at the given position with XOR decoding.
    pub fn read_raw_bytes(&self, position: BlkPosition, size: usize) -> Result<Vec<u8>> {
        self.ensure_blk_cached(position.blk_index())?;

        let cache = self.blk_file_cache.read();
        let file = cache.get(&position.blk_index()).unwrap();
        let mut buffer = vec![0u8; size];
        file.read_at(&mut buffer, position.offset() as u64)?;
        XORIndex::decode_at(&mut buffer, position.offset() as usize, self.xor_bytes);
        Ok(buffer)
    }

    /// Returns a `Read` impl positioned at `position` in the blk file.
    /// Reads only the bytes requested — no upfront allocation.
    pub fn reader_at(&self, position: BlkPosition) -> Result<BlkRead<'_>> {
        self.ensure_blk_cached(position.blk_index())?;

        let mut xor_index = XORIndex::default();
        xor_index.add_assign(position.offset() as usize);

        Ok(BlkRead {
            cache: self.blk_file_cache.read(),
            blk_index: position.blk_index(),
            offset: position.offset() as u64,
            xor_index,
            xor_bytes: self.xor_bytes,
        })
    }

    // ─────────────────────────────────────────────────────────────────
    // Public streaming API — all calls delegate to `pipeline::spawn`.
    // ─────────────────────────────────────────────────────────────────

    /// Streams every canonical block strictly after `hash` (or from
    /// genesis when `None`) up to the current chain tip, in canonical
    /// order. Uses the default parser-thread count; see
    /// [`after_with`](Self::after_with) to override.
    pub fn after(&self, hash: Option<BlockHash>) -> Result<Receiver<ReadBlock>> {
        self.after_with(hash, pipeline::DEFAULT_PARSER_THREADS)
    }

    /// Like [`after`](Self::after) but with a configurable number of
    /// parser threads. `parser_threads = 1` is the minimal-thread
    /// default (1 reader + 1 parser, uncontended mutex). Higher values
    /// trade extra cores for throughput on dense ranges where the
    /// parser is the bottleneck.
    pub fn after_with(
        &self,
        hash: Option<BlockHash>,
        parser_threads: usize,
    ) -> Result<Receiver<ReadBlock>> {
        let tip = self.client.get_last_height()?;
        let canonical = CanonicalRange::walk(&self.client, hash, tip)?;
        pipeline::spawn(self, canonical, parser_threads)
    }

    /// Streams every canonical block in the inclusive height range
    /// `start..=end` in canonical order, via the same pipeline as
    /// [`after`](Self::after).
    pub fn range(&self, start: Height, end: Height) -> Result<Receiver<ReadBlock>> {
        self.range_with(start, end, pipeline::DEFAULT_PARSER_THREADS)
    }

    /// Like [`range`](Self::range) but with a configurable number of
    /// parser threads. See [`after_with`](Self::after_with) for the
    /// parser-count tradeoff.
    pub fn range_with(
        &self,
        start: Height,
        end: Height,
        parser_threads: usize,
    ) -> Result<Receiver<ReadBlock>> {
        let canonical = CanonicalRange::between(&self.client, start, end)?;
        pipeline::spawn(self, canonical, parser_threads)
    }

    /// Binary-searches `blk_index_to_blk_path` for the first file
    /// whose earliest block height is ≤ `target_start`, then backs
    /// off a few files as a safety margin for blocks that were written
    /// out of height order (see [`START_BLK_INDEX_BACKOFF`]).
    fn find_start_blk_index(
        &self,
        target_start: Option<Height>,
        blk_index_to_blk_path: &BlkIndexToBlkPath,
        xor_bytes: XORBytes,
    ) -> Result<u16> {
        let Some(target_start) = target_start else {
            return Ok(0);
        };

        let blk_indices: Vec<u16> = blk_index_to_blk_path.keys().copied().collect();
        if blk_indices.is_empty() {
            return Ok(0);
        }

        let mut left = 0;
        let mut right = blk_indices.len() - 1;
        let mut best_start_idx = 0;

        while left <= right {
            let mid = (left + right) / 2;
            let blk_index = blk_indices[mid];

            let Some(blk_path) = blk_index_to_blk_path.get(&blk_index) else {
                break;
            };
            match self.first_block_height(blk_path, xor_bytes) {
                Ok(height) if height <= target_start => {
                    best_start_idx = mid;
                    left = mid + 1;
                }
                Ok(_) => {
                    if mid == 0 {
                        break;
                    }
                    right = mid - 1;
                }
                Err(_) => {
                    left = mid + 1;
                }
            }
        }

        let final_idx = best_start_idx.saturating_sub(START_BLK_INDEX_BACKOFF);
        Ok(blk_indices.get(final_idx).copied().unwrap_or(0))
    }

    pub fn first_block_height(
        &self,
        blk_path: &Path,
        xor_bytes: XORBytes,
    ) -> Result<Height> {
        let mut file = File::open(blk_path)?;
        let mut buf = [0u8; 4096];
        let n = file.read(&mut buf)?;

        let mut xor_i = XORIndex::default();
        let magic_end = find_magic(&buf[..n], &mut xor_i, xor_bytes)
            .ok_or_else(|| Error::NotFound("No magic bytes found".into()))?;

        let size_end = magic_end + 4;
        xor_i.bytes(&mut buf[magic_end..size_end], xor_bytes);

        let header_end = size_end + 80;
        xor_i.bytes(&mut buf[size_end..header_end], xor_bytes);

        let header =
            Header::consensus_decode(&mut std::io::Cursor::new(&buf[size_end..header_end]))?;

        let height = self.client.get_block_info(&header.block_hash())?.height as u32;

        Ok(Height::new(height))
    }
}

/// Streaming reader at a position in a blk file. Reads via pread + XOR on demand.
pub struct BlkRead<'a> {
    cache: RwLockReadGuard<'a, BTreeMap<u16, File>>,
    blk_index: u16,
    offset: u64,
    xor_index: XORIndex,
    xor_bytes: XORBytes,
}

impl Read for BlkRead<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let file = self.cache.get(&self.blk_index).unwrap();
        let n = file.read_at(buf, self.offset)?;
        self.xor_index.bytes(&mut buf[..n], self.xor_bytes);
        self.offset += n as u64;
        Ok(n)
    }
}
