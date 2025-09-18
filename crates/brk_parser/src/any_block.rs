use bitcoin::{Transaction, VarInt, block::Header, consensus::Decodable, io::Cursor};
use bitcoincore_rpc::RpcApi;
use brk_error::Result;
use brk_structs::{BlkPosition, Block, Height, ParsedBlock};

use crate::{XORBytes, XORIndex};

pub enum AnyBlock {
    Raw(Vec<u8>),
    Decoded(ParsedBlock),
    Skipped,
}

impl AnyBlock {
    pub fn decode(
        self,
        position: BlkPosition,
        rpc: &'static bitcoincore_rpc::Client,
        mut xor_i: XORIndex,
        xor_bytes: XORBytes,
        start: Option<Height>,
        end: Option<Height>,
    ) -> Result<Self> {
        let mut bytes = match self {
            AnyBlock::Raw(bytes) => bytes,
            _ => unreachable!(),
        };

        xor_i.bytes(bytes.as_mut_slice(), xor_bytes);

        let mut cursor = Cursor::new(bytes);

        let header = Header::consensus_decode(&mut cursor)?;

        let hash = header.block_hash();

        let tx_count = VarInt::consensus_decode(&mut cursor)?.0;

        let Ok(block_header_result) = rpc.get_block_header_info(&hash) else {
            return Ok(Self::Skipped);
        };

        let height = Height::from(block_header_result.height);

        if let Some(start) = start
            && start > height
        {
            return Ok(Self::Skipped);
        } else if let Some(end) = end
            && end < height
        {
            return Ok(Self::Skipped);
        } else if block_header_result.confirmations <= 0 {
            return Ok(Self::Skipped);
        }

        let mut txdata = Vec::with_capacity(tx_count as usize);
        let mut tx_positions = Vec::with_capacity(tx_count as usize);
        for _ in 0..tx_count {
            let tx_position = BlkPosition::new(position.blk_index(), cursor.position() as u32);
            tx_positions.push(tx_position);
            let tx = Transaction::consensus_decode(&mut cursor)?;
            txdata.push(tx);
        }

        let block = bitcoin::Block { header, txdata };
        let block = Block::from((height, hash, block));
        let block = ParsedBlock::from((block, position, tx_positions));

        Ok(Self::Decoded(block))
    }
}
