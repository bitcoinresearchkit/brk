use brk_error::Result;
use brk_types::{Dollars, Height, Indexes, Sats, StoredF32, StoredF64};
use vecdb::{Exit, ReadableVec};

use super::Vecs;
use crate::{
    blocks::{self, ONE_TERA_HASH, TARGET_BLOCKS_PER_DAY_F64},
    internal::RatioDiffF32Bps32,
};

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        count_vecs: &blocks::CountVecs,
        lookback: &blocks::LookbackVecs,
        difficulty_vecs: &blocks::DifficultyVecs,
        coinbase_sats_24h_sum: &impl ReadableVec<Height, Sats>,
        coinbase_usd_24h_sum: &impl ReadableVec<Height, Dollars>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rate.base.height.compute_transform2(
            starting_indexes.height,
            &count_vecs.total.sum._24h.height,
            &difficulty_vecs.hashrate.height,
            |(i, block_count_sum, difficulty_as_hash, ..)| {
                (
                    i,
                    StoredF64::from(
                        (f64::from(block_count_sum) / TARGET_BLOCKS_PER_DAY_F64)
                            * f64::from(difficulty_as_hash),
                    ),
                )
            },
            exit,
        )?;

        let hash_rate = &self.rate.base.height;
        for (sma, window) in [
            (&mut self.rate.sma._1w.height, &lookback._1w.inner),
            (&mut self.rate.sma._1m.height, &lookback._1m.inner),
            (&mut self.rate.sma._2m.height, &lookback._2m),
            (&mut self.rate.sma._1y.height, &lookback._1y.inner),
        ] {
            sma.compute_rolling_average(starting_indexes.height, window, hash_rate, exit)?;
        }

        self.rate.ath.height.compute_all_time_high(
            starting_indexes.height,
            &self.rate.base.height,
            exit,
        )?;

        self.rate.drawdown.compute_drawdown(
            starting_indexes.height,
            &self.rate.base.height,
            &self.rate.ath.height,
            exit,
        )?;

        self.price.ths.height.compute_transform2(
            starting_indexes.height,
            coinbase_usd_24h_sum,
            &self.rate.base.height,
            |(i, coinbase_sum, hashrate, ..)| {
                let hashrate_ths = *hashrate / ONE_TERA_HASH;
                let price = if hashrate_ths == 0.0 {
                    StoredF32::NAN
                } else {
                    (*coinbase_sum / hashrate_ths).into()
                };
                (i, price)
            },
            exit,
        )?;

        self.value.ths.height.compute_transform2(
            starting_indexes.height,
            coinbase_sats_24h_sum,
            &self.rate.base.height,
            |(i, coinbase_sum, hashrate, ..)| {
                let hashrate_ths = *hashrate / ONE_TERA_HASH;
                let value = if hashrate_ths == 0.0 {
                    StoredF32::NAN
                } else {
                    StoredF32::from(*coinbase_sum as f64 / hashrate_ths)
                };
                (i, value)
            },
            exit,
        )?;

        for (min_vec, src_vec) in [
            (&mut self.price.ths_min.height, &self.price.ths.height),
            (&mut self.value.ths_min.height, &self.value.ths.height),
        ] {
            min_vec.compute_all_time_low_(starting_indexes.height, src_vec, exit, true)?;
        }

        self.price
            .rebound
            .compute_binary::<StoredF32, StoredF32, RatioDiffF32Bps32>(
                starting_indexes.height,
                &self.price.phs.height,
                &self.price.phs_min.height,
                exit,
            )?;

        self.value
            .rebound
            .compute_binary::<StoredF32, StoredF32, RatioDiffF32Bps32>(
                starting_indexes.height,
                &self.value.phs.height,
                &self.value.phs_min.height,
                exit,
            )?;

        Ok(())
    }
}
