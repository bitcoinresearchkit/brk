use brk_error::Result;
use brk_types::{Indexes, StoredF64};
use vecdb::Exit;

use super::{super::value, Vecs};
use crate::{blocks, internal::algo::ComputeRollingMedianFromStarts, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.vocdd_median_1y.compute_rolling_median_from_starts(
            starting_indexes.height,
            &blocks.lookback._1y,
            &value.vocdd.block,
            exit,
        )?;

        self.hodl_bank.compute_cumulative_transformed_binary(
            starting_indexes.height,
            &prices.cached_spot_usd,
            &self.vocdd_median_1y,
            |price, median| StoredF64::from(f64::from(price) - f64::from(median)),
            exit,
        )?;

        self.value.height.compute_divide(
            starting_indexes.height,
            &prices.cached_spot_usd,
            &self.hodl_bank,
            exit,
        )?;

        Ok(())
    }
}
