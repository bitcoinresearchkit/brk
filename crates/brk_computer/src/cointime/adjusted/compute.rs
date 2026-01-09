use brk_error::Result;
use vecdb::Exit;

use super::super::activity;
use super::Vecs;
use crate::{ComputeIndexes, supply};

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        supply: &supply::Vecs,
        activity: &activity::Vecs,
        has_price: bool,
        exit: &Exit,
    ) -> Result<()> {
        self.cointime_adj_inflation_rate
            .compute_all(starting_indexes, exit, |v| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    activity.activity_to_vaultedness_ratio.dateindex.inner(),
                    &supply.inflation.dateindex,
                    exit,
                )?;
                Ok(())
            })?;

        self.cointime_adj_tx_btc_velocity
            .compute_all(starting_indexes, exit, |v| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    activity.activity_to_vaultedness_ratio.dateindex.inner(),
                    &supply.velocity.btc.dateindex,
                    exit,
                )?;
                Ok(())
            })?;

        if has_price {
            self.cointime_adj_tx_usd_velocity
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_multiply(
                        starting_indexes.dateindex,
                        activity.activity_to_vaultedness_ratio.dateindex.inner(),
                        &supply.velocity.usd.as_ref().unwrap().dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        Ok(())
    }
}
