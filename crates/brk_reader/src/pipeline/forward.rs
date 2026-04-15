//! Forward pipeline: 1 reader thread + N scoped parser threads.
//! Reader walks blk files from a bisection lower bound, peeks each
//! block's header against `CanonicalRange`, and ships hits to the
//! parser pool. Parsers decode bodies in parallel and emit in-order
//! through `ReorderState`.

use std::{fs, ops::ControlFlow, sync::OnceLock, thread};

use bitcoin::block::Header;
use brk_error::{Error, Result};
use brk_types::{BlkMetadata, Height, ReadBlock};
use crossbeam::channel::{Receiver, Sender, bounded};
use parking_lot::Mutex;
use tracing::error;

use crate::{
    BlkIndexToBlkPath, BlockHash, XORBytes, XORIndex,
    canonical::CanonicalRange,
    parse::{parse_canonical_body, peek_canonical},
    pipeline::{CHANNEL_CAPACITY, reorder::ReorderState},
    scan::scan_bytes,
};

/// Reader→parser channel message. `header` was decoded during the
/// peek and is reused so the parser doesn't redo it.
struct ScannedBlock {
    metadata: BlkMetadata,
    bytes: Vec<u8>,
    xor_state: XORIndex,
    canonical_offset: u32,
    header: Header,
}

/// Single shared signal carrying both the cancel flag and (when set
/// to `Failed`) the first parse error. `stop.get().is_some()` is the
/// reader's cheap "should I stop" check.
enum Stop {
    Done,
    Failed(Error),
}

pub(super) fn pipeline_forward(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    anchor: Option<BlockHash>,
    send: &Sender<Result<ReadBlock>>,
    parser_threads: usize,
) -> Result<()> {
    let (parser_send, parser_recv) = bounded::<ScannedBlock>(CHANNEL_CAPACITY);
    let reorder = Mutex::new(ReorderState::new(send.clone(), anchor));
    let stop: OnceLock<Stop> = OnceLock::new();

    thread::scope(|scope| {
        for _ in 0..parser_threads {
            let parser_recv = parser_recv.clone();
            scope.spawn(|| parser_loop(parser_recv, &reorder, &stop, canonical, xor_bytes));
        }
        // Every parser owns its own clone; ours would otherwise leak
        // a dangling receiver.
        drop(parser_recv);

        let read_result =
            read_and_dispatch(paths, first_blk_index, xor_bytes, canonical, &parser_send, &stop);
        // End-of-input signal so parser `for` loops exit and the
        // scope can join.
        drop(parser_send);
        read_result
    })?;

    if let Some(Stop::Failed(e)) = stop.into_inner() {
        return Err(e);
    }
    reorder.into_inner().finalize(canonical.len())
}

/// Per-thread parser body: drain `parser_recv`, decode each block,
/// emit through `reorder`. Stops on `stop`.
fn parser_loop(
    parser_recv: Receiver<ScannedBlock>,
    reorder: &Mutex<ReorderState>,
    stop: &OnceLock<Stop>,
    canonical: &CanonicalRange,
    xor_bytes: XORBytes,
) {
    for ScannedBlock {
        metadata,
        bytes,
        xor_state,
        canonical_offset,
        header,
    } in parser_recv
    {
        if stop.get().is_some() {
            continue;
        }
        let height = Height::from(*canonical.start + canonical_offset);
        let block =
            match parse_canonical_body(bytes, metadata, xor_state, xor_bytes, height, header) {
                Ok(block) => block,
                Err(e) => {
                    error!("parse_canonical_body failed at height {height}: {e}");
                    let _ = stop.set(Stop::Failed(e));
                    continue;
                }
            };
        let pipeline_finished = {
            let mut state = reorder.lock();
            !state.try_emit(canonical_offset, block)
                || state.next_offset as usize >= canonical.len()
        };
        if pipeline_finished {
            let _ = stop.set(Stop::Done);
        }
    }
}

/// `peek_canonical` filters orphans **before** the block bytes are
/// cloned, so a sparse catchup avoids allocating for the ~99% of
/// blocks outside the window.
fn read_and_dispatch(
    paths: &BlkIndexToBlkPath,
    first_blk_index: u16,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
    parser_send: &Sender<ScannedBlock>,
    stop: &OnceLock<Stop>,
) -> Result<()> {
    for (&blk_index, blk_path) in paths.range(first_blk_index..) {
        if stop.get().is_some() {
            return Ok(());
        }
        let mut bytes = fs::read(blk_path)?;
        scan_bytes(
            &mut bytes,
            blk_index,
            0,
            xor_bytes,
            |metadata, block_bytes, xor_state| {
                if stop.get().is_some() {
                    return ControlFlow::Break(());
                }
                let Some((canonical_offset, header)) =
                    peek_canonical(block_bytes, xor_state, xor_bytes, canonical)
                else {
                    return ControlFlow::Continue(());
                };
                let scanned = ScannedBlock {
                    metadata,
                    bytes: block_bytes.to_vec(),
                    xor_state,
                    canonical_offset,
                    header,
                };
                if parser_send.send(scanned).is_err() {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(())
                }
            },
        );
    }
    Ok(())
}
