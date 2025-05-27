#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{cmp::Ordering, collections::BTreeMap, fs, ops::ControlFlow, path::PathBuf, thread};

use bitcoin::{Block, BlockHash};
use bitcoincore_rpc::RpcApi;
use blk_index_to_blk_path::*;
use blk_recap::BlkRecap;
use brk_core::Height;
use crossbeam::channel::{Receiver, bounded};
use rayon::prelude::*;

mod blk_index_to_blk_path;
mod blk_index_to_blk_recap;
mod blk_metadata;
mod blk_recap;
mod block_state;
mod error;
mod utils;
mod xor_bytes;
mod xor_index;

use blk_index_to_blk_recap::*;
use blk_metadata::*;
use block_state::*;
pub use error::*;
use utils::*;
use xor_bytes::*;
use xor_index::*;

pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 1000;

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

    pub fn get(&self, height: Height) -> Block {
        self.parse(Some(height), Some(height))
            .iter()
            .next()
            .unwrap()
            .1
    }

    ///
    /// Returns a crossbeam channel receiver that receives `(Height, Block, BlockHash)` tuples from an **inclusive** range (`start` and `end`)
    ///
    /// For an example checkout `./main.rs`
    ///
    pub fn parse(
        &self,
        start: Option<Height>,
        end: Option<Height>,
    ) -> Receiver<(Height, Block, BlockHash)> {
        let blocks_dir = self.blocks_dir.as_path();
        let rpc = self.rpc;

        let (send_bytes, recv_bytes) = bounded(BOUND_CAP);
        let (send_block, recv_block) = bounded(BOUND_CAP);
        let (send_height_block_hash, recv_height_block_hash) = bounded(BOUND_CAP);

        let blk_index_to_blk_path = BlkIndexToBlkPath::scan(blocks_dir);

        let (mut blk_index_to_blk_recap, blk_index) =
            BlkIndexToBlkRecap::import(blocks_dir, &blk_index_to_blk_path, start);

        let xor_bytes = XORBytes::from(blocks_dir);

        thread::spawn(move || {
            let xor_bytes = xor_bytes;

            let _ = blk_index_to_blk_path.range(blk_index..).try_for_each(
                move |(blk_index, blk_path)| {
                    let mut xor_i = XORIndex::default();

                    let blk_index = *blk_index;

                    let blk_metadata = BlkMetadata::new(blk_index, blk_path.as_path());

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

                        let len = u32::from_le_bytes(
                            xor_i
                                .bytes(&mut blk_bytes[i..(i + 4)], &xor_bytes)
                                .try_into()
                                .unwrap(),
                        ) as usize;
                        i += 4;

                        let block_bytes = (blk_bytes[i..(i + len)]).to_vec();

                        if send_bytes
                            .send((blk_metadata, BlockState::Raw(block_bytes), xor_i))
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

            let drain_and_send = |bulk: &mut Vec<_>| {
                // Using a vec and sending after to not end up with stuck threads in par iter
                bulk.par_iter_mut().for_each(|(_, block_state, xor_i)| {
                    BlockState::decode(block_state, xor_i, &xor_bytes);
                });

                bulk.drain(..)
                    .try_for_each(|(blk_metadata, block_state, _)| {
                        let block = match block_state {
                            BlockState::Decoded(block) => block,
                            _ => unreachable!(),
                        };

                        if send_block.send((blk_metadata, block)).is_err() {
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
                .try_for_each(|(blk_metadata, block)| -> ControlFlow<(), _> {
                    let hash = block.block_hash();
                    let header = rpc.get_block_header_info(&hash);

                    if header.is_err() {
                        return ControlFlow::Continue(());
                    }
                    let header = header.unwrap();
                    if header.confirmations <= 0 {
                        return ControlFlow::Continue(());
                    }

                    let height = Height::from(header.height);

                    let len = blk_index_to_blk_recap.tree.len();
                    if blk_metadata.index == len as u16 || blk_metadata.index + 1 == len as u16 {
                        match (len as u16).cmp(&blk_metadata.index) {
                            Ordering::Equal => {
                                if len % 21 == 0 {
                                    blk_index_to_blk_recap.export();
                                }
                            }
                            Ordering::Less => panic!(),
                            Ordering::Greater => {}
                        }

                        blk_index_to_blk_recap
                            .tree
                            .entry(blk_metadata.index)
                            .and_modify(|recap| {
                                if recap.max_height < height {
                                    recap.max_height = height;
                                }
                            })
                            .or_insert(BlkRecap {
                                max_height: height,
                                modified_time: blk_metadata.modified_time,
                            });
                    }

                    let mut opt = if current_height == height {
                        Some((block, hash))
                    } else {
                        if start.is_none_or(|start| start <= height)
                            && end.is_none_or(|end| end >= height)
                        {
                            future_blocks.insert(height, (block, hash));
                        }
                        None
                    };

                    while let Some((block, hash)) = opt.take().or_else(|| {
                        if !future_blocks.is_empty() {
                            future_blocks.remove(&current_height)
                        } else {
                            None
                        }
                    }) {
                        if end.is_some_and(|end| end < current_height) {
                            return ControlFlow::Break(());
                        }

                        send_height_block_hash
                            .send((current_height, block, hash))
                            .unwrap();

                        if end.is_some_and(|end| end == current_height) {
                            return ControlFlow::Break(());
                        }

                        current_height.increment();
                    }

                    ControlFlow::Continue(())
                });

            blk_index_to_blk_recap.export();
        });

        recv_height_block_hash
    }
}
