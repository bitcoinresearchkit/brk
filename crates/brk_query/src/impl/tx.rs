use std::io::Cursor;

use bitcoin::{consensus::Decodable, hex::DisplayHex};
use brk_error::{Error, Result};
use brk_types::{
    Height, MerkleProof, OutputType, Sats, Transaction, TxIn, TxInIndex, TxIndex, TxOut,
    TxOutspend, TxStatus, Txid, TxidParam, TxidPrefix, Vin, Vout, Weight,
};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn transaction(&self, TxidParam { txid }: TxidParam) -> Result<Transaction> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
        {
            return Ok(tx_with_hex.tx().clone());
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(&txid);
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

    pub fn transaction_status(&self, TxidParam { txid }: TxidParam) -> Result<TxStatus> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && mempool.get_txs().contains_key(&txid)
        {
            return Ok(TxStatus::UNCONFIRMED);
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(&txid);
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
        let height = indexer
            .vecs
            .transactions
            .height
            .collect_one(tx_index)
            .unwrap();
        let block_hash = indexer.vecs.blocks.blockhash.read_once(height)?;
        let block_time = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();

        Ok(TxStatus {
            confirmed: true,
            block_height: Some(height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        })
    }

    pub fn transaction_raw(&self, TxidParam { txid }: TxidParam) -> Result<Vec<u8>> {
        let prefix = TxidPrefix::from(&txid);
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

    pub fn transaction_hex(&self, TxidParam { txid }: TxidParam) -> Result<String> {
        // First check mempool for unconfirmed transactions
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
        {
            return Ok(tx_with_hex.hex().to_string());
        }

        // Look up confirmed transaction by txid prefix
        let prefix = TxidPrefix::from(&txid);
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

    pub fn outspend(&self, TxidParam { txid }: TxidParam, vout: Vout) -> Result<TxOutspend> {
        // Mempool outputs are unspent in on-chain terms
        if let Some(mempool) = self.mempool()
            && mempool.get_txs().contains_key(&txid)
        {
            return Ok(TxOutspend::UNSPENT);
        }

        // Look up confirmed transaction
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(tx_index)) = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        // Calculate txout_index
        let first_txout_index = indexer
            .vecs
            .transactions
            .first_txout_index
            .read_once(tx_index)?;
        let txout_index = first_txout_index + vout;

        // Look up spend status
        let computer = self.computer();
        let txin_index = computer.outputs.spent.txin_index.read_once(txout_index)?;

        if txin_index == TxInIndex::UNSPENT {
            return Ok(TxOutspend::UNSPENT);
        }

        self.outspend_details(txin_index)
    }

    pub fn outspends(&self, TxidParam { txid }: TxidParam) -> Result<Vec<TxOutspend>> {
        // Mempool outputs are unspent in on-chain terms
        if let Some(mempool) = self.mempool()
            && let Some(tx_with_hex) = mempool.get_txs().get(&txid)
        {
            let output_count = tx_with_hex.tx().output.len();
            return Ok(vec![TxOutspend::UNSPENT; output_count]);
        }

        // Look up confirmed transaction
        let prefix = TxidPrefix::from(&txid);
        let indexer = self.indexer();
        let Ok(Some(tx_index)) = indexer
            .stores
            .txid_prefix_to_tx_index
            .get(&prefix)
            .map(|opt| opt.map(|cow| cow.into_owned()))
        else {
            return Err(Error::UnknownTxid);
        };

        // Get output range
        let first_txout_index = indexer
            .vecs
            .transactions
            .first_txout_index
            .read_once(tx_index)?;
        let next_first_txout_index = indexer
            .vecs
            .transactions
            .first_txout_index
            .read_once(tx_index.incremented())?;
        let output_count = usize::from(next_first_txout_index) - usize::from(first_txout_index);

        // Get spend status for each output
        let computer = self.computer();
        let txin_index_reader = computer.outputs.spent.txin_index.reader();

        let mut outspends = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let txout_index = first_txout_index + Vout::from(i);
            let txin_index = txin_index_reader.get(usize::from(txout_index));

            if txin_index == TxInIndex::UNSPENT {
                outspends.push(TxOutspend::UNSPENT);
            } else {
                outspends.push(self.outspend_details(txin_index)?);
            }
        }

        Ok(outspends)
    }

    // === Helper methods ===

    pub fn transaction_by_index(&self, tx_index: TxIndex) -> Result<Transaction> {
        let indexer = self.indexer();
        let reader = self.reader();

        // Get tx metadata using collect_one for PcoVec, read_once for BytesVec
        let txid = indexer.vecs.transactions.txid.read_once(tx_index)?;
        let height = indexer
            .vecs
            .transactions
            .height
            .collect_one(tx_index)
            .unwrap();
        let version = indexer
            .vecs
            .transactions
            .tx_version
            .collect_one(tx_index)
            .unwrap();
        let lock_time = indexer
            .vecs
            .transactions
            .raw_locktime
            .collect_one(tx_index)
            .unwrap();
        let total_size = indexer
            .vecs
            .transactions
            .total_size
            .collect_one(tx_index)
            .unwrap();
        let first_txin_index = indexer
            .vecs
            .transactions
            .first_txin_index
            .collect_one(tx_index)
            .unwrap();
        let position = indexer
            .vecs
            .transactions
            .position
            .collect_one(tx_index)
            .unwrap();

        // Get block info for status
        let block_hash = indexer.vecs.blocks.blockhash.read_once(height)?;
        let block_time = indexer.vecs.blocks.timestamp.collect_one(height).unwrap();

        // Read and decode the raw transaction from blk file
        let buffer = reader.read_raw_bytes(position, *total_size as usize)?;
        let mut cursor = Cursor::new(buffer);
        let tx = bitcoin::Transaction::consensus_decode(&mut cursor)
            .map_err(|_| Error::Parse("Failed to decode transaction".into()))?;

        // Create readers for random access lookups
        let txid_reader = indexer.vecs.transactions.txid.reader();
        let first_txout_index_reader = indexer.vecs.transactions.first_txout_index.reader();
        let value_reader = indexer.vecs.outputs.value.reader();
        let output_type_reader = indexer.vecs.outputs.output_type.reader();
        let type_index_reader = indexer.vecs.outputs.type_index.reader();
        let addr_readers = indexer.vecs.addrs.addr_readers();

        // Batch-read outpoints for all inputs (avoids per-input PcoVec page decompression)
        let outpoints: Vec<_> = indexer.vecs.inputs.outpoint.collect_range_at(
            usize::from(first_txin_index),
            usize::from(first_txin_index) + tx.input.len(),
        );

        // Build inputs with prevout information
        let input: Vec<TxIn> = tx
            .input
            .iter()
            .enumerate()
            .map(|(i, txin)| {
                let outpoint = outpoints[i];

                let is_coinbase = outpoint.is_coinbase();

                // Get prevout info if not coinbase
                let (prev_txid, prev_vout, prevout) = if is_coinbase {
                    (Txid::COINBASE, Vout::MAX, None)
                } else {
                    let prev_tx_index = outpoint.tx_index();
                    let prev_vout = outpoint.vout();
                    let prev_txid = txid_reader.get(prev_tx_index.to_usize());

                    // Calculate the txout_index for the prevout
                    let prev_first_txout_index =
                        first_txout_index_reader.get(prev_tx_index.to_usize());
                    let prev_txout_index = prev_first_txout_index + prev_vout;

                    let prev_value = value_reader.get(usize::from(prev_txout_index));
                    let prev_output_type: OutputType =
                        output_type_reader.get(usize::from(prev_txout_index));
                    let prev_type_index = type_index_reader.get(usize::from(prev_txout_index));
                    let script_pubkey =
                        addr_readers.script_pubkey(prev_output_type, prev_type_index);

                    let prevout = Some(TxOut::from((script_pubkey, prev_value)));

                    (prev_txid, prev_vout, prevout)
                };

                TxIn {
                    txid: prev_txid,
                    vout: prev_vout,
                    prevout,
                    script_sig: txin.script_sig.clone(),
                    script_sig_asm: (),
                    is_coinbase,
                    sequence: txin.sequence.0,
                    inner_redeem_script_asm: (),
                }
            })
            .collect();

        // Calculate weight before consuming tx.output
        let weight = Weight::from(tx.weight());

        // Calculate sigop cost
        let total_sigop_cost = tx.total_sigop_cost(|_| None);

        // Build outputs
        let output: Vec<TxOut> = tx.output.into_iter().map(TxOut::from).collect();

        // Build status
        let status = TxStatus {
            confirmed: true,
            block_height: Some(height),
            block_hash: Some(block_hash),
            block_time: Some(block_time),
        };

        let mut transaction = Transaction {
            index: Some(tx_index),
            txid,
            version,
            lock_time,
            total_size: *total_size as usize,
            weight,
            total_sigop_cost,
            fee: Sats::ZERO, // Will be computed below
            input,
            output,
            status,
        };

        // Compute fee from inputs - outputs
        transaction.compute_fee();

        Ok(transaction)
    }

    fn transaction_raw_by_index(&self, tx_index: TxIndex) -> Result<Vec<u8>> {
        let indexer = self.indexer();
        let total_size = indexer.vecs.transactions.total_size.collect_one(tx_index).unwrap();
        let position = indexer.vecs.transactions.position.collect_one(tx_index).unwrap();
        self.reader().read_raw_bytes(position, *total_size as usize)
    }

    fn transaction_hex_by_index(&self, tx_index: TxIndex) -> Result<String> {
        Ok(self.transaction_raw_by_index(tx_index)?.to_lower_hex_string())
    }

    fn outspend_details(&self, txin_index: TxInIndex) -> Result<TxOutspend> {
        let indexer = self.indexer();

        // Look up spending tx_index directly
        let spending_tx_index = indexer
            .vecs
            .inputs
            .tx_index
            .collect_one(txin_index)
            .unwrap();

        // Calculate vin
        let spending_first_txin_index = indexer
            .vecs
            .transactions
            .first_txin_index
            .collect_one(spending_tx_index)
            .unwrap();
        let vin = Vin::from(usize::from(txin_index) - usize::from(spending_first_txin_index));

        // Get spending tx details
        let spending_txid = indexer
            .vecs
            .transactions
            .txid
            .read_once(spending_tx_index)?;
        let spending_height = indexer
            .vecs
            .transactions
            .height
            .collect_one(spending_tx_index)
            .unwrap();
        let block_hash = indexer.vecs.blocks.blockhash.read_once(spending_height)?;
        let block_time = indexer
            .vecs
            .blocks
            .timestamp
            .collect_one(spending_height)
            .unwrap();

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

    fn resolve_tx(&self, txid: &Txid) -> Result<(TxIndex, Height)> {
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

    pub fn merkleblock_proof(&self, txid_param: TxidParam) -> Result<String> {
        let (_, height) = self.resolve_tx(&txid_param.txid)?;
        let header = self.read_block_header(height)?;
        let txids = self.block_txids_by_height(height)?;

        let target: bitcoin::Txid = (&txid_param.txid).into();
        let btxids: Vec<bitcoin::Txid> = txids.iter().map(bitcoin::Txid::from).collect();
        let mb = bitcoin::MerkleBlock::from_header_txids_with_predicate(&header, &btxids, |t| {
            *t == target
        });
        Ok(bitcoin::consensus::encode::serialize_hex(&mb))
    }

    pub fn merkle_proof(&self, txid_param: TxidParam) -> Result<MerkleProof> {
        let (tx_index, height) = self.resolve_tx(&txid_param.txid)?;
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
