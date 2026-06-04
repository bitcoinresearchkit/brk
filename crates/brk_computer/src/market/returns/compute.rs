use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{BasisPointsSigned32, Dollars};
use vecdb::Exit;

use super::Vecs;
use crate::{
    blocks, internal::RatioDiffDollarsBps32, investing::ByDcaPeriod, market::lookback, price,
};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        prices: &price::Vecs,
        blocks: &blocks::Vecs,
        lookback: &lookback::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_lengths = indexer.safe_lengths();

        // Compute price returns at height level
        for ((returns, _), (lookback_price, _)) in self
            .periods
            .iter_mut_with_days()
            .zip(lookback.price_past.iter_with_days())
        {
            returns.compute_binary::<Dollars, Dollars, RatioDiffDollarsBps32>(
                starting_lengths.height,
                &prices.spot.usd.height,
                &lookback_price.usd.height,
                exit,
            )?;
        }

        // CAGR computed from returns at height level (2y+ periods only)
        let price_return_dca = ByDcaPeriod::from_lookback(&self.periods);
        for (cagr, returns, days) in self.cagr.zip_mut_with_period(&price_return_dca) {
            let years = days as f64 / 365.0;
            cagr.bps.height.compute_transform(
                starting_lengths.height,
                &returns.bps.height,
                |(h, r, ..)| {
                    let ratio = f64::from(r);
                    let v = (ratio + 1.0).powf(1.0 / years) - 1.0;
                    (h, BasisPointsSigned32::from(v))
                },
                exit,
            )?;
        }

        let _24h_price_return_ratio = &self.periods._24h.ratio.height;

        for sd in self.sd_24h.as_mut_array() {
            sd.compute_all(blocks, &starting_lengths, exit, _24h_price_return_ratio)?;
        }

        Ok(())
    }
}
