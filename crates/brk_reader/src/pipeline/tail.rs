//! Tail pipeline: single-threaded reverse scan of the newest blk
//! files until every canonical hash is matched, then forward-emit
//! with an inline chain check. Avoids the forward pipeline's
//! bisection + out-of-order backoff (~2.7 GB of reads) for any
//! tip-clustered catchup.

use std::{fs, ops::ControlFlow};

use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::{Height, ReadBlock};
use crossbeam::channel::Sender;

use crate::{
    BlkIndexToBlkPath, BlockHash, OUT_OF_ORDER_FILE_BACKOFF, XORBytes, bisect,
    canonical::CanonicalRange,
    parse::{parse_canonical_body, peek_canonical},
    scan::scan_bytes,
};

pub(super) fn pipeline_tail(
    client: &Client,
    paths: &BlkIndexToBlkPath,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    anchor: Option<BlockHash>,
    send: &Sender<Result<ReadBlock>>,
) -> Result<()> {
    let mut slots: Vec<Option<ReadBlock>> = (0..canonical.len()).map(|_| None).collect();
    let mut remaining = canonical.len();
    let mut parse_failure: Option<Error> = None;
    // Bailout streak: gives up after OUT_OF_ORDER_FILE_BACKOFF
    // consecutive files below the canonical window so a permanent
    // miss doesn't scan the entire chain in reverse.
    let mut below_floor_streak: usize = 0;

    for (&blk_index, path) in paths.iter().rev() {
        // If this file's first block is below the lowest still-missing
        // canonical height, we've walked past the window.
        if let Some(missing_idx) = slots.iter().position(Option::is_none)
            && let Ok(first_height) = bisect::first_block_height(client, path, xor_bytes)
        {
            let lowest_missing = Height::from(*canonical.start + missing_idx as u32);
            if first_height < lowest_missing {
                below_floor_streak += 1;
                if below_floor_streak >= OUT_OF_ORDER_FILE_BACKOFF {
                    return Err(Error::Internal(
                        "tail pipeline: walked past the canonical window without finding all blocks",
                    ));
                }
            } else {
                below_floor_streak = 0;
            }
        }

        let mut bytes = fs::read(path)?;
        scan_bytes(
            &mut bytes,
            blk_index,
            xor_bytes,
            |metadata, block_bytes, xor_state| {
                let Some((offset, header)) =
                    peek_canonical(block_bytes, xor_state, xor_bytes, canonical)
                else {
                    return ControlFlow::Continue(());
                };
                if slots[offset as usize].is_some() {
                    return ControlFlow::Continue(());
                }
                let height = Height::from(*canonical.start + offset);
                match parse_canonical_body(
                    block_bytes.to_vec(),
                    metadata,
                    xor_state,
                    xor_bytes,
                    height,
                    header,
                ) {
                    Ok(block) => {
                        slots[offset as usize] = Some(block);
                        remaining -= 1;
                    }
                    Err(e) => {
                        parse_failure = Some(e);
                        return ControlFlow::Break(());
                    }
                }
                if remaining == 0 {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            },
        );

        if let Some(e) = parse_failure {
            return Err(e);
        }
        if remaining == 0 {
            break;
        }
    }

    if remaining > 0 {
        return Err(Error::Internal(
            "tail pipeline: blk files missing canonical blocks",
        ));
    }

    // Inline chain check; ReorderState would be 130 lines of
    // machinery for the single-threaded path.
    let mut last_hash: Option<BlockHash> = anchor;
    for slot in slots {
        let block = slot.expect("tail pipeline left a slot empty after `remaining == 0`");
        if let Some(prev) = &last_hash {
            let actual_prev = BlockHash::from(block.header.prev_blockhash);
            if actual_prev != *prev {
                return Err(Error::Internal(
                    "tail pipeline: canonical batch stitched across a reorg",
                ));
            }
        }
        last_hash = Some(block.hash().clone());
        if send.send(Ok(block)).is_err() {
            return Ok(()); // consumer dropped — clean exit
        }
    }
    Ok(())
}
