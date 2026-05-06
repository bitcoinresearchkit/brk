use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Dollars;
use vecdb::Exit;

use super::super::{activity, value};
use super::Vecs;
use crate::{distribution, mining};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        mining: &mining::Vecs,
        distribution: &distribution::Vecs,
        activity: &activity::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_lengths = indexer.safe_lengths();
        let all_metrics = &distribution.utxo_cohorts.all.metrics;
        let realized_cap_cents = &all_metrics.realized.cap.cents.height;
        let circulating_supply = &all_metrics.supply.total.btc.height;

        self.thermo.cents.height.compute_transform(
            starting_lengths.height,
            &mining.rewards.subsidy.cumulative.cents.height,
            |(i, v, ..)| (i, v),
            exit,
        )?;

        self.investor.cents.height.compute_subtract(
            starting_lengths.height,
            realized_cap_cents,
            &self.thermo.cents.height,
            exit,
        )?;

        self.vaulted.cents.height.compute_multiply(
            starting_lengths.height,
            realized_cap_cents,
            &activity.vaultedness.height,
            exit,
        )?;

        self.active.cents.height.compute_multiply(
            starting_lengths.height,
            realized_cap_cents,
            &activity.liveliness.height,
            exit,
        )?;

        // cointime_cap = (cointime_value_destroyed_cumulative * circulating_supply) / coinblocks_stored_cumulative
        self.cointime.cents.height.compute_transform3(
            starting_lengths.height,
            &value.destroyed.cumulative.height,
            circulating_supply,
            &activity.coinblocks_stored.cumulative.height,
            |(i, destroyed, supply, stored, ..)| {
                let destroyed: f64 = *destroyed;
                let supply: f64 = supply.into();
                let stored: f64 = *stored;
                let usd = Dollars::from(destroyed * supply / stored);
                (i, usd.to_cents())
            },
            exit,
        )?;

        // AVIV = active_cap / investor_cap
        self.aviv.compute_ratio(
            &starting_lengths,
            &self.active.cents.height,
            &self.investor.cents.height,
            exit,
        )?;

        Ok(())
    }
}
