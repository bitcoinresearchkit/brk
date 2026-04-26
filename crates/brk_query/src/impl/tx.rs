use bitcoin::hex::DisplayHex;
use brk_error::{Error, OptionData, Result};
use brk_types::{
    BlockHash, Height, MerkleProof, Timestamp, Transaction, TxInIndex, TxIndex, TxOutIndex,
    TxOutspend, TxStatus, Txid, TxidPrefix, Vin, Vout,
};
use vecdb::{AnyVec, ReadableVec, VecIndex};

use crate::Query;

impl Query {
    // ── Txid → TxIndex resolution (single source of truth) ─────────

    /// Resolve a txid to its internal TxIndex via prefix lookup.
    #[inline]
    pub(crate) fn resolve_tx_index(&self, txid: &Txid) -> Result<TxIndex> {
        self.indexer()
            .stores
            .txid_prefix_to_tx_index
            .get(&TxidPrefix::from(txid))
            .map_err(|_| Error::UnknownTxid)?
            .map(|cow| cow.into_owned())
            .ok_or(Error::UnknownTxid)
    }

    pub fn txid_by_index(&self, index: TxIndex) -> Result<Txid> {
        let len = self.indexer().vecs.transactions.txid.len();
        if index.to_usize() >= len {
            return Err(Error::OutOfRange("Transaction index out of range".into()));
        }
        self.indexer()
            .vecs
            .transactions
            .txid
            .collect_one(index)
            .ok_or_else(|| Error::OutOfRange("Transaction index out of range".into()))
    }

    /// Resolve a txid to (TxIndex, Height).
    pub fn resolve_tx(&self, txid: &Txid) -> Result<(TxIndex, Height)> {
        let tx_index = self.resolve_tx_index(txid)?;
        let height = self.confirmed_status_height(tx_index)?;
        Ok((tx_index, height))
    }

    // ── TxStatus construction (single source of truth) ─────────────

    /// Height for a confirmed tx_index via in-memory TxHeights lookup.
    #[inline]
    pub(crate) fn confirmed_status_height(&self, tx_index: TxIndex) -> Result<Height> {
        self.computer()
            .indexes
            .tx_heights
            .get_shared(tx_index)
            .data()
    }

    /// Full confirmed TxStatus from a tx_index.
    #[inline]
    pub(crate) fn confirmed_status(&self, tx_index: TxIndex) -> Result<TxStatus> {
        let height = self.confirmed_status_height(tx_index)?;
        self.confirmed_status_at(height)
    }

    /// Full confirmed TxStatus from a known height.
    #[inline]
    pub(crate) fn confirmed_status_at(&self, height: Height) -> Result<TxStatus> {
        let (block_hash, block_time) = self.block_hash_and_time(height)?;
        Ok(TxStatus {
            confirmed: true,
            block_height: Some(height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        })
    }

    /// Block hash + timestamp for a height (cached vecs, fast).
    #[inline]
    pub(crate) fn block_hash_and_time(&self, height: Height) -> Result<(BlockHash, Timestamp)> {
        let indexer = self.indexer();
        let hash = indexer.vecs.blocks.blockhash.collect_one(height).data()?;
        let time = indexer.vecs.blocks.timestamp.collect_one(height).data()?;
        Ok((hash, time))
    }

    // ── Transaction queries ────────────────────────────────────────

    pub fn transaction(&self, txid: &Txid) -> Result<Transaction> {
        if let Some(mempool) = self.mempool()
            && let Some(tx) = mempool.txs().get(txid)
        {
            return Ok(tx.clone());
        }
        self.transaction_by_index(self.resolve_tx_index(txid)?)
    }

    pub fn transaction_status(&self, txid: &Txid) -> Result<TxStatus> {
        if self.mempool().is_some_and(|m| m.txs().contains_key(txid)) {
            return Ok(TxStatus::UNCONFIRMED);
        }
        self.confirmed_status(self.resolve_tx_index(txid)?)
    }

    pub fn transaction_raw(&self, txid: &Txid) -> Result<Vec<u8>> {
        if let Some(mempool) = self.mempool()
            && let Some(tx) = mempool.txs().get(txid)
        {
            return Ok(tx.encode_bytes());
        }
        self.transaction_raw_by_index(self.resolve_tx_index(txid)?)
    }

    pub fn transaction_hex(&self, txid: &Txid) -> Result<String> {
        if let Some(mempool) = self.mempool()
            && let Some(tx) = mempool.txs().get(txid)
        {
            return Ok(tx.encode_bytes().to_lower_hex_string());
        }
        self.transaction_hex_by_index(self.resolve_tx_index(txid)?)
    }

    // ── Outspend queries ───────────────────────────────────────────

    pub fn outspend(&self, txid: &Txid, vout: Vout) -> Result<TxOutspend> {
        if self.mempool().is_some_and(|m| m.txs().contains_key(txid)) {
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
            && let Some(tx) = mempool.txs().get(txid)
        {
            return Ok(vec![TxOutspend::UNSPENT; tx.output.len()]);
        }
        let (_, first_txout, output_count) = self.resolve_tx_outputs(txid)?;
        self.resolve_outspends(first_txout, output_count)
    }

    /// Resolve spend status for a single output. Minimal reads.
    fn resolve_outspend(&self, txout_index: TxOutIndex) -> Result<TxOutspend> {
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

        self.build_outspend(txin_index)
    }

    /// Resolve spend status for a contiguous range of outputs.
    /// Readers/cursors created once, reused for all outputs.
    fn resolve_outspends(
        &self,
        first_txout: TxOutIndex,
        output_count: usize,
    ) -> Result<Vec<TxOutspend>> {
        let indexer = self.indexer();
        let txin_index_reader = self.computer().outputs.spent.txin_index.reader();
        let txid_reader = indexer.vecs.transactions.txid.reader();

        let tx_heights = &self.computer().indexes.tx_heights;
        let mut input_tx_cursor = indexer.vecs.inputs.tx_index.cursor();
        let mut first_txin_cursor = indexer.vecs.transactions.first_txin_index.cursor();

        let mut cached_status: Option<(Height, BlockHash, Timestamp)> = None;
        let mut outspends = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let txin_index = txin_index_reader.get(usize::from(first_txout + Vout::from(i)));

            if txin_index == TxInIndex::UNSPENT {
                outspends.push(TxOutspend::UNSPENT);
                continue;
            }

            let spending_tx_index = input_tx_cursor.get(usize::from(txin_index)).data()?;
            let spending_first_txin = first_txin_cursor.get(spending_tx_index.to_usize()).data()?;
            let vin = Vin::from(usize::from(txin_index) - usize::from(spending_first_txin));
            let spending_txid = txid_reader.get(spending_tx_index.to_usize());
            let spending_height: Height = tx_heights.get_shared(spending_tx_index).data()?;

            let (block_hash, block_time) =
                if let Some((h, ref bh, bt)) = cached_status
                    && h == spending_height
                {
                    (bh.clone(), bt)
                } else {
                    let (bh, bt) = self.block_hash_and_time(spending_height)?;
                    cached_status = Some((spending_height, bh.clone(), bt));
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

    /// Build a single TxOutspend from a known-spent TxInIndex.
    fn build_outspend(&self, txin_index: TxInIndex) -> Result<TxOutspend> {
        let indexer = self.indexer();
        let spending_tx_index: TxIndex = indexer
            .vecs
            .inputs
            .tx_index
            .collect_one_at(usize::from(txin_index))
            .data()?;
        let spending_first_txin: TxInIndex = indexer
            .vecs
            .transactions
            .first_txin_index
            .collect_one(spending_tx_index)
            .data()?;
        let vin = Vin::from(usize::from(txin_index) - usize::from(spending_first_txin));
        let spending_txid = indexer
            .vecs
            .transactions
            .txid
            .reader()
            .get(spending_tx_index.to_usize());
        let spending_height = self.confirmed_status_height(spending_tx_index)?;
        let (block_hash, block_time) = self.block_hash_and_time(spending_height)?;

        Ok(TxOutspend {
            spent: true,
            txid: Some(spending_txid),
            vin: Some(vin),
            status: Some(TxStatus {
                confirmed: true,
                block_height: Some(spending_height),
                block_hash: Some(block_hash),
                block_time: Some(block_time),
            }),
        })
    }

    /// Resolve txid to (tx_index, first_txout_index, output_count).
    fn resolve_tx_outputs(&self, txid: &Txid) -> Result<(TxIndex, TxOutIndex, usize)> {
        let tx_index = self.resolve_tx_index(txid)?;
        let indexer = self.indexer();
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
            .data()?;
        let position = indexer
            .vecs
            .transactions
            .position
            .collect_one(tx_index)
            .data()?;
        self.reader().read_raw_bytes(position, *total_size as usize)
    }

    fn transaction_hex_by_index(&self, tx_index: TxIndex) -> Result<String> {
        Ok(self
            .transaction_raw_by_index(tx_index)?
            .to_lower_hex_string())
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
