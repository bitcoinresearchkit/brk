use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, distribution, indexes, price, supply, ComputeIndexes};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        price: Option<&price::Vecs>,
        blocks: &blocks::Vecs,
        supply_vecs: &supply::Vecs,
        distribution: &distribution::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        // Activity computes first (liveliness, vaultedness, etc.)
        self.activity
            .compute(indexes, starting_indexes, distribution, exit)?;

        // Supply computes next (depends on activity)
        self.supply.compute(
            indexes,
            starting_indexes,
            distribution,
            &self.activity,
            exit,
        )?;

        // Adjusted velocity metrics (BTC) - can compute without price
        self.adjusted.compute(
            starting_indexes,
            supply_vecs,
            &self.activity,
            price.is_some(),
            exit,
        )?;

        // Price-dependent metrics
        if let Some(price) = price {
            // Value computes (cointime value destroyed/created/stored)
            self.value.compute(
                indexes,
                starting_indexes,
                price,
                distribution,
                &self.activity,
                exit,
            )?;

            // Cap computes (thermo, investor, vaulted, active, cointime caps)
            self.cap.compute(
                indexes,
                starting_indexes,
                blocks,
                distribution,
                &self.activity,
                &self.value,
                exit,
            )?;

            // Pricing computes (all prices derived from caps)
            self.pricing.compute(
                indexes,
                starting_indexes,
                price,
                distribution,
                &self.activity,
                &self.supply,
                &self.cap,
                exit,
            )?;
        }

        let _lock = exit.lock();
        self.db.compact()?;
        Ok(())
    }
}
