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
        self.hash_rate.height.compute_transform2(
            starting_indexes.height,
            &count_vecs.block_count.sum._24h.height,
            &difficulty_vecs.as_hash.height,
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

        let hash_rate = &self.hash_rate.height;
        for (sma, window) in [
            (&mut self.hash_rate_sma_1w.height, &lookback.height_1w_ago),
            (&mut self.hash_rate_sma_1m.height, &lookback.height_1m_ago),
            (&mut self.hash_rate_sma_2m.height, &lookback.height_2m_ago),
            (&mut self.hash_rate_sma_1y.height, &lookback.height_1y_ago),
        ] {
            sma.compute_rolling_average(starting_indexes.height, window, hash_rate, exit)?;
        }

        self.hash_rate_ath.height.compute_all_time_high(
            starting_indexes.height,
            &self.hash_rate.height,
            exit,
        )?;

        self.hash_rate_drawdown.compute_drawdown(
            starting_indexes.height,
            &self.hash_rate.height,
            &self.hash_rate_ath.height,
            exit,
        )?;

        self.hash_price_ths.height.compute_transform2(
            starting_indexes.height,
            coinbase_usd_24h_sum,
            &self.hash_rate.height,
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

        self.hash_price_phs.height.compute_transform(
            starting_indexes.height,
            &self.hash_price_ths.height,
            |(i, price, ..)| (i, (*price * 1000.0).into()),
            exit,
        )?;

        self.hash_value_ths.height.compute_transform2(
            starting_indexes.height,
            coinbase_sats_24h_sum,
            &self.hash_rate.height,
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

        self.hash_value_phs.height.compute_transform(
            starting_indexes.height,
            &self.hash_value_ths.height,
            |(i, value, ..)| (i, (*value * 1000.0).into()),
            exit,
        )?;

        for (min_vec, src_vec) in [
            (
                &mut self.hash_price_ths_min.height,
                &self.hash_price_ths.height,
            ),
            (
                &mut self.hash_price_phs_min.height,
                &self.hash_price_phs.height,
            ),
            (
                &mut self.hash_value_ths_min.height,
                &self.hash_value_ths.height,
            ),
            (
                &mut self.hash_value_phs_min.height,
                &self.hash_value_phs.height,
            ),
        ] {
            min_vec.compute_all_time_low_(starting_indexes.height, src_vec, exit, true)?;
        }

        self.hash_price_rebound
            .compute_binary::<StoredF32, StoredF32, RatioDiffF32Bps32>(
                starting_indexes.height,
                &self.hash_price_phs.height,
                &self.hash_price_phs_min.height,
                exit,
            )?;

        self.hash_value_rebound
            .compute_binary::<StoredF32, StoredF32, RatioDiffF32Bps32>(
                starting_indexes.height,
                &self.hash_value_phs.height,
                &self.hash_value_phs_min.height,
                exit,
            )?;

        Ok(())
    }
}
