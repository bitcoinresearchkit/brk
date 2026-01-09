#![doc = include_str!("../README.md")]

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    mem,
    ops::ControlFlow,
    path::PathBuf,
    sync::Arc,
    thread,
    time::Duration,
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

mod any_block;
mod blk_index_to_blk_path;
mod xor_bytes;
mod xor_index;

use any_block::*;
pub use xor_bytes::*;
pub use xor_index::*;

const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];
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

    ///
    /// Returns a crossbeam channel receiver that receives `Block` from an **inclusive** range (`start` and `end`)
    ///
    /// For an example checkout `./main.rs`
    ///
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

        thread::spawn(move || {
            let _ = blk_index_to_blk_path.range(first_blk_index..).try_for_each(
                move |(blk_index, blk_path)| {
                    let mut xor_i = XORIndex::default();

                    let blk_index = *blk_index;

                    let mut blk_bytes_ = fs::read(blk_path).unwrap();
                    let blk_bytes = blk_bytes_.as_mut_slice();
                    let blk_bytes_len = blk_bytes.len();

                    let mut current_4bytes = [0; 4];

                    let mut i = 0;

                    'parent: loop {
                        loop {
                            if i == blk_bytes_len {
                                break 'parent;
                            }

                            current_4bytes.rotate_left(1);

                            current_4bytes[3] = xor_i.byte(blk_bytes[i], xor_bytes);
                            i += 1;

                            if current_4bytes == MAGIC_BYTES {
                                break;
                            }
                        }

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

                        if send_bytes
                            .send((metadata, AnyBlock::Raw(block_bytes), xor_i))
                            .is_err()
                        {
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
            let xor_bytes = xor_bytes;

            let mut bulk = vec![];

            // Private pool to prevent collision with the global pool
            // Without it there can be hanging
            let parser_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread::available_parallelism().unwrap().get() / 2)
                .build()
                .expect("Failed to create parser thread pool");

            let drain_and_send = |bulk: &mut Vec<(BlkMetadata, AnyBlock, XORIndex)>| {
                parser_pool.install(|| {
                    mem::take(bulk)
                        .into_par_iter()
                        .try_for_each(|(metdata, any_block, xor_i)| {
                            if let Ok(AnyBlock::Decoded(block)) =
                                any_block.decode(metdata, &client, xor_i, xor_bytes, start, end)
                                && send_block.send(block).is_err()
                            {
                                return ControlFlow::Break(());
                            }
                            ControlFlow::Continue(())
                        })
                })
            };

            recv_bytes.iter().try_for_each(|tuple| {
                bulk.push(tuple);

                if bulk.len() < BOUND_CAP / 2 {
                    return ControlFlow::Continue(());
                }

                while send_block.len() >= bulk.len() {
                    thread::sleep(Duration::from_micros(100));
                }
                drain_and_send(&mut bulk)
            })?;

            drain_and_send(&mut bulk)
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

                        if end.is_some_and(|end| end == current_height) {
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
                .cloned()
                .unwrap_or_default());
        }

        let blk_indices: Vec<u16> = blk_index_to_blk_path
            .range(0..)
            .map(|(idx, _)| *idx)
            .collect();

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
                            if mid == usize::MAX {
                                break;
                            }
                            left = mid + 1;
                        } else {
                            if mid == 0 {
                                break;
                            }
                            right = mid - 1;
                        }
                    }
                    Err(_) => {
                        if mid == usize::MAX {
                            break;
                        }
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

    fn get_first_block_height(&self, blk_path: &PathBuf, xor_bytes: XORBytes) -> Result<Height> {
        let mut file = File::open(blk_path)?;
        let mut xor_i = XORIndex::default();
        let mut current_4bytes = [0; 4];
        let mut byte_buffer = [0u8; 1];

        loop {
            if file.read_exact(&mut byte_buffer).is_err() {
                return Err(Error::NotFound("No magic bytes found".into()));
            }

            current_4bytes.rotate_left(1);
            current_4bytes[3] = xor_i.byte(byte_buffer[0], xor_bytes);

            if current_4bytes == MAGIC_BYTES {
                break;
            }
        }

        let mut size_bytes = [0u8; 4];
        file.read_exact(&mut size_bytes)?;
        let _block_size =
            u32::from_le_bytes(xor_i.bytes(&mut size_bytes, xor_bytes).try_into().unwrap());

        let mut header_bytes = [0u8; 80];
        file.read_exact(&mut header_bytes)?;
        xor_i.bytes(&mut header_bytes, xor_bytes);

        let header = Header::consensus_decode(&mut std::io::Cursor::new(&header_bytes))?;

        let height = self.client.get_block_info(&header.block_hash())?.height as u32;

        Ok(Height::new(height))
    }
}
