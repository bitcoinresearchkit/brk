use brk_error::{Error, Result};
use brk_types::{
    CpfpEntry, CpfpInfo, MempoolBlock, MempoolInfo, MempoolRecentTx, RecommendedFees, Txid,
    TxidParam, TxidPrefix, Weight,
};

use crate::Query;

impl Query {
    pub fn mempool_info(&self) -> Result<MempoolInfo> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        Ok(mempool.get_info())
    }

    pub fn mempool_txids(&self) -> Result<Vec<Txid>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let txs = mempool.get_txs();
        Ok(txs.keys().cloned().collect())
    }

    pub fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.mempool()
            .map(|mempool| mempool.get_fees())
            .ok_or(Error::MempoolNotAvailable)
    }

    pub fn mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;

        let block_stats = mempool.get_block_stats();

        let blocks = block_stats
            .into_iter()
            .map(|stats| {
                MempoolBlock::new(
                    stats.tx_count,
                    stats.total_vsize,
                    stats.total_fee,
                    stats.fee_range,
                )
            })
            .collect();

        Ok(blocks)
    }

    pub fn mempool_recent(&self) -> Result<Vec<MempoolRecentTx>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        Ok(mempool.get_txs().recent().to_vec())
    }

    pub fn cpfp(&self, TxidParam { txid }: TxidParam) -> Result<CpfpInfo> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let entries = mempool.get_entries();
        let prefix = TxidPrefix::from(&txid);

        let entry = entries
            .get(&prefix)
            .ok_or(Error::NotFound("Transaction not in mempool".into()))?;

        // Ancestors: walk up the depends chain
        let mut ancestors = Vec::new();
        let mut stack: Vec<TxidPrefix> = entry.depends.to_vec();
        while let Some(p) = stack.pop() {
            if let Some(anc) = entries.get(&p) {
                ancestors.push(CpfpEntry {
                    txid: anc.txid.clone(),
                    weight: Weight::from(anc.vsize),
                    fee: anc.fee,
                });
                stack.extend(anc.depends.iter().cloned());
            }
        }

        // Descendants: find entries that depend on this tx's prefix
        let mut descendants = Vec::new();
        for e in entries.entries().iter().flatten() {
            if e.depends.contains(&prefix) {
                descendants.push(CpfpEntry {
                    txid: e.txid.clone(),
                    weight: Weight::from(e.vsize),
                    fee: e.fee,
                });
            }
        }

        let effective_fee_per_vsize = entry.effective_fee_rate();

        Ok(CpfpInfo {
            ancestors,
            descendants,
            effective_fee_per_vsize,
        })
    }

    pub fn transaction_times(&self, txids: &[Txid]) -> Result<Vec<u64>> {
        let mempool = self.mempool().ok_or(Error::MempoolNotAvailable)?;
        let entries = mempool.get_entries();
        Ok(txids
            .iter()
            .map(|txid| {
                entries
                    .get(&TxidPrefix::from(txid))
                    .map(|e| usize::from(e.first_seen) as u64)
                    .unwrap_or(0)
            })
            .collect())
    }
}
