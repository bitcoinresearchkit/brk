use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, distribution, transactions};

impl Vecs {
    pub fn compute(
        &mut self,
        transactions: &transactions::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // velocity = annualized_volume / circulating_supply
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.supply;

        // BTC velocity
        self.btc.compute_all(starting_indexes, exit, |v| {
            v.compute_divide(
                starting_indexes.dateindex,
                &*transactions.volume.annualized_volume.bitcoin.dateindex,
                &*circulating_supply.bitcoin.dateindex,
                exit,
            )?;
            Ok(())
        })?;

        // USD velocity
        if let Some(usd_velocity) = self.usd.as_mut()
            && let Some(supply_usd) = circulating_supply.dollars.as_ref()
            && let Some(volume_usd) = transactions.volume.annualized_volume.dollars.as_ref()
        {
            usd_velocity.compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    &volume_usd.dateindex,
                    &supply_usd.dateindex.0,
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
