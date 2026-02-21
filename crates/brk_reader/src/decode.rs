use bitcoin::{Transaction, VarInt, block::Header, consensus::Decodable, io::Cursor};
use brk_error::Result;
use brk_rpc::Client;
use brk_types::{BlkMetadata, Block, Height, ReadBlock};

use crate::{XORBytes, XORIndex};

/// Margin for timestamp non-monotonicity
const TIMESTAMP_MARGIN: u32 = 3 * 3600;

#[allow(clippy::too_many_arguments)]
pub fn decode_block(
    mut bytes: Vec<u8>,
    metadata: BlkMetadata,
    client: &Client,
    mut xor_i: XORIndex,
    xor_bytes: XORBytes,
    start: Option<Height>,
    end: Option<Height>,
    start_time: u32,
    end_time: u32,
) -> Result<Option<ReadBlock>> {
    xor_i.bytes(bytes.as_mut_slice(), xor_bytes);

    let mut cursor = Cursor::new(bytes);

    let header = Header::consensus_decode(&mut cursor)?;

    // Skip blocks clearly outside the target range using header timestamp
    if header.time < start_time.saturating_sub(TIMESTAMP_MARGIN)
        || (end_time > 0 && header.time > end_time.saturating_add(TIMESTAMP_MARGIN))
    {
        return Ok(None);
    }

    let hash = header.block_hash();

    let Ok(block_header_result) = client.get_block_header_info(&hash) else {
        return Ok(None);
    };

    let height = Height::from(block_header_result.height);

    if start.is_some_and(|s| s > height) || end.is_some_and(|e| e < height) {
        return Ok(None);
    }
    if block_header_result.confirmations <= 0 {
        return Ok(None);
    }

    let tx_count = VarInt::consensus_decode(&mut cursor)?.0 as usize;

    let mut txdata = Vec::with_capacity(tx_count);
    let mut tx_metadata = Vec::with_capacity(tx_count);
    let mut tx_offsets = Vec::with_capacity(tx_count);
    for _ in 0..tx_count {
        let offset = cursor.position() as u32;
        tx_offsets.push(offset);
        let position = metadata.position() + offset;
        let tx = Transaction::consensus_decode(&mut cursor)?;
        txdata.push(tx);
        let len = cursor.position() as u32 - offset;
        tx_metadata.push(BlkMetadata::new(position, len));
    }

    let block_bytes = cursor.into_inner();

    let block = bitcoin::Block { header, txdata };
    let mut block = Block::from((height, hash, block));
    block.set_raw_data(block_bytes, tx_offsets);

    Ok(Some(ReadBlock::from((block, metadata, tx_metadata))))
}
