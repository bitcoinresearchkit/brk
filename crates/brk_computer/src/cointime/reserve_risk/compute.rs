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

        self.vocdd_365d_median.compute_rolling_median(
            starting_indexes.dateindex,
            vocdd_dateindex_sum,
            365,
            exit,
        )?;

        let price_close = &price.usd.split.close.dateindex;

        self.hodl_bank.compute_cumulative_transformed_binary(
            starting_indexes.dateindex,
            price_close,
            &self.vocdd_365d_median,
            |price: Close<Dollars>, median: StoredF64| StoredF64::from(f64::from(price) - f64::from(median)),
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
