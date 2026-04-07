use bitcoin::hex::{DisplayHex, FromHex};
use brk_error::{Error, Result};
use brk_types::{
    BlockHash, Height, MerkleProof, Timestamp, Transaction, TxInIndex, TxIndex, TxOutIndex,
    TxOutspend, TxStatus, Txid, TxidPrefix, Vin, Vout,
};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn transaction(&self, txid: &Txid) -> Result<Transaction> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(txid)
        {
            return Ok(tx_with_hex.tx().clone());
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(txid);
        let indexer = self.indexer();
        let Ok(Some(tx_index)) = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        self.transaction_by_index(tx_index)
    }

    pub fn transaction_status(&self, txid: &Txid) -> Result<TxStatus> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && mempool.get_txs().contains_key(txid)
        {
            return Ok(TxStatus::UNCONFIRMED);
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(txid);
        let indexer = self.indexer();
        let Ok(Some(tx_index)) = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        // Get block info for status
        let height = indexer.vecs.transactions.height.collect_one(tx_index).unwrap();
        let block_hash = indexer.vecs.blocks.blockhash.reader().get(height.to_usize());
        let block_time = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();

        Ok(TxStatus {
            confirmed: true,
            block_height: Some(height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        })
    }

    pub fn transaction_raw(&self, txid: &Txid) -> Result<Vec<u8>> {
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(txid)
        {
            return Vec::from_hex(tx_with_hex.hex())
                .map_err(|_| Error::Parse("Failed to decode mempool tx hex".into()));
        }

        let prefix = TxidPrefix::from(txid);
        let indexer = self.indexer();
        let Ok(Some(tx_index)) = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };
        self.transaction_raw_by_index(tx_index)
    }

    pub fn transaction_hex(&self, txid: &Txid) -> Result<String> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(txid)
        {
            return Ok(tx_with_hex.hex().to_string());
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(txid);
        let indexer = self.indexer();
        let Ok(Some(tx_index)) = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        self.transaction_hex_by_index(tx_index)
    }

    pub fn outspend(&self, txid: &Txid, vout: Vout) -> Result<TxOutspend> {
        if self.mempool().is_some_and(|m| m.get_txs().contains_key(txid)) {
            return Ok(TxOutspend::UNSPENT);
        }
        let (_, first_txout, output_count) = self.resolve_tx_outputs(txid)?;
        if usize::from(vout) >= output_count {
            return Ok(TxOutspend::UNSPENT);
        }
        self.resolve_outspend(first_txout + vout)
    }

    pub fn outspends(&self, txid: &Txid) -> Result<Vec<TxOutspend>> {
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(txid)
        {
            return Ok(vec![TxOutspend::UNSPENT; tx_with_hex.tx().output.len()]);
        }
        let (_, first_txout, output_count) = self.resolve_tx_outputs(txid)?;

        let indexer = self.indexer();
        let txin_index_reader = self.computer().outputs.spent.txin_index.reader();
        let txid_reader = indexer.vecs.transactions.txid.reader();
        let blockhash_reader = indexer.vecs.blocks.blockhash.reader();

        let mut input_tx_cursor = indexer.vecs.inputs.tx_index.cursor();
        let mut first_txin_cursor = indexer.vecs.transactions.first_txin_index.cursor();
        let mut height_cursor = indexer.vecs.transactions.height.cursor();
        let mut block_ts_cursor = indexer.vecs.blocks.timestamp.cursor();

        let mut cached_block: Option<(Height, BlockHash, Timestamp)> = None;

        let mut outspends = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let txin_index = txin_index_reader.get(usize::from(first_txout + Vout::from(i)));

            if txin_index == TxInIndex::UNSPENT {
                outspends.push(TxOutspend::UNSPENT);
                continue;
            }

            let spending_tx_index = input_tx_cursor.get(usize::from(txin_index)).unwrap();
            let spending_first_txin =
                first_txin_cursor.get(spending_tx_index.to_usize()).unwrap();
            let vin = Vin::from(usize::from(txin_index) - usize::from(spending_first_txin));
            let spending_txid = txid_reader.get(spending_tx_index.to_usize());
            let spending_height = height_cursor.get(spending_tx_index.to_usize()).unwrap();

            let (block_hash, block_time) = if let Some((h, ref bh, bt)) = cached_block
                && h == spending_height
            {
                (bh.clone(), bt)
            } else {
                let bh = blockhash_reader.get(spending_height.to_usize());
                let bt = block_ts_cursor.get(spending_height.to_usize()).unwrap();
                cached_block = Some((spending_height, bh.clone(), bt));
                (bh, bt)
            };

            outspends.push(TxOutspend {
                spent: true,
                txid: Some(spending_txid),
                vin: Some(vin),
                status: Some(TxStatus {
                    confirmed: true,
                    block_height: Some(spending_height),
                    block_hash: Some(block_hash),
                    block_time: Some(block_time),
                }),
            });
        }

        Ok(outspends)
    }

    /// Resolve txid to (tx_index, first_txout_index, output_count).
    fn resolve_tx_outputs(&self, txid: &Txid) -> Result<(TxIndex, TxOutIndex, usize)> {
        let prefix = TxidPrefix::from(txid);
        let indexer = self.indexer();
        let tx_index: TxIndex = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)?
            .map(|cow| cow.into_owned())
            .ok_or(Error::UnknownTxid)?;
        let first = indexer
            .vecs
            .transactions
            .first_txout_index
            .read_once(tx_index)?;
        let next = indexer
            .vecs
            .transactions
            .first_txout_index
            .read_once(tx_index.incremented())?;
        Ok((tx_index, first, usize::from(next) - usize::from(first)))
    }

    /// Resolve spend status for a single output.
    fn resolve_outspend(&self, txout_index: TxOutIndex) -> Result<TxOutspend> {
        let indexer = self.indexer();
        let txin_index = self
            .computer()
            .outputs
            .spent
            .txin_index
            .reader()
            .get(usize::from(txout_index));

        if txin_index == TxInIndex::UNSPENT {
            return Ok(TxOutspend::UNSPENT);
        }

        let spending_tx_index = indexer
            .vecs
            .inputs
            .tx_index
            .collect_one_at(usize::from(txin_index))
            .unwrap();
        let spending_first_txin = indexer
            .vecs
            .transactions
            .first_txin_index
            .collect_one(spending_tx_index)
            .unwrap();
        let spending_height = indexer
            .vecs
            .transactions
            .height
            .collect_one(spending_tx_index)
            .unwrap();

        Ok(TxOutspend {
            spent: true,
            txid: Some(
                indexer
                    .vecs
                    .transactions
                    .txid
                    .reader()
                    .get(spending_tx_index.to_usize()),
            ),
            vin: Some(Vin::from(
                usize::from(txin_index) - usize::from(spending_first_txin),
            )),
            status: Some(TxStatus {
                confirmed: true,
                block_height: Some(spending_height),
                block_hash: Some(
                    indexer
                        .vecs
                        .blocks
                        .blockhash
                        .reader()
                        .get(spending_height.to_usize()),
                ),
                block_time: Some(
                    indexer
                        .vecs
                        .blocks
                        .timestamp
                        .collect_one(spending_height)
                        .unwrap(),
                ),
            }),
        })
    }

    // === Helper methods ===

    pub fn transaction_by_index(&self, tx_index: TxIndex) -> Result<Transaction> {
        self.transactions_by_indices(&[tx_index])?
            .into_iter()
            .next()
            .ok_or(Error::NotFound("Transaction not found".into()))
    }

    fn transaction_raw_by_index(&self, tx_index: TxIndex) -> Result<Vec<u8>> {
        let indexer = self.indexer();
        let total_size = indexer
            .vecs
            .transactions
            .total_size
            .collect_one(tx_index)
            .unwrap();
        let position = indexer
            .vecs
            .transactions
            .position
            .collect_one(tx_index)
            .unwrap();
        self.reader().read_raw_bytes(position, *total_size as usize)
    }

    fn transaction_hex_by_index(&self, tx_index: TxIndex) -> Result<String> {
        Ok(self
            .transaction_raw_by_index(tx_index)?
            .to_lower_hex_string())
    }

    pub fn resolve_tx(&self, txid: &Txid) -> Result<(TxIndex, Height)> {
        let indexer = self.indexer();
        let prefix = TxidPrefix::from(txid);
        let tx_index: TxIndex = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)?
            .map(|cow| cow.into_owned())
            .ok_or(Error::UnknownTxid)?;
        let height: Height = indexer
            .vecs
            .transactions
            .height
            .collect_one(tx_index)
            .unwrap();
        Ok((tx_index, height))
    }

    pub fn broadcast_transaction(&self, hex: &str) -> Result<Txid> {
        self.client().send_raw_transaction(hex)
    }

    pub fn merkleblock_proof(&self, txid: &Txid) -> Result<String> {
        let (_, height) = self.resolve_tx(txid)?;
        let header = self.read_block_header(height)?;
        let txids = self.block_txids_by_height(height)?;

        let target: bitcoin::Txid = txid.into();
        let btxids: Vec<bitcoin::Txid> = txids.iter().map(bitcoin::Txid::from).collect();
        let mb = bitcoin::MerkleBlock::from_header_txids_with_predicate(&header, &btxids, |t| {
            *t == target
        });
        Ok(bitcoin::consensus::encode::serialize_hex(&mb))
    }

    pub fn merkle_proof(&self, txid: &Txid) -> Result<MerkleProof> {
        let (tx_index, height) = self.resolve_tx(txid)?;
        let first_tx = self
            .indexer()
            .vecs
            .transactions
            .first_tx_index
            .collect_one(height)
            .ok_or(Error::NotFound("Block not found".into()))?;
        let pos = tx_index.to_usize() - first_tx.to_usize();
        let txids = self.block_txids_by_height(height)?;

        Ok(MerkleProof {
            block_height: height,
            merkle: merkle_path(&txids, pos),
            pos,
        })
    }
}

fn merkle_path(txids: &[Txid], pos: usize) -> Vec<String> {
    use bitcoin::hashes::{Hash, sha256d};

    // Txid bytes are in internal order (same layout as bitcoin::Txid)
    let mut hashes: Vec<[u8; 32]> = txids
        .iter()
        .map(|t| bitcoin::Txid::from(t).to_byte_array())
        .collect();

    let mut proof = Vec::new();
    let mut idx = pos;

    while hashes.len() > 1 {
        let sibling = if idx ^ 1 < hashes.len() { idx ^ 1 } else { idx };
        // Display order: reverse bytes for hex output
        let mut display = hashes[sibling];
        display.reverse();
        proof.push(bitcoin::hex::DisplayHex::to_lower_hex_string(&display));

        hashes = hashes
            .chunks(2)
            .map(|pair| {
                let right = pair.last().unwrap();
                let mut combined = [0u8; 64];
                combined[..32].copy_from_slice(&pair[0]);
                combined[32..].copy_from_slice(right);
                sha256d::Hash::hash(&combined).to_byte_array()
            })
            .collect();
        idx /= 2;
    }

    proof
}
