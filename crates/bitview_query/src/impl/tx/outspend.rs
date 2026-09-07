use brk_error::{Error, OptionData, Result};
use brk_types::{
    BlockHash, Height, Timestamp, TxInIndex, TxOutIndex, TxOutspend, TxStatus, Txid, Vin, Vout,
};
use serde_json::to_vec;
use vecdb::{ReadableVec, VecIndex};

use crate::{Query, RepresentationId};

impl Query {
    pub fn outspend(&self, txid: &Txid, vout: Vout) -> Result<TxOutspend> {
        let plugins = self.plugins();
        let _guard = self.read_plugins(vec![plugins.indexer, plugins.mappings, plugins.outputs])?;
        let (_, first_txout, output_count) = match self.resolve_tx_outputs(txid) {
            Ok(outputs) => outputs,
            Err(Error::UnknownTxid) => {
                return self
                    .mempool()
                    .ok_or(Error::UnknownTxid)?
                    .outspend_if_present(txid, vout, &self.tip_blockhash())?
                    .ok_or(Error::UnknownTxid);
            }
            Err(error) => return Err(error),
        };
        if usize::from(vout) >= output_count {
            return Ok(TxOutspend::UNSPENT);
        }
        let confirmed = self
            .resolve_outspends(first_txout + vout, 1)?
            .pop()
            .data()?;
        if confirmed.spent {
            return Ok(confirmed);
        }
        self.mempool_outspend(txid, vout)
    }

    /// Resolve and serialize one outspend exactly once, identifying confirmed
    /// results by their spending block and live results by their content.
    pub fn outspend_json(&self, txid: &Txid, vout: Vout) -> Result<(Vec<u8>, RepresentationId)> {
        let outspend = self.outspend(txid, vout)?;
        let bytes = to_vec(&outspend).unwrap();
        let identity = outspend_identity(&outspend, &bytes);
        Ok((bytes, identity))
    }

    pub fn outspends(&self, txid: &Txid) -> Result<Vec<TxOutspend>> {
        let plugins = self.plugins();
        let _guard = self.read_plugins(vec![plugins.indexer, plugins.mappings, plugins.outputs])?;
        let (_, first_txout, output_count) = match self.resolve_tx_outputs(txid) {
            Ok(outputs) => outputs,
            Err(Error::UnknownTxid) => {
                return self
                    .mempool()
                    .ok_or(Error::UnknownTxid)?
                    .outspends_if_present(txid, &self.tip_blockhash())?
                    .ok_or(Error::UnknownTxid);
            }
            Err(error) => return Err(error),
        };
        let mut outspends = self.resolve_outspends(first_txout, output_count)?;
        if let Some(mempool) = self.mempool() {
            mempool.merge_outspends(txid, &mut outspends, &self.tip_blockhash())?;
        }
        Ok(outspends)
    }

    /// Resolve and serialize all outspends exactly once. Fully confirmed
    /// arrays are identified by their newest spending block; arrays with any
    /// live element are identified by their exact content.
    pub fn outspends_json(&self, txid: &Txid) -> Result<(Vec<u8>, RepresentationId)> {
        let outspends = self.outspends(txid)?;
        let bytes = to_vec(&outspends).unwrap();
        let identity = outspends_identity(&outspends, &bytes);
        Ok((bytes, identity))
    }

    /// Resolve spend status for a contiguous range of outputs.
    /// Readers/cursors created once, reused for all outputs.
    fn resolve_outspends(
        &self,
        first_txout: TxOutIndex,
        output_count: usize,
    ) -> Result<Vec<TxOutspend>> {
        let indexer = self.indexer();
        let txin_index_reader = self.plugins().outputs.spent.txin_index.reader();
        let txid_reader = indexer.vecs().transactions.txid.reader();

        let tx_heights = &self.plugins().mappings.tx_heights;
        let mut input_tx_cursor = indexer.vecs().inputs.tx_index.cursor();
        let mut first_txin_cursor = indexer.vecs().transactions.first_txin_index.cursor();

        let bound = self.safe_lengths();

        let mut cached_status: Option<(Height, BlockHash, Timestamp)> = None;
        let mut outspends = Vec::with_capacity(output_count);
        for index in 0..output_count {
            let txin_index = txin_index_reader
                .try_get(first_txout + Vout::from(index))
                .data()?;

            if txin_index == TxInIndex::UNSPENT || txin_index >= bound.txin_index {
                outspends.push(TxOutspend::UNSPENT);
                continue;
            }

            let spending_tx_index = input_tx_cursor.get(usize::from(txin_index)).data()?;
            if spending_tx_index >= bound.tx_index {
                outspends.push(TxOutspend::UNSPENT);
                continue;
            }
            let spending_first_txin = first_txin_cursor.get(spending_tx_index.to_usize()).data()?;
            let vin = checked_vin(txin_index, spending_first_txin)?;
            let spending_txid = txid_reader.try_get(spending_tx_index).data()?;
            let spending_height: Height = tx_heights.get_shared(spending_tx_index).data()?;
            if spending_height >= bound.height {
                return Err(Error::UnknownTxid);
            }

            let (block_hash, block_time) = if let Some((height, hash, time)) = cached_status
                && height == spending_height
            {
                (hash, time)
            } else {
                let (hash, time) = self.block_hash_and_time(spending_height)?;
                cached_status = Some((spending_height, hash, time));
                (hash, time)
            };

            outspends.push(TxOutspend {
                spent: true,
                txid: Some(spending_txid),
                vin: Some(vin),
                status: Some(TxStatus::confirmed(spending_height, block_hash, block_time)),
            });
        }

        Ok(outspends)
    }
}

fn checked_vin(input: TxInIndex, first: TxInIndex) -> Result<Vin> {
    usize::from(input)
        .checked_sub(usize::from(first))
        .filter(|vin| *vin <= usize::from(u16::MAX))
        .map(Vin::from)
        .ok_or(Error::Internal("Invalid spending input position"))
}

fn outspend_identity(outspend: &TxOutspend, bytes: &[u8]) -> RepresentationId {
    confirmed_spending_block(outspend).map_or_else(
        || RepresentationId::content(bytes),
        |(hash, _)| RepresentationId::Block(hash),
    )
}

fn confirmed_spending_block(outspend: &TxOutspend) -> Option<(BlockHash, Height)> {
    if !outspend.spent {
        return None;
    }
    let status = outspend.status.as_ref()?;
    if !status.confirmed {
        return None;
    }
    status.block_hash.zip(status.block_height)
}

fn outspends_identity(outspends: &[TxOutspend], bytes: &[u8]) -> RepresentationId {
    let content = || RepresentationId::content(bytes);
    let mut newest = None;

    for outspend in outspends {
        let Some((hash, height)) = confirmed_spending_block(outspend) else {
            return content();
        };

        match newest {
            Some((newest_hash, newest_height)) if newest_height == height => {
                if newest_hash != hash {
                    return content();
                }
            }
            Some((_, newest_height)) if newest_height > height => {}
            _ => newest = Some((hash, height)),
        }
    }

    newest.map_or_else(content, |(hash, _)| RepresentationId::Block(hash))
}

#[cfg(test)]
#[path = "../../../tests/unit/impl/tx/outspend.rs"]
mod tests;
impl Query {
    fn mempool_outspend(&self, txid: &Txid, vout: Vout) -> Result<TxOutspend> {
        self.mempool().map_or(Ok(TxOutspend::UNSPENT), |mempool| {
            mempool.outspend(txid, vout, &self.tip_blockhash())
        })
    }
}
