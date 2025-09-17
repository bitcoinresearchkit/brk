use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    io::Cursor,
    ops::Deref,
};

use bitcoin::{block::Header, consensus::Decodable};
use brk_error::Result;

use crate::BlockPosition;

use super::{BlockHash, Height};

#[derive(Debug)]
pub struct Block {
    height: Height,
    hash: BlockHash,
    block: bitcoin::Block,
}

impl Block {
    pub fn height(&self) -> Height {
        self.height
    }

    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    pub fn parse(block_data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(block_data);

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
        })
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

impl From<(Height, bitcoin::Block)> for Block {
    fn from((height, block): (Height, bitcoin::Block)) -> Self {
        Self::from((height, block.block_hash(), block))
    }
}

impl From<(Height, bitcoin::BlockHash, bitcoin::Block)> for Block {
    fn from((height, hash, block): (Height, bitcoin::BlockHash, bitcoin::Block)) -> Self {
        Self {
            height,
            hash: hash.into(),
            block,
        }
    }
}

impl Deref for Block {
    type Target = bitcoin::Block;
    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

#[derive(Debug)]
pub struct ParsedBlock {
    block: Block,
    position: BlockPosition,
    tx_positions: BTreeMap<usize, usize>, // txid -> offset
}

impl From<(Block, BlockPosition)> for ParsedBlock {
    fn from((block, position): (Block, BlockPosition)) -> Self {
        Self {
            block,
            position,
            tx_positions: BTreeMap::default(),
        }
    }
}

impl ParsedBlock {
    pub fn position(&self) -> &BlockPosition {
        &self.position
    }

    pub fn tx_positions(&self) -> &BTreeMap<usize, usize> {
        &self.tx_positions
    }

    // pub fn parse(block_data: &[u8], file_offset: usize) -> Result<Self> {
    //     let mut cursor = std::io::Cursor::new(block_data);

    //     let header = Header::consensus_decode(&mut cursor)?;

    //     // Parse transactions with positions
    //     let tx_count = bitcoin::VarInt::consensus_decode(&mut cursor)?.0 as usize;
    //     let mut transactions = Vec::with_capacity(tx_count);
    //     let mut tx_positions = HashMap::with_capacity(tx_count);

    //     for _ in 0..tx_count {
    //         let start = cursor.position() as usize;
    //         let tx = bitcoin::Transaction::consensus_decode(&mut cursor)?;

    //         tx_positions.insert(tx.compute_txid(), start);
    //         transactions.push(tx);
    //     }

    //     let block = bitcoin::Block {
    //         header,
    //         txdata: transactions,
    //     };

    //     // block.bip34_block_height()

    //     let hash = block.block_hash();

    //     Ok(Block {
    //         block,
    //         hash: hash.into(),
    //         height: Height::ZERO,
    //         tx_positions,
    //         block_start_offset: file_offset,
    //     })
    // }

    // pub fn get_absolute_position(&self, txid: &Txid) -> Option<usize> {
    //     self.tx_positions
    //         .get(txid)
    //         .map(|offset| self.block_start_offset + offset)
    // }
}

impl Deref for ParsedBlock {
    type Target = Block;
    fn deref(&self) -> &Self::Target {
        &self.block
    }
}
