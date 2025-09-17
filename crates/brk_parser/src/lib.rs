#![doc = include_str!("../README.md")]

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Read,
    mem,
    ops::ControlFlow,
    path::PathBuf,
    thread,
};

use bitcoin::{block::Header, consensus::Decodable};
use bitcoincore_rpc::RpcApi;
use blk_index_to_blk_path::*;
use brk_error::Result;
use brk_structs::{Block, BlockPosition, Height, ParsedBlock};
use crossbeam::channel::{Receiver, bounded};
use rayon::prelude::*;

mod any_block;
mod blk_index_to_blk_path;
mod xor_bytes;
mod xor_index;

use any_block::*;
use xor_bytes::*;
use xor_index::*;

pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 100;

const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];
const BOUND_CAP: usize = 50;

pub struct Parser {
    blocks_dir: PathBuf,
    rpc: &'static bitcoincore_rpc::Client,
}

impl Parser {
    pub fn new(blocks_dir: PathBuf, rpc: &'static bitcoincore_rpc::Client) -> Self {
        Self { blocks_dir, rpc }
    }

    pub fn get(&self, height: Height) -> Result<Block> {
        Ok((
            height,
            self.rpc
                .get_block(&self.rpc.get_block_hash(height.into())?)?,
        )
            .into())
    }

    ///
    /// Returns a crossbeam channel receiver that receives `Block` from an **inclusive** range (`start` and `end`)
    ///
    /// For an example checkout `./main.rs`
    ///
    pub fn parse(&self, start: Option<Height>, end: Option<Height>) -> Receiver<ParsedBlock> {
        let blocks_dir = self.blocks_dir.as_path();
        let rpc = self.rpc;

        let (send_bytes, recv_bytes) = bounded(BOUND_CAP);
        let (send_block, recv_block) = bounded(BOUND_CAP);
        let (send_ordered, recv_ordered) = bounded(BOUND_CAP);

        let blk_index_to_blk_path = BlkIndexToBlkPath::scan(blocks_dir);

        let xor_bytes = XORBytes::from(blocks_dir);

        let first_blk_index = self
            .find_start_blk_index(start, &blk_index_to_blk_path, &xor_bytes)
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

                            current_4bytes[3] = xor_i.byte(blk_bytes[i], &xor_bytes);
                            i += 1;

                            if current_4bytes == MAGIC_BYTES {
                                break;
                            }
                        }

                        let deser_len = u32::from_le_bytes(
                            xor_i
                                .bytes(&mut blk_bytes[i..(i + 4)], &xor_bytes)
                                .try_into()
                                .unwrap(),
                        );
                        let len = deser_len as usize;
                        i += 4;

                        let block_bytes = (blk_bytes[i..(i + len)]).to_vec();

                        if send_bytes
                            .send((
                                BlockPosition {
                                    blk_index,
                                    offset: i,
                                    len: deser_len,
                                },
                                AnyBlock::Raw(block_bytes),
                                xor_i,
                            ))
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

            let drain_and_send = |bulk: &mut Vec<(BlockPosition, AnyBlock, XORIndex)>| {
                // Using a vec and sending after to not end up with stuck threads in par iter
                mem::take(bulk)
                    .into_par_iter()
                    .try_for_each(|(position, any_block, xor_i)| {
                        if let Ok(AnyBlock::Decoded(block)) =
                            any_block.decode(position, rpc, xor_i, &xor_bytes, start, end)
                            && send_block.send(block).is_err()
                        {
                            return ControlFlow::Break(());
                        }

                        ControlFlow::Continue(())
                    })
            };

            recv_bytes.iter().try_for_each(|tuple| {
                bulk.push(tuple);

                if bulk.len() < BOUND_CAP / 2 {
                    return ControlFlow::Continue(());
                }

                // Sending in bulk to not lock threads in standby
                drain_and_send(&mut bulk)
            })?;

            drain_and_send(&mut bulk)
        });

        thread::spawn(move || {
            let mut current_height = start.unwrap_or_default();

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
                        send_ordered.send(block).unwrap();

                        current_height.increment();
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
        xor_bytes: &XORBytes,
    ) -> Result<u16> {
        let Some(target_start) = target_start else {
            return Ok(0);
        };

        // If start is a very recent block we only look back X blk file before the last
        if let Ok(count) = self.rpc.get_block_count()
            && (count as u32).saturating_sub(*target_start) <= 3
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

    fn get_first_block_height(&self, blk_path: &PathBuf, xor_bytes: &XORBytes) -> Result<Height> {
        let mut file = File::open(blk_path)?;
        let mut xor_i = XORIndex::default();
        let mut current_4bytes = [0; 4];
        let mut byte_buffer = [0u8; 1];

        loop {
            if file.read_exact(&mut byte_buffer).is_err() {
                return Err("No magic bytes found".into());
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

        let height = self.rpc.get_block_info(&header.block_hash())?.height as u32;

        Ok(Height::new(height))
    }
}
