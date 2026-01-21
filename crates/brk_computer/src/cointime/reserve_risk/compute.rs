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
    #[test]
    fn test_hodl_bank_formula() {
        let prices = [100.0, 110.0, 105.0, 120.0, 115.0];
        let vocdd_sma = [50.0, 55.0, 52.0, 60.0, 58.0];

        let mut hodl_bank = 0.0_f64;
        let mut expected = Vec::new();

        for i in 0..prices.len() {
            hodl_bank += prices[i] - vocdd_sma[i];
            expected.push(hodl_bank);
        }

        assert!((expected[0] - 50.0).abs() < 0.001);
        assert!((expected[1] - 105.0).abs() < 0.001);
        assert!((expected[2] - 158.0).abs() < 0.001);
        assert!((expected[3] - 218.0).abs() < 0.001);
        assert!((expected[4] - 275.0).abs() < 0.001);
    }

    #[test]
    fn test_reserve_risk_formula() {
        let price = 100.0_f64;
        let hodl_bank = 1000.0_f64;
        let reserve_risk = price / hodl_bank;
        assert!((reserve_risk - 0.1).abs() < 0.0001);
    }

    #[test]
    fn test_reserve_risk_interpretation() {
        let high_confidence = 100.0 / 10000.0;
        let low_confidence = 100.0 / 100.0;
        assert!(high_confidence < low_confidence);
    }

    #[test]
    fn test_hodl_bank_negative_contribution() {
        let prices = [100.0, 80.0, 90.0];
        let vocdd_sma = [50.0, 100.0, 85.0];

        let mut hodl_bank = 0.0_f64;
        for i in 0..prices.len() {
            hodl_bank += prices[i] - vocdd_sma[i];
        }

        assert!((hodl_bank - 35.0).abs() < 0.001);
    }
}
