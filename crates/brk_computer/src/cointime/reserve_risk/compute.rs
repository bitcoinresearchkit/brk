use brk_error::Result;
use brk_types::StoredF64;
use vecdb::Exit;

use super::{super::value, Vecs};
use crate::{blocks, ComputeIndexes, prices, traits::ComputeRollingMedianFromStarts};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.vocdd_365d_median.compute_rolling_median_from_starts(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &value.vocdd.height,
            exit,
        )?;

        self.hodl_bank.compute_cumulative_transformed_binary(
            starting_indexes.height,
            &prices.usd.price,
            &self.vocdd_365d_median,
            |price, median| StoredF64::from(f64::from(price) - f64::from(median)),
            exit,
        )?;

        self.reserve_risk.height.compute_divide(
            starting_indexes.height,
            &prices.usd.price,
            &self.hodl_bank,
            exit,
        )?;

        Ok(())
    }
}
