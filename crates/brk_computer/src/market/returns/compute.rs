use brk_error::Result;
use brk_types::{BasisPointsSigned32, Dollars, Indexes, StoredF32};
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, internal::RatioDiffDollarsBps32, market::lookback, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        lookback: &lookback::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        // Compute price returns at height level
        for ((returns, _), (lookback_price, _)) in self
            .price_return
            .iter_mut_with_days()
            .zip(lookback.price_lookback.iter_with_days())
        {
            returns.compute_binary::<Dollars, Dollars, RatioDiffDollarsBps32>(
                starting_indexes.height,
                &prices.price.usd.height,
                &lookback_price.usd.height,
                exit,
            )?;
        }

        // CAGR computed from returns at height level (2y+ periods only)
        let price_return_dca = self.price_return.as_dca_period();
        for (cagr, returns, days) in self.price_cagr.zip_mut_with_period(&price_return_dca) {
            let years = days as f64 / 365.0;
            cagr.bps.height.compute_transform(
                starting_indexes.height,
                &returns.bps.height,
                |(h, r, ..)| {
                    let ratio = f64::from(r);
                    let v = (ratio + 1.0).powf(1.0 / years) - 1.0;
                    (h, BasisPointsSigned32::from(v))
                },
                exit,
            )?;
        }

        let _24h_price_return_ratio = &self.price_return._24h.ratio.height;

        for sd in [
            &mut self.price_return_24h_sd_1w,
            &mut self.price_return_24h_sd_1m,
            &mut self.price_return_24h_sd_1y,
        ] {
            sd.compute_all(blocks, starting_indexes, exit, _24h_price_return_ratio)?;
        }

        // Downside returns: min(return, 0)
        self.price_downside_24h.compute_transform(
            starting_indexes.height,
            _24h_price_return_ratio,
            |(i, ret, ..)| {
                let v = f32::from(ret).min(0.0);
                (i, StoredF32::from(v))
            },
            exit,
        )?;

        // Downside deviation (SD of downside returns)
        for sd in [
            &mut self.price_downside_24h_sd_1w,
            &mut self.price_downside_24h_sd_1m,
            &mut self.price_downside_24h_sd_1y,
        ] {
            sd.compute_all(blocks, starting_indexes, exit, &self.price_downside_24h)?;
        }

        Ok(())
    }
}
