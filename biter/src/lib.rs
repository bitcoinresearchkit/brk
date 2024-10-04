use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::{self},
    ops::ControlFlow,
    thread,
};

use bitcoin::{
    consensus::{Decodable, ReadExt},
    hashes::Hash,
    io::{Cursor, Read},
    Block, BlockHash,
};
use bitcoincore_rpc::RpcApi;
use crossbeam::channel::{bounded, Receiver};
use rayon::prelude::*;

pub use bitcoin;
pub use bitcoincore_rpc;

mod blk_index_to_blk_recap;
mod blk_metadata;
mod blk_metadata_and_block;
mod blk_recap;
mod utils;

use blk_index_to_blk_recap::*;
use blk_metadata::*;
use blk_metadata_and_block::*;
use utils::*;

pub const NUMBER_OF_UNSAFE_BLOCKS: usize = 100;
const MAGIC_BYTES: [u8; 4] = [249, 190, 180, 217];
const BOUND_CAP: usize = 210;

enum BlockState {
    Raw(Vec<u8>),
    Decoded(Block),
}

///
/// Returns a crossbeam channel receiver that receives `(usize, Block, BlockHash)` tuples (with `usize` being the height) in sequential order.
///
/// # Arguments
///
/// * `data_dir` - Path to the Bitcoin data directory
/// * `export_dir` - Path to the export directory where a mini blk indexer will be exported
/// * `start` - Inclusive starting height of the blocks received, `None` for 0
/// * `end` - Inclusive ending height of the blocks received, `None` for the last one
/// * `rpc` - RPC client to filter out forks
///
/// # Example
///
/// ```rust
/// use bitcoincore_rpc::{Auth, Client};
///
/// fn main() {
///     let i = std::time::Instant::now();
///
///     let url = "http://localhost:8332";
///     let auth = Auth::UserPass("satoshi".to_string(), "nakamoto".to_string());
///     let rpc = Client::new(url, auth).unwrap();
///
///     let data_dir = "../../bitcoin";
///     let export_dir = "./target";
///     let start = Some(850_000);
///     let end = None;
///
///     biter::new(data_dir, export_dir, start, end, rpc)
///         .iter()
///         .for_each(|(height, _block, hash)| {
///             println!("{height}: {hash}");
///         });
///
///     dbg!(i.elapsed());
///}
/// ```
///
pub fn new(
    data_dir: &str,
    export_dir: &str,
    start: Option<usize>,
    end: Option<usize>,
    rpc: bitcoincore_rpc::Client,
) -> Receiver<(usize, Block, BlockHash)> {
    let (send_block_reader, recv_block_reader) = bounded(BOUND_CAP);
    let (send_block, recv_block) = bounded(BOUND_CAP);
    let (send_height_block_hash, recv_height_block_hash) = bounded(BOUND_CAP);

    let blocks_dir = scan_blocks_dir(data_dir);

    let mut blk_index_to_blk_recap = BlkIndexToBlkRecap::import(&blocks_dir, export_dir);

    let start_recap = blk_index_to_blk_recap.get_start_recap(start);
    let starting_blk_index = start_recap.as_ref().map_or(0, |(index, _)| *index);

    thread::spawn(move || {
        blocks_dir
            .into_iter()
            .filter(|(blk_index, _)| blk_index >= &starting_blk_index)
            .try_for_each(move |(blk_index, blk_path)| {
                let blk_metadata = BlkMetadata::new(blk_index, &blk_path);

                let blk_bytes = fs::read(&blk_path).unwrap();
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

                    let block_size = cursor.read_u32().unwrap();

                    let mut raw_block = vec![0u8; block_size as usize];

                    cursor.read_exact(&mut raw_block).unwrap();

                    if send_block_reader
                        .send((blk_metadata, BlockState::Raw(raw_block)))
                        .is_err()
                    {
                        return ControlFlow::Break(());
                    }
                }

                ControlFlow::Continue(())
            })
    });

    // thread::spawn(move || {
    //     recv_block_reader.iter().par_bridge().try_for_each(
    //         move |(blk_metadata, mut block_state)| {
    //             let raw_block = match block_state {
    //                 BlockState::Raw(vec) => vec,
    //                 _ => unreachable!(),
    //             };

    //             let mut cursor = Cursor::new(raw_block);

    //             block_state = BlockState::Decoded(Block::consensus_decode(&mut cursor).unwrap());

    //             if send_block
    //                 .send(BlkMetadataAndBlock::new(
    //                     blk_metadata,
    //                     match block_state {
    //                         BlockState::Decoded(block) => block,
    //                         _ => unreachable!(),
    //                     },
    //                 ))
    //                 .is_err()
    //             {
    //                 return ControlFlow::Break(());
    //             }

    //             ControlFlow::Continue(())
    //         },
    //     );
    // });

    // Can't use the previous code because .send() blocks all the threads if full
    // And other .par_iter() are also stuck because of that
    thread::spawn(move || {
        let mut bulk = vec![];

        let drain_and_send = |bulk: &mut Vec<_>| {
            // Using a vec and sending after to not end up with stuck threads in par iter
            bulk.par_iter_mut().for_each(|(_, block_state)| {
                let raw_block = match block_state {
                    BlockState::Raw(vec) => vec,
                    _ => unreachable!(),
                };

                let mut cursor = Cursor::new(raw_block);

                *block_state = BlockState::Decoded(Block::consensus_decode(&mut cursor).unwrap());
            });

            bulk.drain(..).try_for_each(|(blk_metadata, block_state)| {
                let block = match block_state {
                    BlockState::Decoded(block) => block,
                    _ => unreachable!(),
                };

                if send_block
                    .send(BlkMetadataAndBlock::new(blk_metadata, block))
                    .is_err()
                {
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

            drain_and_send(&mut bulk)
        });

        drain_and_send(&mut bulk)
    });

    // Tokio version: 1022s
    // Slighlty slower than rayon version
    // thread::spawn(move || {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     let _guard = rt.enter();

    //     let mut tasks = VecDeque::with_capacity(BOUND);

    //     recv_block_reader
    //         .iter()
    //         .try_for_each(move |(blk_metadata, block_state)| {
    //             let raw_block = match block_state {
    //                 BlockState::Raw(vec) => vec,
    //                 _ => unreachable!(),
    //             };

    //             tasks.push_back(tokio::task::spawn(async move {
    //                 let block = Block::consensus_decode(&mut Cursor::new(raw_block)).unwrap();

    //                 (blk_metadata, block)
    //             }));

    //             while tasks.len() > BOUND {
    //                 let (blk_metadata, block) = rt.block_on(tasks.pop_front().unwrap()).unwrap();

    //                 if send_block
    //                     .send(BlkMetadataAndBlock::new(blk_metadata, block))
    //                     .is_err()
    //                 {
    //                     return ControlFlow::Break(());
    //                 }
    //             }

    //             ControlFlow::Continue(())
    //         });
    //
    // todo!("Send the rest")
    // });

    thread::spawn(move || {
        let mut height = start_recap.map_or(0, |(_, recap)| recap.height());

        let mut future_blocks = BTreeMap::default();
        let mut recent_chain: VecDeque<(BlockHash, BlkMetadataAndBlock)> = VecDeque::default();
        let mut recent_hashes: BTreeSet<BlockHash> = BTreeSet::default();

        let mut prev_hash =
            start_recap.map_or_else(BlockHash::all_zeros, |(_, recap)| *recap.prev_hash());

        let mut prepare_and_send = |(hash, tuple): (BlockHash, BlkMetadataAndBlock)| {
            blk_index_to_blk_recap.update(&tuple, height);

            if start.map_or(true, |start| start <= height) {
                send_height_block_hash
                    .send((height, tuple.block, hash))
                    .unwrap();
            }

            if end.map_or(false, |end| height == end) {
                return ControlFlow::Break(());
            }

            height += 1;

            ControlFlow::Continue(())
        };

        let mut update_tip = |prev_hash: &mut BlockHash,
                              recent_hashes: &mut BTreeSet<BlockHash>,
                              recent_chain: &mut VecDeque<(BlockHash, BlkMetadataAndBlock)>,
                              future_blocks: &mut BTreeMap<BlockHash, BlkMetadataAndBlock>,
                              tuple: BlkMetadataAndBlock| {
            let mut tuple = Some(tuple);

            while let Some(tuple) = tuple.take().or_else(|| future_blocks.remove(prev_hash)) {
                let hash = tuple.block.block_hash();

                *prev_hash = hash;
                recent_hashes.insert(hash);
                recent_chain.push_back((hash, tuple));
            }

            while recent_chain.len() > NUMBER_OF_UNSAFE_BLOCKS {
                let (hash, tuple) = recent_chain.pop_front().unwrap();

                recent_hashes.remove(&hash);

                if prepare_and_send((hash, tuple)).is_break() {
                    return ControlFlow::Break(());
                }
            }

            ControlFlow::Continue(())
        };

        let flow = recv_block.iter().try_for_each(|tuple| {
            // block isn't next after current tip
            if prev_hash != tuple.block.header.prev_blockhash {
                let is_block_active =
                    |hash| rpc.get_block_header_info(hash).unwrap().confirmations > 0;

                // block prev has already been processed
                if recent_hashes.contains(&tuple.block.header.prev_blockhash) {
                    let hash = tuple.block.block_hash();

                    if is_block_active(&hash) {
                        let prev_index = recent_chain
                            .iter()
                            .position(|(hash, ..)| hash == &tuple.block.header.prev_blockhash)
                            .unwrap();

                        let bad_index_start = prev_index + 1;

                        recent_chain.drain(bad_index_start..).for_each(|(hash, _)| {
                            recent_hashes.remove(&hash);
                        });

                        return update_tip(
                            &mut prev_hash,
                            &mut recent_hashes,
                            &mut recent_chain,
                            &mut future_blocks,
                            tuple,
                        );
                    }
                // Check if there was already a future block with the same prev hash
                } else if let Some(prev_tuple) =
                    future_blocks.insert(tuple.block.header.prev_blockhash, tuple)
                {
                    // If the previous was the active one
                    if is_block_active(&prev_tuple.block.block_hash()) {
                        // Rollback the insert
                        future_blocks.insert(prev_tuple.block.header.prev_blockhash, prev_tuple);
                    }
                }
            } else {
                return update_tip(
                    &mut prev_hash,
                    &mut recent_hashes,
                    &mut recent_chain,
                    &mut future_blocks,
                    tuple,
                );
            }

            ControlFlow::Continue(())
        });

        if flow.is_continue() {
            // Send the last (up to 100) blocks
            recent_chain.into_iter().try_for_each(prepare_and_send);
        }

        blk_index_to_blk_recap.export();
    });

    recv_height_block_hash
}
