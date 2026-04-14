#![doc = include_str!("../README.md")]

use std::{
    collections::BTreeMap,
    fs::File,
    io::Read,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
    sync::Arc,
};

use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::{BlkPosition, BlockHash, Height, ReadBlock};
pub use crossbeam::channel::Receiver;
use parking_lot::RwLock;
use tracing::warn;

mod bisect;
mod blk_index_to_blk_path;
mod canonical;
mod parse;
mod pipeline;
mod scan;
mod xor_bytes;
mod xor_index;

pub use blk_index_to_blk_path::BlkIndexToBlkPath;
pub use canonical::CanonicalRange;
pub use xor_bytes::*;
pub use xor_index::*;

/// Files of out-of-order play to tolerate. bitcoind sometimes writes
/// blocks slightly out of height order across files (initial sync,
/// headers-first body fetch, reindex), so a single "out of bounds"
/// signal isn't enough to declare failure. Used by the forward
/// bisection backoff and the tail bailout streak.
pub(crate) const OUT_OF_ORDER_FILE_BACKOFF: usize = 21;

const TARGET_NOFILE: u64 = 15_000;

/// Bitcoin Core blk-file reader. Cheap to clone (`Arc`-backed) and
/// thread-safe: every method takes `&self` and the
/// `Receiver<Result<ReadBlock>>` from the streaming API can be
/// drained from any thread.
#[derive(Debug, Clone)]
pub struct Reader(Arc<ReaderInner>);

impl Reader {
    /// Raises the per-process `NOFILE` limit so the file-handle cache
    /// can keep one open `File` per `blkNNNNN.dat`. For tests or
    /// embeddings that don't want the process-wide rlimit side
    /// effect, use [`Self::new_without_rlimit`].
    pub fn new(blocks_dir: PathBuf, client: &Client) -> Self {
        Self::raise_fd_limit();
        Self::new_without_rlimit(blocks_dir, client)
    }

    pub fn new_without_rlimit(blocks_dir: PathBuf, client: &Client) -> Self {
        Self(Arc::new(ReaderInner {
            xor_bytes: XORBytes::from(blocks_dir.as_path()),
            blk_file_cache: RwLock::new(BTreeMap::new()),
            blocks_dir,
            client: client.clone(),
        }))
    }

    /// Called automatically by [`Self::new`]. Exposed so callers
    /// using [`Self::new_without_rlimit`] can opt in once.
    ///
    /// Raises **only the soft limit**, clamped to the current hard
    /// limit — raising the hard limit requires `CAP_SYS_RESOURCE`
    /// and would fail (dropping the entire call) on containers and
    /// unprivileged macOS user processes.
    pub fn raise_fd_limit() {
        let (soft, hard) = rlimit::getrlimit(rlimit::Resource::NOFILE).unwrap_or((0, 0));
        let new_soft = soft.max(TARGET_NOFILE).min(hard);
        if new_soft > soft
            && let Err(e) = rlimit::setrlimit(rlimit::Resource::NOFILE, new_soft, hard)
        {
            warn!("failed to raise NOFILE rlimit: {e}");
        }
    }

    pub fn client(&self) -> &Client {
        &self.0.client
    }

    pub fn blocks_dir(&self) -> &Path {
        &self.0.blocks_dir
    }

    #[inline]
    pub fn xor_bytes(&self) -> XORBytes {
        self.0.xor_bytes
    }

    /// Decode the first block in `blk_path` and resolve its height
    /// via RPC. Exposed for inspection tools (see
    /// `examples/blk_heights.rs`).
    pub fn first_block_height(&self, blk_path: &Path, xor_bytes: XORBytes) -> Result<Height> {
        bisect::first_block_height(&self.0.client, blk_path, xor_bytes)
    }

    /// `read_exact_at` so a short read becomes a hard error instead
    /// of silent corruption from the buffer's zero-init tail.
    pub fn read_raw_bytes(&self, position: BlkPosition, size: usize) -> Result<Vec<u8>> {
        let file = self.0.open_blk(position.blk_index())?;
        let mut buffer = vec![0u8; size];
        file.read_exact_at(&mut buffer, position.offset() as u64)?;
        XORIndex::decode_at(&mut buffer, position.offset() as usize, self.0.xor_bytes);
        Ok(buffer)
    }

    /// Streaming `Read` at `position`. Holds an `Arc<File>` so the
    /// cache lock isn't held across the I/O.
    pub fn reader_at(&self, position: BlkPosition) -> Result<BlkRead> {
        let file = self.0.open_blk(position.blk_index())?;
        Ok(BlkRead {
            file,
            offset: position.offset() as u64,
            xor_index: XORIndex::at_offset(position.offset() as usize),
            xor_bytes: self.0.xor_bytes,
        })
    }

    /// Streams every canonical block strictly after `hash` (or from
    /// genesis when `None`) up to the current chain tip.
    pub fn after(&self, hash: Option<BlockHash>) -> Result<Receiver<Result<ReadBlock>>> {
        self.after_with(hash, pipeline::DEFAULT_PARSER_THREADS)
    }

    /// Like [`after`](Self::after) with a configurable parser-thread
    /// count. The default of 1 reader + 1 parser leaves the rest of
    /// the cores for the indexer; bench tools that drain the channel
    /// cheaply can override.
    pub fn after_with(
        &self,
        hash: Option<BlockHash>,
        parser_threads: usize,
    ) -> Result<Receiver<Result<ReadBlock>>> {
        let tip = self.0.client.get_last_height()?;
        let canonical = CanonicalRange::walk(&self.0.client, hash.as_ref(), tip)?;
        pipeline::spawn(self.0.clone(), canonical, hash, parser_threads)
    }

    /// Inclusive height range `start..=end` in canonical order.
    pub fn range(&self, start: Height, end: Height) -> Result<Receiver<Result<ReadBlock>>> {
        self.range_with(start, end, pipeline::DEFAULT_PARSER_THREADS)
    }

    pub fn range_with(
        &self,
        start: Height,
        end: Height,
        parser_threads: usize,
    ) -> Result<Receiver<Result<ReadBlock>>> {
        let tip = self.0.client.get_last_height()?;
        if end > tip {
            return Err(Error::OutOfRange(format!(
                "range end {end} is past current tip {tip}"
            )));
        }
        let canonical = CanonicalRange::between(&self.0.client, start, end)?;
        // No anchor: caller asked for "blocks at heights X..=Y", they
        // get whatever bitcoind says is canonical there.
        pipeline::spawn(self.0.clone(), canonical, None, parser_threads)
    }
}

/// `pub(crate)` so `pipeline` can capture it via `Arc<ReaderInner>`
/// for spawned workers; everything else goes through `Reader`.
#[derive(Debug)]
pub(crate) struct ReaderInner {
    /// Invalidated on every [`refresh_paths`](Self::refresh_paths) so
    /// a pruned/reindexed blk file can't keep serving stale bytes
    /// from a dead inode. `Arc<File>` lets us hand out cheap clones
    /// without holding the cache lock during I/O.
    blk_file_cache: RwLock<BTreeMap<u16, Arc<File>>>,
    pub(crate) xor_bytes: XORBytes,
    pub(crate) blocks_dir: PathBuf,
    pub(crate) client: Client,
}

impl ReaderInner {
    /// Rescan the blocks directory and drop the file-handle cache in
    /// the same critical section. Old `Arc<File>`s already in flight
    /// stay valid until their last drop; new lookups go through the
    /// fresh inode.
    pub(crate) fn refresh_paths(&self) -> Result<BlkIndexToBlkPath> {
        let paths = BlkIndexToBlkPath::scan(&self.blocks_dir)?;
        self.blk_file_cache.write().clear();
        Ok(paths)
    }

    /// The blk path is deterministic (`<blocks_dir>/blkNNNNN.dat`),
    /// so we don't need a directory scan to resolve it. Two threads
    /// racing on a missing entry will both call `File::open`; the
    /// loser's `Arc` is dropped via `or_insert`.
    fn open_blk(&self, blk_index: u16) -> Result<Arc<File>> {
        if let Some(file) = self.blk_file_cache.read().get(&blk_index).cloned() {
            return Ok(file);
        }
        let path = self.blocks_dir.join(format!("blk{blk_index:05}.dat"));
        let file = Arc::new(File::open(&path)?);
        let mut cache = self.blk_file_cache.write();
        Ok(cache.entry(blk_index).or_insert(file).clone())
    }
}

/// Streaming reader at a position in a blk file. Holds an `Arc<File>`
/// so it doesn't lock the file cache while the consumer is reading.
pub struct BlkRead {
    file: Arc<File>,
    offset: u64,
    xor_index: XORIndex,
    xor_bytes: XORBytes,
}

impl Read for BlkRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.file.read_at(buf, self.offset)?;
        self.xor_index.bytes(&mut buf[..n], self.xor_bytes);
        self.offset += n as u64;
        Ok(n)
    }
}
