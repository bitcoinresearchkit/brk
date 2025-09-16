use std::{borrow::Cow, collections::HashMap, ops::Deref};

use bitcoin::{Txid, block::Header, consensus::Decodable};
use brk_error::Result;

use super::{BlockHash, Height};

#[derive(Debug)]
pub struct Block {
    pub block: bitcoin::Block,
    pub hash: BlockHash,
    pub height: Height,
    pub tx_positions: HashMap<Txid, usize>, // txid -> offset
    pub block_start_offset: usize,
}

impl Block {
    pub fn parse(block_data: &[u8], file_offset: usize) -> Result<Self> {
        let mut cursor = std::io::Cursor::new(block_data);

        let header = Header::consensus_decode(&mut cursor)?;

        // Parse transactions with positions
        let tx_count = bitcoin::VarInt::consensus_decode(&mut cursor)?.0 as usize;
        let mut transactions = Vec::with_capacity(tx_count);
        let mut tx_positions = HashMap::with_capacity(tx_count);

        for _ in 0..tx_count {
            let start = cursor.position() as usize;
            let tx = bitcoin::Transaction::consensus_decode(&mut cursor)?;

            tx_positions.insert(tx.compute_txid(), start);
            transactions.push(tx);
        }

        let block = bitcoin::Block {
            header,
            txdata: transactions,
        };

        // block.bip34_block_height()

        let hash = block.block_hash();

        Ok(Block {
            block,
            hash: hash.into(),
            height: Height::ZERO,
            tx_positions,
            block_start_offset: file_offset,
        })
    }

    pub fn get_absolute_position(&self, txid: &Txid) -> Option<usize> {
        self.tx_positions
            .get(txid)
            .map(|offset| self.block_start_offset + offset)
    }

    pub fn coinbase_tag(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(
            self.txdata
                .first()
                .and_then(|tx| tx.input.first())
                .unwrap()
                .script_sig
                .as_bytes(),
        )
    }
}

impl Deref for Block {
    type Target = bitcoin::Block;
    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

pub trait BlockExtended {
    fn coinbase_tag(&self) -> Cow<'_, str>;
}

impl BlockExtended for bitcoin::Block {
    fn coinbase_tag(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(
            self.txdata
                .first()
                .and_then(|tx| tx.input.first())
                .unwrap()
                .script_sig
                .as_bytes(),
        )
    }
}
