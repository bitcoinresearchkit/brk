use brk_error::Result;
use brk_types::{Cents, Indexes};
use vecdb::Exit;

use super::super::{activity, value};
use super::Vecs;
use crate::{distribution, mining};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        mining: &mining::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let realized_cap_cents = &all_metrics.realized.cap_cents.height;
        let circulating_supply = &all_metrics.supply.total.btc.height;

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

        self.vaulted_cap.cents.height.compute_multiply(
            starting_indexes.height,
            realized_cap_cents,
            &activity.vaultedness.height,
            exit,
        )?;

        self.active_cap.cents.height.compute_multiply(
            starting_indexes.height,
            realized_cap_cents,
            &activity.liveliness.height,
            exit,
        )?;

        // cointime_cap = (cointime_value_destroyed_cumulative * circulating_supply) / coinblocks_stored_cumulative
        self.cointime_cap.cents.height.compute_transform3(
            starting_indexes.height,
            &value.value_destroyed.cumulative.height,
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
