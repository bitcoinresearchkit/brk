#![doc = include_str!("../README.md")]

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    ops::ControlFlow,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use bitcoin::{block::Header, consensus::Decodable};
use blk_index_to_blk_path::*;
use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::{BlkPosition, BlockHash, Height, ReadBlock};
pub use crossbeam::channel::Receiver;
use crossbeam::channel::bounded;
use derive_more::Deref;
use parking_lot::{RwLock, RwLockReadGuard};
use rayon::prelude::*;
use tracing::{error, warn};

mod blk_index_to_blk_path;
mod decode;
mod scan;
mod xor_bytes;
mod xor_index;

use decode::*;
use scan::*;
pub use xor_bytes::*;
pub use xor_index::*;

const BOUND_CAP: usize = 50;

///
/// Bitcoin BLK file reader
///
/// Thread safe and free to clone
///
///
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
    xor_bytes: XORBytes,
    blocks_dir: PathBuf,
    client: Client,
}

impl ReaderInner {
    pub fn new(blocks_dir: PathBuf, client: Client) -> Self {
        Self {
            xor_bytes: XORBytes::from(blocks_dir.as_path()),
            blk_index_to_blk_path: Arc::new(RwLock::new(BlkIndexToBlkPath::scan(
                blocks_dir.as_path(),
            ))),
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

    /// Read raw bytes from a blk file at the given position with XOR decoding
    pub fn read_raw_bytes(&self, position: BlkPosition, size: usize) -> Result<Vec<u8>> {
        let blk_paths = self.blk_index_to_blk_path();
        let blk_path = blk_paths
            .get(&position.blk_index())
            .ok_or(Error::NotFound("Blk file not found".into()))?;

        let mut file = File::open(blk_path)?;
        file.seek(SeekFrom::Start(position.offset() as u64))?;

        let mut buffer = vec![0u8; size];
        file.read_exact(&mut buffer)?;

        let mut xori = XORIndex::default();
        xori.add_assign(position.offset() as usize);
        xori.bytes(&mut buffer, self.xor_bytes);

        Ok(buffer)
    }

    /// Returns a receiver streaming `ReadBlock`s from `hash + 1` to the chain tip.
    /// If `hash` is `None`, starts from genesis.
    pub fn after(&self, hash: Option<BlockHash>) -> Result<Receiver<ReadBlock>> {
        let start = if let Some(hash) = hash.as_ref() {
            let info = self.client.get_block_header_info(hash)?;
            Height::from(info.height + 1)
        } else {
            Height::ZERO
        };
        let end = self.client.get_last_height()?;

        if end < start {
            return Ok(bounded(0).1);
        }

        if *end - *start < 10 {
            let mut blocks: Vec<_> = self.read_rev(Some(start), Some(end)).iter().collect();
            blocks.reverse();

            let (send, recv) = bounded(blocks.len());
            for block in blocks {
                let _ = send.send(block);
            }
            return Ok(recv);
        }

        Ok(self.read(Some(start), Some(end)))
    }

    /// Returns a crossbeam channel receiver that streams `ReadBlock`s in chain order.
    ///
    /// Both `start` and `end` are inclusive. `None` means unbounded.
    pub fn read(&self, start: Option<Height>, end: Option<Height>) -> Receiver<ReadBlock> {
        if let (Some(s), Some(e)) = (start, end)
            && s > e
        {
            let (_, recv) = bounded(0);
            return recv;
        }

        let client = self.client.clone();

        let (send_bytes, recv_bytes) = bounded(BOUND_CAP / 2);
        let (send_block, recv_block) = bounded(BOUND_CAP);
        let (send_ordered, recv_ordered) = bounded(BOUND_CAP);

        let blk_index_to_blk_path = BlkIndexToBlkPath::scan(&self.blocks_dir);
        *self.blk_index_to_blk_path.write() = blk_index_to_blk_path.clone();

        let xor_bytes = self.xor_bytes;

        let first_blk_index = self
            .find_start_blk_index(start, &blk_index_to_blk_path, xor_bytes)
            .unwrap_or_default();

        let get_block_time = |h: Height| -> u32 {
            self.client
                .get_block_hash(*h as u64)
                .ok()
                .and_then(|hash| self.client.get_block_header(&hash).ok())
                .map(|h| h.time)
                .unwrap_or(0)
        };

        let start_time = start.filter(|h| **h > 0).map(&get_block_time).unwrap_or(0);
        let end_time = end.map(&get_block_time).unwrap_or(0);

        thread::spawn(move || {
            let _ = blk_index_to_blk_path.range(first_blk_index..).try_for_each(
                move |(blk_index, blk_path)| {
                    let Ok(mut bytes) = fs::read(blk_path) else {
                        error!("Failed to read blk file: {}", blk_path.display());
                        return ControlFlow::Break(());
                    };
                    let result = scan_bytes(
                        &mut bytes,
                        *blk_index,
                        0,
                        xor_bytes,
                        |metadata, block_bytes, xor_i| {
                            if send_bytes.send((metadata, block_bytes, xor_i)).is_err() {
                                return ControlFlow::Break(());
                            }
                            ControlFlow::Continue(())
                        },
                    );
                    if result.interrupted {
                        return ControlFlow::Break(());
                    }
                    ControlFlow::Continue(())
                },
            );
        });

        thread::spawn(move || {
            // Private pool to prevent collision with the global pool
            let parser_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(4.min(thread::available_parallelism().unwrap().get() / 2))
                .build()
                .expect("Failed to create parser thread pool");

            parser_pool.install(|| {
                let _ =
                    recv_bytes
                        .into_iter()
                        .par_bridge()
                        .try_for_each(|(metadata, bytes, xor_i)| {
                            let position = metadata.position();
                            match decode_block(
                                bytes, metadata, &client, xor_i, xor_bytes, start, end, start_time,
                                end_time,
                            ) {
                                Ok(Some(block)) => {
                                    if send_block.send(block).is_err() {
                                        return ControlFlow::Break(());
                                    }
                                }
                                Ok(None) => {} // Block filtered out (outside range, unconfirmed)
                                Err(e) => {
                                    warn!("Failed to decode block at {position}: {e}");
                                }
                            }
                            ControlFlow::Continue(())
                        });
            });
        });

        thread::spawn(move || {
            let mut current_height = start.unwrap_or_default();
            let mut prev_hash: Option<BlockHash> = None;
            let mut future_blocks = BTreeMap::default();

            let _ = recv_block
                .iter()
                .try_for_each(|block| -> ControlFlow<(), _> {
                    let mut opt = if current_height == block.height() {
                        Some(block)
                    } else {
                        future_blocks.insert(block.height(), block);
                        None
                    };

                    while let Some(block) = opt.take().or_else(|| {
                        if !future_blocks.is_empty() {
                            future_blocks.remove(&current_height)
                        } else {
                            None
                        }
                    }) {
                        if let Some(expected_prev) = prev_hash.as_ref() && block.header.prev_blockhash != expected_prev.into() {
                            error!(
                                "Chain discontinuity detected at height {}: expected prev_hash {}, got {}. Stopping iteration.",
                                *block.height(),
                                expected_prev,
                                block.header.prev_blockhash
                            );
                            return ControlFlow::Break(());
                        }

                        prev_hash = Some(block.hash().clone());

                        if send_ordered.send(block).is_err() {
                            return ControlFlow::Break(());
                        }

                        current_height.increment();

                        if end.is_some_and(|end| current_height > end) {
                            return ControlFlow::Break(());
                        }
                    }

                    ControlFlow::Continue(())
                });
        });

        recv_ordered
    }

    /// Streams `ReadBlock`s in reverse order (newest first) by scanning
    /// `.blk` files from the tail. Efficient for reading recent blocks.
    /// Both `start` and `end` are inclusive. `None` means unbounded.
    pub fn read_rev(&self, start: Option<Height>, end: Option<Height>) -> Receiver<ReadBlock> {
        const CHUNK: usize = 5 * 1024 * 1024;

        if let (Some(s), Some(e)) = (start, end)
            && s > e
        {
            return bounded(0).1;
        }

        let client = self.client.clone();
        let xor_bytes = self.xor_bytes;
        let paths = BlkIndexToBlkPath::scan(&self.blocks_dir);
        *self.blk_index_to_blk_path.write() = paths.clone();
        let (send, recv) = bounded(BOUND_CAP);

        thread::spawn(move || {
            let mut head = Vec::new();

            for (&blk_index, path) in paths.iter().rev() {
                let file_len = fs::metadata(path).map(|m| m.len() as usize).unwrap_or(0);
                if file_len == 0 {
                    continue;
                }
                let Ok(mut file) = File::open(path) else {
                    return;
                };
                let mut read_end = file_len;

                while read_end > 0 {
                    let read_start = read_end.saturating_sub(CHUNK);
                    let chunk_len = read_end - read_start;
                    read_end = read_start;

                    let _ = file.seek(SeekFrom::Start(read_start as u64));
                    let mut buf = vec![0u8; chunk_len + head.len()];
                    if file.read_exact(&mut buf[..chunk_len]).is_err() {
                        return;
                    }
                    buf[chunk_len..].copy_from_slice(&head);
                    head.clear();

                    let mut blocks = Vec::new();
                    let result = scan_bytes(
                        &mut buf,
                        blk_index,
                        read_start,
                        xor_bytes,
                        |metadata, bytes, xor_i| {
                            if let Ok(Some(block)) = decode_block(
                                bytes, metadata, &client, xor_i, xor_bytes, start, end, 0, 0,
                            ) {
                                blocks.push(block);
                            }
                            ControlFlow::Continue(())
                        },
                    );

                    for block in blocks.into_iter().rev() {
                        let done = start.is_some_and(|s| block.height() <= s);
                        if send.send(block).is_err() || done {
                            return;
                        }
                    }

                    if read_start > 0 {
                        head = buf[..result.first_magic.unwrap_or(buf.len())].to_vec();
                    }
                }
            }
        });

        recv
    }

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

            if let Some(blk_path) = blk_index_to_blk_path.get(&blk_index) {
                match self.get_first_block_height(blk_path, xor_bytes) {
                    Ok(height) => {
                        if height <= target_start {
                            best_start_idx = mid;
                            left = mid + 1;
                        } else {
                            if mid == 0 {
                                break;
                            }
                            right = mid - 1;
                        }
                    }
                    Err(_) => {
                        left = mid + 1;
                    }
                }
            } else {
                break;
            }
        }

        // buffer for worst-case scenarios when a block as far behind
        let final_idx = best_start_idx.saturating_sub(21);

        Ok(blk_indices.get(final_idx).copied().unwrap_or(0))
    }

    pub fn get_first_block_height(
        &self,
        blk_path: &PathBuf,
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
