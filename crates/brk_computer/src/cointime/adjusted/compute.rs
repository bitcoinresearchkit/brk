use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use super::super::activity;
use crate::{supply, ComputeIndexes, utils::OptionExt};

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        supply: &supply::Vecs,
        activity: &activity::Vecs,
        has_price: bool,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_cointime_adj_inflation_rate
            .compute_all(starting_indexes, exit, |v| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    activity
                        .indexes_to_activity_to_vaultedness_ratio
                        .dateindex
                        .unwrap_last(),
                    supply.inflation.indexes.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_cointime_adj_tx_btc_velocity
            .compute_all(starting_indexes, exit, |v| {
                v.compute_multiply(
                    starting_indexes.dateindex,
                    activity
                        .indexes_to_activity_to_vaultedness_ratio
                        .dateindex
                        .unwrap_last(),
                    supply.velocity.indexes_to_btc.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        if has_price {
            self.indexes_to_cointime_adj_tx_usd_velocity.compute_all(
                starting_indexes,
                exit,
                |v| {
                    v.compute_multiply(
                        starting_indexes.dateindex,
                        activity
                            .indexes_to_activity_to_vaultedness_ratio
                            .dateindex
                            .unwrap_last(),
                        supply.velocity.indexes_to_usd.u().dateindex.u(),
                        exit,
                    )?;
                    Ok(())
                },
            )?;
        }

        Ok(())
    }
}
