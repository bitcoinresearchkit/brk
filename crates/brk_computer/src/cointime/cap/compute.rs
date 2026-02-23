use brk_error::Result;
use brk_types::Dollars;
use vecdb::Exit;

use super::super::{activity, value};
use super::Vecs;
use crate::{ComputeIndexes, distribution, mining};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        mining: &mining::Vecs,
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
            .realized_cap
            .height;

        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .btc
            .height;

        self.thermo_cap.height.compute_transform(
            starting_indexes.height,
            &*mining.rewards.subsidy.usd.height_cumulative,
            |(i, v, ..)| (i, v),
            exit,
        )?;

        self.investor_cap.height.compute_subtract(
            starting_indexes.height,
            realized_cap,
            &self.thermo_cap.height,
            exit,
        )?;

        self.vaulted_cap.height.compute_divide(
            starting_indexes.height,
            realized_cap,
            &activity.vaultedness.height,
            exit,
        )?;

        self.active_cap.height.compute_multiply(
            starting_indexes.height,
            realized_cap,
            &activity.liveliness.height,
            exit,
        )?;

        // cointime_cap = (cointime_value_destroyed_cumulative * circulating_supply) / coinblocks_stored_cumulative
        self.cointime_cap.height.compute_transform3(
            starting_indexes.height,
            &value.cointime_value_destroyed.height_cumulative.0,
            circulating_supply,
            &activity.coinblocks_stored.height_cumulative.0,
            |(i, destroyed, supply, stored, ..)| {
                let destroyed: f64 = *destroyed;
                let supply: f64 = supply.into();
                let stored: f64 = *stored;
                (i, Dollars::from(destroyed * supply / stored))
            },
            exit,
        )?;

        Ok(())
    }
}
