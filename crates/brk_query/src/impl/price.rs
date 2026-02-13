use brk_error::{Error, Result};
use brk_types::Dollars;

use crate::Query;

impl Query {
    pub fn live_price(&self) -> Result<Dollars> {
        let oracle_vecs = &self
            .computer()
            .price
            .as_ref()
            .ok_or_else(|| Error::OutOfRange("Oracle prices not computed yet".into()))?
            .oracle;

        let mut oracle = oracle_vecs.live_oracle(self.indexer())?;

        if let Some(mempool) = self.mempool() {
            let txs = mempool.get_txs();
            oracle.process_outputs(
                txs.values()
                    .flat_map(|tx| &tx.tx().output)
                    .map(|txout| (txout.value, txout.type_())),
            );
        }

        Ok(oracle.price_dollars())
    }
}
