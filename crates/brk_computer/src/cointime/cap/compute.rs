use brk_error::Result;
use brk_types::Dollars;
use vecdb::Exit;

use super::super::{activity, value};
use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, indexes, utils::OptionExt};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        blocks: &blocks::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let realized_cap = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .u()
            .realized_cap
            .height;

        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .supply
            .bitcoin
            .height;

        self.thermo_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    &blocks
                        .rewards
                        .subsidy
                        .dollars
                        .as_ref()
                        .unwrap()
                        .height_cumulative
                        .0,
                    |(i, v, ..)| (i, v),
                    exit,
                )?;
                Ok(())
            })?;

        self.investor_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    realized_cap,
                    &self.thermo_cap.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.vaulted_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    realized_cap,
                    &activity.vaultedness.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.active_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    realized_cap,
                    &activity.liveliness.height,
                    exit,
                )?;
                Ok(())
            })?;

        // cointime_cap = (cointime_value_destroyed_cumulative * circulating_supply) / coinblocks_stored_cumulative
        self.cointime_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform3(
                    starting_indexes.height,
                    value.cointime_value_destroyed.height_cumulative.inner(),
                    circulating_supply,
                    activity.coinblocks_stored.height_cumulative.inner(),
                    |(i, destroyed, supply, stored, ..)| {
                        let destroyed: f64 = *destroyed;
                        let supply: f64 = supply.into();
                        let stored: f64 = *stored;
                        (i, Dollars::from(destroyed * supply / stored))
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
