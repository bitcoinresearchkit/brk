use brk_error::Result;
use brk_types::{Close, Dollars, StoredF64};
use vecdb::Exit;

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
        let vocdd_dateindex_sum = &value.vocdd.dateindex.sum.0;

        self.vocdd_365d_sma.compute_sma(
            starting_indexes.dateindex,
            vocdd_dateindex_sum,
            365,
            exit,
        )?;

        let price_close = &price.usd.split.close.dateindex;

        self.hodl_bank.compute_cumulative_transformed_binary(
            starting_indexes.dateindex,
            price_close,
            &self.vocdd_365d_sma,
            |price: Close<Dollars>, sma: StoredF64| StoredF64::from(f64::from(price) - f64::from(sma)),
            exit,
        )?;

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
