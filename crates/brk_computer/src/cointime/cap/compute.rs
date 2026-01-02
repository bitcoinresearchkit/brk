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
            .height_to_realized_cap;

        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .height_to_supply_value
            .bitcoin;

        self.indexes_to_thermo_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    blocks
                        .rewards
                        .indexes_to_subsidy
                        .dollars
                        .as_ref()
                        .unwrap()
                        .height_extra
                        .unwrap_cumulative(),
                    |(i, v, ..)| (i, v),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_investor_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.height,
                    realized_cap,
                    self.indexes_to_thermo_cap.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_vaulted_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_divide(
                    starting_indexes.height,
                    realized_cap,
                    activity.indexes_to_vaultedness.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_active_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_multiply(
                    starting_indexes.height,
                    realized_cap,
                    activity.indexes_to_liveliness.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        // cointime_cap = (cointime_value_destroyed_cumulative * circulating_supply) / coinblocks_stored_cumulative
        self.indexes_to_cointime_cap
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform3(
                    starting_indexes.height,
                    value
                        .indexes_to_cointime_value_destroyed
                        .height_extra
                        .unwrap_cumulative(),
                    circulating_supply,
                    activity
                        .indexes_to_coinblocks_stored
                        .height_extra
                        .unwrap_cumulative(),
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
