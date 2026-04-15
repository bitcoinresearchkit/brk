//! Helpers for picking where to start scanning: probe the first
//! block of a file ([`first_block_height`]) and bisect the blk-index
//! map for a target height ([`find_start_blk_index`]).

use std::{fs::File, io::Read, path::Path};

use bitcoin::{block::Header, consensus::Decodable};
use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::Height;
use tracing::warn;

use crate::{
    BlkIndexToBlkPath, OUT_OF_ORDER_FILE_BACKOFF, XORBytes, XORIndex,
    parse::HEADER_LEN, scan::find_magic,
};

const PROBE_BUF_LEN: usize = 4096;

/// Decodes the first block in `blk_path` and resolves its height via
/// RPC. One short read + one RPC.
pub(crate) fn first_block_height(
    client: &Client,
    blk_path: &Path,
    xor_bytes: XORBytes,
) -> Result<Height> {
    let mut file = File::open(blk_path)?;
    let mut buf = [0u8; PROBE_BUF_LEN];
    let n = file.read(&mut buf)?;

    let mut xor_i = XORIndex::default();
    let magic_end = find_magic(&buf[..n], &mut xor_i, xor_bytes)
        .ok_or_else(|| Error::NotFound("No magic bytes found".into()))?;

    // Decode the 4-byte size + 80-byte header in one pass; the size
    // is discarded. Bounds-check first so a corrupt file whose only
    // magic-shaped bytes sit at the end of the probe doesn't index
    // past `n`.
    let header_end = magic_end + 4 + HEADER_LEN;
    if header_end > n {
        warn!(
            "first_block_height: {} has magic-shaped bytes at offset {} but \
             not enough room in the {}-byte probe to decode the header — \
             the file is probably corrupt",
            blk_path.display(),
            magic_end - 4,
            PROBE_BUF_LEN,
        );
        return Err(Error::Parse(format!(
            "blk file probe truncated before header at {}",
            blk_path.display()
        )));
    }
    xor_i.bytes(&mut buf[magic_end..header_end], xor_bytes);

    let header = Header::consensus_decode_from_finite_reader(&mut &buf[magic_end + 4..header_end])?;
    let height = client.get_block_info(&header.block_hash())?.height as u32;

    Ok(Height::new(height))
}

/// Bisects the map for the file whose first block height is ≤
/// `target_start`, then backs off [`OUT_OF_ORDER_FILE_BACKOFF`] files.
/// Always returns a valid blk index — read errors mid-search log and
/// fall through to the backoff (or to 0 if the map is empty).
///
/// On a transient read error we **break** rather than `left = mid + 1`:
/// the height bound at `mid` is unknown, so any further bisection on
/// that side could skip valid lower indices. Falling through to the
/// backoff still gives a safe lower bound.
pub(crate) fn find_start_blk_index(
    client: &Client,
    target_start: Height,
    paths: &BlkIndexToBlkPath,
    xor_bytes: XORBytes,
) -> u16 {
    let entries: Vec<(u16, &Path)> = paths.iter().map(|(&i, p)| (i, p.as_path())).collect();
    if entries.is_empty() {
        return 0;
    }

    let mut left = 0;
    let mut right = entries.len() - 1;
    let mut best_start_idx = 0;

    while left <= right {
        let mid = (left + right) / 2;
        let (blk_index, blk_path) = entries[mid];
        match first_block_height(client, blk_path, xor_bytes) {
            Ok(height) if height <= target_start => {
                best_start_idx = mid;
                left = mid + 1;
            }
            Ok(_) => {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            }
            Err(e) => {
                warn!("find_start_blk_index: read error at blk{blk_index:05}.dat: {e}");
                break;
            }
        }
    }

    let final_idx = best_start_idx.saturating_sub(OUT_OF_ORDER_FILE_BACKOFF);
    entries[final_idx].0
}
