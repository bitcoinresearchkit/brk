use brk_error::Result;
use brk_types::{DateIndex, StoredF64};
use vecdb::{Exit, TypedVecIterator};

use super::{super::value, Vecs};
use crate::{price, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        price: &price::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        // Get VOCDD dateindex sum data (from cointime/value module)
        // The dateindex.sum.0 contains daily VOCDD values as EagerVec
        let vocdd_dateindex_sum = &value.vocdd.dateindex.sum.0;

        // Compute 365-day SMA of VOCDD
        self.vocdd_365d_sma.compute_sma(
            starting_indexes.dateindex,
            vocdd_dateindex_sum,
            365,
            exit,
        )?;

        let price_close = &price.usd.split.close.dateindex;

        // Compute HODL Bank = cumulative sum of (price - vocdd_sma)
        // Start from where we left off and maintain cumulative state
        let starting_dateindex = starting_indexes.dateindex.to_usize().min(self.hodl_bank.len());
        let target_len = price_close.len().min(self.vocdd_365d_sma.len());

        if target_len > starting_dateindex {
            let mut price_iter = price_close.into_iter();
            let mut vocdd_sma_iter = self.vocdd_365d_sma.into_iter();

            // Get previous cumulative value, or start at 0
            let mut cumulative: f64 = if starting_dateindex > 0 {
                let prev_dateindex = DateIndex::from(starting_dateindex - 1);
                f64::from(*self.hodl_bank.into_iter().get_unwrap(prev_dateindex))
            } else {
                0.0
            };

            for i in starting_dateindex..target_len {
                let dateindex = DateIndex::from(i);
                let price_val = f64::from(*price_iter.get_unwrap(dateindex));
                let vocdd_sma = f64::from(*vocdd_sma_iter.get_unwrap(dateindex));

                // HODL Bank contribution: price - smoothed VOCDD
                // Accumulate over time
                cumulative += price_val - vocdd_sma;
                self.hodl_bank
                    .truncate_push(dateindex, StoredF64::from(cumulative))?;

                exit.check()?;
            }
            self.hodl_bank.write()?;
        }

        // Compute Reserve Risk = price / hodl_bank (if enabled)
        if let Some(reserve_risk) = self.reserve_risk.as_mut() {
            reserve_risk.compute_all(starting_indexes, exit, |v| {
                v.compute_divide(
                    starting_indexes.dateindex,
                    price_close,
                    &self.hodl_bank,
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
