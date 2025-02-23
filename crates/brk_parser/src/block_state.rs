use bitcoin::{Block, consensus::Decodable, io::Cursor};

use crate::{XORBytes, XORIndex};

pub enum BlockState {
    Raw(Vec<u8>),
    Decoded(Block),
}

impl BlockState {
    pub fn decode(&mut self, xor_i: &mut XORIndex, xor_bytes: &XORBytes) {
        let bytes = match self {
            BlockState::Raw(bytes) => bytes,
            _ => unreachable!(),
        };

        xor_i.bytes(bytes.as_mut_slice(), xor_bytes);

        let mut cursor = Cursor::new(bytes);

        let block = Block::consensus_decode(&mut cursor).unwrap();

        *self = BlockState::Decoded(block);
    }
}
