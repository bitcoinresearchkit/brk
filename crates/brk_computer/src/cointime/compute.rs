use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, blocks, distribution, mining, prices, supply};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        mining: &mining::Vecs,
        supply_vecs: &supply::Vecs,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        // Activity computes first (liveliness, vaultedness, etc.)
        self.activity
            .compute(starting_indexes, blocks, distribution, exit)?;

        // Supply computes next (depends on activity)
        self.supply.compute(
            starting_indexes,
            distribution,
            &self.activity,
            exit,
        )?;

        // Adjusted velocity metrics (BTC) - can compute without price
        self.adjusted
            .compute(starting_indexes, supply_vecs, &self.activity, exit)?;

        // Value computes (cointime value destroyed/created/stored, VOCDD)
        self.value.compute(
            starting_indexes,
            prices,
            blocks,
            distribution,
            &self.activity,
            exit,
        )?;

        // Cap computes (thermo, investor, vaulted, active, cointime caps)
        self.cap.compute(
            starting_indexes,
            mining,
            distribution,
            &self.activity,
            &self.value,
            exit,
        )?;

        // Pricing computes (all prices derived from caps)
        self.pricing.compute(
            starting_indexes,
            prices,
            blocks,
            distribution,
            &self.activity,
            &self.supply,
            &self.cap,
            exit,
        )?;

        // Reserve Risk computes (depends on value.vocdd and price)
        self.reserve_risk
            .compute(starting_indexes, blocks, prices, &self.value, exit)?;

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
