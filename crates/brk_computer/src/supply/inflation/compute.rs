use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution};

impl Vecs {
    pub fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // inflation = daily_subsidy / circulating_supply * 365 * 100
        let circulating_supply = &distribution.utxo_cohorts.all.metrics.supply.supply.sats;

        self.compute_all(starting_indexes, exit, |v| {
            v.compute_transform2(
                starting_indexes.dateindex,
                &blocks.rewards.subsidy.sats.dateindex.sum_cum.sum.0,
                &circulating_supply.dateindex.0,
                |(i, subsidy_1d_sum, supply, ..)| {
                    let inflation = if *supply > 0 {
                        365.0 * *subsidy_1d_sum as f64 / *supply as f64 * 100.0
                    } else {
                        0.0
                    };
                    (i, inflation.into())
                },
                exit,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
