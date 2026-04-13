//! Pure block parsing — XOR decoding, header and body decode.
//!
//! Split into a cheap header peek and a full body parse so the scan
//! loop can reject non-canonical blocks without copying them. No RPC,
//! no threading, no state.

use std::io::Cursor;

use bitcoin::{Transaction, VarInt, block::Header, consensus::Decodable};
use brk_error::{Error, Result};
use brk_types::{BlkMetadata, Block, BlockHash, Height, ReadBlock};

use crate::{XORBytes, XORIndex, canonical::CanonicalRange};

const HEADER_LEN: usize = 80;

/// Returns the canonical offset of `bytes` if its header hashes to a
/// known canonical block, otherwise `None`. Does not allocate and does
/// not mutate `bytes`: the header is copied onto a stack buffer and
/// XOR-decoded there so an orphan short-circuits cleanly and a
/// canonical hit can still be cloned out intact.
pub fn peek_canonical_offset(
    bytes: &[u8],
    mut xor_state: XORIndex,
    xor_bytes: XORBytes,
    canonical: &CanonicalRange,
) -> Option<u32> {
    if bytes.len() < HEADER_LEN {
        return None;
    }
    let mut header_buf = [0u8; HEADER_LEN];
    header_buf.copy_from_slice(&bytes[..HEADER_LEN]);
    xor_state.bytes(&mut header_buf, xor_bytes);
    let header = Header::consensus_decode(&mut &header_buf[..]).ok()?;
    canonical.offset_of(&BlockHash::from(header.block_hash()))
}

/// Full XOR-decode + parse for a block that has already been confirmed
/// canonical by `peek_canonical_offset`. Takes owned `bytes` so it can
/// mutate them in place and hand them to the resulting `ReadBlock`.
pub fn parse_canonical_body(
    mut bytes: Vec<u8>,
    metadata: BlkMetadata,
    mut xor_state: XORIndex,
    xor_bytes: XORBytes,
    height: Height,
) -> Result<ReadBlock> {
    if bytes.len() < HEADER_LEN {
        return Err(Error::Internal("Block bytes shorter than header"));
    }

    xor_state.bytes(&mut bytes, xor_bytes);
    let mut cursor = Cursor::new(bytes);
    let header = Header::consensus_decode(&mut cursor)?;
    let bitcoin_hash = header.block_hash();
    let tx_count = VarInt::consensus_decode(&mut cursor)?.0 as usize;
    let mut txdata = Vec::with_capacity(tx_count);
    let mut tx_metadata = Vec::with_capacity(tx_count);
    let mut tx_offsets = Vec::with_capacity(tx_count);
    for _ in 0..tx_count {
        let tx_start = cursor.position() as u32;
        tx_offsets.push(tx_start);
        let tx = Transaction::consensus_decode(&mut cursor)?;
        let tx_len = cursor.position() as u32 - tx_start;
        txdata.push(tx);
        tx_metadata.push(BlkMetadata::new(metadata.position() + tx_start, tx_len));
    }

    let raw_bytes = cursor.into_inner();
    let mut block = Block::from((height, bitcoin_hash, bitcoin::Block { header, txdata }));
    block.set_raw_data(raw_bytes, tx_offsets);
    Ok(ReadBlock::from((block, metadata, tx_metadata)))
}
