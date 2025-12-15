use brk_error::{Error, Result};
use brk_types::{MempoolBlock, MempoolInfo, RecommendedFees, Txid};

use crate::Query;

impl Query {
    pub fn mempool_info(&self) -> Result<MempoolInfo> {
        let mempool = self.mempool().ok_or(Error::Str("Mempool not available"))?;
        Ok(mempool.get_info())
    }

    pub fn mempool_txids(&self) -> Result<Vec<Txid>> {
        let mempool = self.mempool().ok_or(Error::Str("Mempool not available"))?;
        let txs = mempool.get_txs();
        Ok(txs.keys().cloned().collect())
    }

    pub fn recommended_fees(&self) -> Result<RecommendedFees> {
        self.mempool()
            .map(|mempool| mempool.get_fees())
            .ok_or(Error::MempoolNotAvailable)
    }

    pub fn mempool_blocks(&self) -> Result<Vec<MempoolBlock>> {
        let mempool = self.mempool().ok_or(Error::Str("Mempool not available"))?;

        let block_stats = mempool.get_block_stats();

        let blocks = block_stats
            .into_iter()
            .map(|stats| {
                MempoolBlock::new(stats.tx_count, stats.total_vsize, stats.total_fee, stats.fee_range)
            })
            .collect();

        Ok(blocks)
    }
}
