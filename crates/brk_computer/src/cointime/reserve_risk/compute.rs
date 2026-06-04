use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::StoredF64;
use vecdb::Exit;

use super::{super::value, Vecs};
use crate::{blocks, internal::algo::ComputeRollingMedianFromStarts, price};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        blocks: &blocks::Vecs,
        prices: &price::Vecs,
        value: &value::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;

        self.vocdd_median_1y.compute_rolling_median_from_starts(
            starting_height,
            &blocks.lookback._1y,
            &value.vocdd.block,
            exit,
        )?;

        self.hodl_bank.compute_cumulative_transformed_binary(
            starting_height,
            &prices.spot.usd.height,
            &self.vocdd_median_1y,
            |price, median| StoredF64::from(f64::from(price) - f64::from(median)),
            exit,
        )?;

        self.value.height.compute_divide(
            starting_height,
            &prices.spot.usd.height,
            &self.hodl_bank,
            exit,
        )?;

        Ok(())
    }
}
