use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{distribution, transactions, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        transactions: &transactions::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // velocity = annualized_volume / circulating_supply
        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .indexes_to_supply;

        // BTC velocity - KISS: dateindex is no longer Option
        self.indexes_to_btc
            .compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    &transactions.volume.indexes_to_annualized_volume_btc.dateindex,
                    &*circulating_supply.bitcoin.dateindex,
                    exit,
                )?;
                Ok(())
            })?;

        // USD velocity
        if let Some(usd_velocity) = self.indexes_to_usd.as_mut()
            && let Some(supply_usd) = circulating_supply.dollars.as_ref()
        {
            // KISS: dateindex is no longer Option
            usd_velocity.compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    &transactions.volume.indexes_to_annualized_volume_usd.dateindex,
                    &supply_usd.dateindex,
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
