use brk_error::Result;
use brk_types::{DateIndex, StoredF64};
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec, IterableVec, VecIndex};

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
        let starting_dateindex = starting_indexes
            .dateindex
            .to_usize()
            .min(self.hodl_bank.len());
        let target_len = price_close.len().min(self.vocdd_365d_sma.len());

        if target_len > starting_dateindex {
            let mut price_iter = price_close.iter();
            let mut vocdd_sma_iter = self.vocdd_365d_sma.iter();

            // Get previous cumulative value, or start at 0
            let mut cumulative: f64 = if starting_dateindex > 0 {
                let prev_dateindex = DateIndex::from(starting_dateindex - 1);
                f64::from(*self.hodl_bank.iter().get_unwrap(prev_dateindex))
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
                    .truncate_push_at(i, StoredF64::from(cumulative))?;
            }

            let _lock = exit.lock();
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the HODL Bank cumulative formula
    /// HODL Bank[n] = HODL Bank[n-1] + (price[n] - vocdd_sma[n])
    #[test]
    fn test_hodl_bank_formula() {
        // Simulate daily data
        let prices = [100.0, 110.0, 105.0, 120.0, 115.0];
        let vocdd_sma = [50.0, 55.0, 52.0, 60.0, 58.0];

        let mut hodl_bank = 0.0_f64;
        let mut expected = Vec::new();

        for i in 0..prices.len() {
            // HODL Bank contribution: price - vocdd_sma
            hodl_bank += prices[i] - vocdd_sma[i];
            expected.push(hodl_bank);
        }

        // Expected values:
        // Day 0: 0 + (100 - 50) = 50
        // Day 1: 50 + (110 - 55) = 105
        // Day 2: 105 + (105 - 52) = 158
        // Day 3: 158 + (120 - 60) = 218
        // Day 4: 218 + (115 - 58) = 275
        assert!((expected[0] - 50.0).abs() < 0.001);
        assert!((expected[1] - 105.0).abs() < 0.001);
        assert!((expected[2] - 158.0).abs() < 0.001);
        assert!((expected[3] - 218.0).abs() < 0.001);
        assert!((expected[4] - 275.0).abs() < 0.001);
    }

    /// Test the Reserve Risk formula
    /// Reserve Risk = price / HODL Bank
    #[test]
    fn test_reserve_risk_formula() {
        let price = 100.0_f64;
        let hodl_bank = 1000.0_f64;

        let reserve_risk = price / hodl_bank;

        // Reserve Risk = 100 / 1000 = 0.1
        assert!((reserve_risk - 0.1).abs() < 0.0001);
    }

    /// Test that low Reserve Risk indicates buying opportunity
    /// (high HODL Bank relative to price)
    #[test]
    fn test_reserve_risk_interpretation() {
        // High HODL Bank (long-term holder confidence) = low Reserve Risk
        let high_confidence = 100.0 / 10000.0; // 0.01

        // Low HODL Bank (low confidence) = high Reserve Risk
        let low_confidence = 100.0 / 100.0; // 1.0

        assert!(high_confidence < low_confidence);
        assert!(high_confidence < 0.05); // Good buying opportunity
        assert!(low_confidence > 0.5); // Overheated market
    }

    /// Test HODL Bank accumulation with negative contributions
    /// When VOCDD_SMA > Price, HODL Bank decreases
    #[test]
    fn test_hodl_bank_negative_contribution() {
        let prices = [100.0, 80.0, 90.0]; // Price drops
        let vocdd_sma = [50.0, 100.0, 85.0]; // VOCDD_SMA rises then normalizes

        let mut hodl_bank = 0.0_f64;

        for i in 0..prices.len() {
            hodl_bank += prices[i] - vocdd_sma[i];
        }

        // Day 0: 0 + (100 - 50) = 50
        // Day 1: 50 + (80 - 100) = 30 (decreases when vocdd_sma > price)
        // Day 2: 30 + (90 - 85) = 35
        assert!((hodl_bank - 35.0).abs() < 0.001);
    }
}
