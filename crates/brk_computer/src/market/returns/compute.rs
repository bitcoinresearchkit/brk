use brk_error::Result;
use brk_types::{Dollars, StoredF32};
use vecdb::{Exit, ReadableOptionVec};

use super::Vecs;
use crate::{ComputeIndexes, blocks, indexes, internal::PercentageDiffDollars, market::lookback, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        lookback: &lookback::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        // Compute price returns at height level
        for ((returns, _), (lookback_price, _)) in self
            .price_returns
            .iter_mut_with_days()
            .zip(lookback.price_ago.iter_with_days())
        {
            returns.compute_binary::<Dollars, Dollars, PercentageDiffDollars>(
                starting_indexes.height,
                &prices.price.usd,
                &lookback_price.usd.height,
                exit,
            )?;
        }

        // CAGR computed from returns (2y+ periods only)
        let h2d = &indexes.height.day1;
        let price_returns_dca = self.price_returns.as_dca_period();
        for (cagr, returns, days) in self.cagr.zip_mut_with_period(&price_returns_dca) {
            let years = days as f32 / 365.0;
            let mut cached_di = None;
            let mut cached_val = StoredF32::from(0.0);
            cagr.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    if cached_di != Some(di) {
                        cached_di = Some(di);
                        cached_val = StoredF32::from(
                            returns.day1
                                .collect_one_flat(di)
                                .map(|r| ((*r / 100.0 + 1.0).powf(1.0 / years) - 1.0) * 100.0)
                                .unwrap_or(0.0)
                        );
                    }
                    (h, cached_val)
                },
                exit,
            )?;
        }

        let _1d_price_returns_height = &self.price_returns._1d.height;

        self._1d_returns_1w_sd
            .compute_all(blocks, starting_indexes, exit, _1d_price_returns_height)?;
        self._1d_returns_1m_sd
            .compute_all(blocks, starting_indexes, exit, _1d_price_returns_height)?;
        self._1d_returns_1y_sd
            .compute_all(blocks, starting_indexes, exit, _1d_price_returns_height)?;

        // Downside returns: min(return, 0)
        self.downside_returns.compute_transform(
            starting_indexes.height,
            _1d_price_returns_height,
            |(i, ret, ..)| (i, StoredF32::from((*ret).min(0.0))),
            exit,
        )?;

        // Downside deviation (SD of downside returns)
        self.downside_1w_sd
            .compute_all(blocks, starting_indexes, exit, &self.downside_returns)?;
        self.downside_1m_sd
            .compute_all(blocks, starting_indexes, exit, &self.downside_returns)?;
        self.downside_1y_sd
            .compute_all(blocks, starting_indexes, exit, &self.downside_returns)?;

        Ok(())
    }
}
