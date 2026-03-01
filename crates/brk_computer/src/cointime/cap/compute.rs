use brk_error::Result;
use brk_types::Cents;
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
        let realized_cap_cents = &distribution
            .utxo_cohorts
            .all
            .metrics
            .realized
            .realized_cap_cents
            .height;

        let circulating_supply = &distribution
            .utxo_cohorts
            .all
            .metrics
            .supply
            .total
            .btc
            .height;

        self.thermo_cap.cents.height.compute_transform(
            starting_indexes.height,
            &mining.rewards.subsidy.cumulative.cents.height,
            |(i, v, ..)| (i, v),
            exit,
        )?;

        self.investor_cap.cents.height.compute_subtract(
            starting_indexes.height,
            realized_cap_cents,
            &self.thermo_cap.cents.height,
            exit,
        )?;

        self.vaulted_cap.cents.height.compute_transform2(
            starting_indexes.height,
            realized_cap_cents,
            &activity.vaultedness.height,
            |(i, cap, vaultedness, ..)| {
                (i, Cents::from(f64::from(cap) / f64::from(vaultedness)))
            },
            exit,
        )?;

        self.active_cap.cents.height.compute_transform2(
            starting_indexes.height,
            realized_cap_cents,
            &activity.liveliness.height,
            |(i, cap, liveliness, ..)| {
                (i, Cents::from(f64::from(cap) * f64::from(liveliness)))
            },
            exit,
        )?;

        // cointime_cap = (cointime_value_destroyed_cumulative * circulating_supply) / coinblocks_stored_cumulative
        self.cointime_cap.cents.height.compute_transform3(
            starting_indexes.height,
            &value.cointime_value_destroyed.cumulative.height,
            circulating_supply,
            &activity.coinblocks_stored.cumulative.height,
            |(i, destroyed, supply, stored, ..)| {
                let destroyed: f64 = *destroyed;
                let supply: f64 = supply.into();
                let stored: f64 = *stored;
                (i, Cents::from(destroyed * supply / stored))
            },
            exit,
        )?;

        Ok(())
    }
}
