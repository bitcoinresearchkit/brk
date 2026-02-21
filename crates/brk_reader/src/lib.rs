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
use brk_types::{BlkMetadata, BlkPosition, BlockHash, Height, ReadBlock};
pub use crossbeam::channel::Receiver;
use crossbeam::channel::bounded;
use derive_more::Deref;
use parking_lot::{RwLock, RwLockReadGuard};
use rayon::prelude::*;
use tracing::error;

mod blk_index_to_blk_path;
mod decode;
mod xor_bytes;
mod xor_index;

use decode::*;
pub use xor_bytes::*;
pub use xor_index::*;

const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];
const BOUND_CAP: usize = 50;

fn find_magic(bytes: &[u8], xor_i: &mut XORIndex, xor_bytes: XORBytes) -> Option<usize> {
    let mut window = [0u8; 4];
    for (i, &b) in bytes.iter().enumerate() {
        window.rotate_left(1);
        window[3] = xor_i.byte(b, xor_bytes);
        if window == MAGIC_BYTES {
            return Some(i + 1);
        }
    }
    None
}

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

    /// Returns a crossbeam channel receiver that streams `ReadBlock`s in chain order.
    ///
    /// Both `start` and `end` are inclusive. `None` means unbounded.
    pub fn read(&self, start: Option<Height>, end: Option<Height>) -> Receiver<ReadBlock> {
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
                    let mut xor_i = XORIndex::default();

                    let blk_index = *blk_index;

                    let Ok(mut blk_bytes_) = fs::read(blk_path) else {
                        error!("Failed to read blk file: {}", blk_path.display());
                        return ControlFlow::Break(());
                    };
                    let blk_bytes = blk_bytes_.as_mut_slice();
                    let mut i = 0;

                    loop {
                        let Some(offset) = find_magic(&blk_bytes[i..], &mut xor_i, xor_bytes)
                        else {
                            break;
                        };
                        i += offset;

                        let len = u32::from_le_bytes(
                            xor_i
                                .bytes(&mut blk_bytes[i..(i + 4)], xor_bytes)
                                .try_into()
                                .unwrap(),
                        ) as usize;
                        i += 4;

                        let position = BlkPosition::new(blk_index, i as u32);
                        let metadata = BlkMetadata::new(position, len as u32);

                        let block_bytes = (blk_bytes[i..(i + len)]).to_vec();

                        if send_bytes.send((metadata, block_bytes, xor_i)).is_err() {
                            return ControlFlow::Break(());
                        }

                        i += len;
                        xor_i.add_assign(len);
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
                            if let Ok(Some(block)) =
                                decode_block(bytes, metadata, &client, xor_i, xor_bytes, start, end, start_time, end_time)
                                && send_block.send(block).is_err()
                            {
                                return ControlFlow::Break(());
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
                                block.hash()
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

    fn find_start_blk_index(
        &self,
        target_start: Option<Height>,
        blk_index_to_blk_path: &BlkIndexToBlkPath,
        xor_bytes: XORBytes,
    ) -> Result<u16> {
        let Some(target_start) = target_start else {
            return Ok(0);
        };

        // If start is a very recent block we only look back X blk file before the last
        if let Ok(height) = self.client.get_last_height()
            && (*height).saturating_sub(*target_start) <= 3
        {
            return Ok(blk_index_to_blk_path
                .keys()
                .rev()
                .nth(2)
                .copied()
                .unwrap_or_default());
        }

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

    pub fn get_first_block_height(&self, blk_path: &PathBuf, xor_bytes: XORBytes) -> Result<Height> {
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
