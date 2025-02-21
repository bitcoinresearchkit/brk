use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fs::{self},
    ops::ControlFlow,
    path::Path,
    thread,
};

use bitcoin::{
    consensus::{Decodable, ReadExt},
    io::{Cursor, Read},
    Block, BlockHash,
};
use bitcoincore_rpc::RpcApi;
use blk_index_to_blk_path::*;
use blk_recap::BlkRecap;
use crossbeam::channel::{bounded, Receiver};
use rayon::prelude::*;

pub use bitcoin;
pub use bitcoincore_rpc as rpc;

mod blk_index_to_blk_path;
mod blk_index_to_blk_recap;
mod blk_metadata;
mod blk_metadata_and_block;
mod blk_recap;
mod error;
mod height;
mod utils;

use blk_index_to_blk_recap::*;
use blk_metadata::*;
use blk_metadata_and_block::*;
pub use error::*;
pub use height::*;
use utils::*;

pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 1000;
const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];
const BOUND_CAP: usize = 100;

///
/// Returns a crossbeam channel receiver that receives `(Height, Block, BlockHash)` tuples from an **inclusive** range (`start` and `end`)
///
/// For an example checkout `iterator/main.rs`
///
pub fn new(
    data_dir: &Path,
    start: Option<Height>,
    end: Option<Height>,
    rpc: &'static bitcoincore_rpc::Client,
) -> Receiver<(Height, Block, BlockHash)> {
    let (send_block_reader, recv_block_reader) = bounded(BOUND_CAP);
    let (send_block, recv_block) = bounded(BOUND_CAP);
    let (send_height_block_hash, recv_height_block_hash) = bounded(BOUND_CAP);

    let blk_index_to_blk_path = BlkIndexToBlkPath::scan(data_dir);

    let (mut blk_index_to_blk_recap, blk_index) = BlkIndexToBlkRecap::import(data_dir, &blk_index_to_blk_path, start);

    thread::spawn(move || {
        blk_index_to_blk_path
            .range(blk_index..)
            .try_for_each(move |(blk_index, blk_path)| {
                let blk_index = *blk_index;

                let blk_metadata = BlkMetadata::new(blk_index, blk_path.as_path());

                let blk_bytes = fs::read(blk_path).unwrap();
                let blk_bytes_len = blk_bytes.len() as u64;

                let mut cursor = Cursor::new(blk_bytes.as_slice());

                let mut current_4bytes = [0; 4];

                'parent: loop {
                    if cursor.position() == blk_bytes_len {
                        break;
                    }

                    // Read until we find a valid suite of MAGIC_BYTES
                    loop {
                        current_4bytes.rotate_left(1);

                        if let Ok(byte) = cursor.read_u8() {
                            current_4bytes[3] = byte;
                        } else {
                            break 'parent;
                        }

                        if current_4bytes == MAGIC_BYTES {
                            break;
                        }
                    }

                    let len = cursor.read_u32().unwrap();

                    let mut bytes = vec![0u8; len as usize];

                    cursor.read_exact(&mut bytes).unwrap();

                    if send_block_reader.send((blk_metadata, BlockState::Raw(bytes))).is_err() {
                        return ControlFlow::Break(());
                    }
                }

                ControlFlow::Continue(())
            });
    });

    thread::spawn(move || {
        let mut bulk = vec![];

        let drain_and_send = |bulk: &mut Vec<_>| {
            // Using a vec and sending after to not end up with stuck threads in par iter
            bulk.par_iter_mut().for_each(|(_, block_state)| {
                BlockState::decode(block_state);
            });

            bulk.drain(..).try_for_each(|(blk_metadata, block_state)| {
                let block = match block_state {
                    BlockState::Decoded(block) => block,
                    _ => unreachable!(),
                };

                if send_block.send(BlkIndexAndBlock::new(blk_metadata, block)).is_err() {
                    return ControlFlow::Break(());
                }

                ControlFlow::Continue(())
            })
        };

        recv_block_reader.iter().try_for_each(|tuple| {
            bulk.push(tuple);

            if bulk.len() < BOUND_CAP / 2 {
                return ControlFlow::Continue(());
            }

            // Sending in bulk to not lock threads in standby
            drain_and_send(&mut bulk)
        });

        drain_and_send(&mut bulk)
    });

    thread::spawn(move || {
        let mut current_height = start.unwrap_or_default();

        let mut future_blocks = BTreeMap::default();

        recv_block.iter().try_for_each(|tuple| -> ControlFlow<(), _> {
            let blk_metadata = tuple.blk_metadata;
            let block = tuple.block;
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
            } else {
                dbg!(blk_metadata.index, len);
                panic!()
            }

            let mut opt = if current_height == height {
                Some((block, hash))
            } else {
                if start.map_or(true, |start| start <= height) && end.map_or(true, |end| end >= height) {
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
                send_height_block_hash.send((current_height, block, hash)).unwrap();

                if end == Some(current_height) {
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

enum BlockState {
    Raw(Vec<u8>),
    Decoded(Block),
}

impl BlockState {
    pub fn decode(&mut self) {
        let bytes = match self {
            BlockState::Raw(bytes) => bytes,
            _ => unreachable!(),
        };

        let mut cursor = Cursor::new(bytes);

        let block = Block::consensus_decode(&mut cursor).unwrap();

        *self = BlockState::Decoded(block);
    }
}
