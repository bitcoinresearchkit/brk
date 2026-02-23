use brk_error::Result;
use brk_types::{StoredF32, StoredF64};
use vecdb::Exit;

use super::Vecs;
use crate::{
    blocks::{self, ONE_TERA_HASH, TARGET_BLOCKS_PER_DAY_F64},
    internal::StoredValueFromHeightLast,
    ComputeIndexes,
    traits::ComputeDrawdown,
};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        count_vecs: &blocks::CountVecs,
        difficulty_vecs: &blocks::DifficultyVecs,
        coinbase_sum_24h: &StoredValueFromHeightLast,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.hash_rate.height.compute_transform2(
            starting_indexes.height,
            &count_vecs.block_count_24h_sum.height,
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

        self.hash_rate_1w_sma.height.compute_rolling_average(
            starting_indexes.height,
            &count_vecs.height_1w_ago,
            &self.hash_rate.height,
            exit,
        )?;

        self.hash_rate_1m_sma.height.compute_rolling_average(
            starting_indexes.height,
            &count_vecs.height_1m_ago,
            &self.hash_rate.height,
            exit,
        )?;

        self.hash_rate_2m_sma.height.compute_rolling_average(
            starting_indexes.height,
            &count_vecs.height_2m_ago,
            &self.hash_rate.height,
            exit,
        )?;

        self.hash_rate_1y_sma.height.compute_rolling_average(
            starting_indexes.height,
            &count_vecs.height_1y_ago,
            &self.hash_rate.height,
            exit,
        )?;

        self.hash_rate_ath.height.compute_all_time_high(
            starting_indexes.height,
            &self.hash_rate.height,
            exit,
        )?;

        self.hash_rate_drawdown.height.compute_drawdown(
            starting_indexes.height,
            &self.hash_rate.height,
            &self.hash_rate_ath.height,
            exit,
        )?;

        self.hash_price_ths.height.compute_transform2(
            starting_indexes.height,
            &coinbase_sum_24h.usd.height,
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
            &coinbase_sum_24h.sats.height,
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

        self.hash_price_ths_min.height.compute_all_time_low_(
            starting_indexes.height,
            &self.hash_price_ths.height,
            exit,
            true,
        )?;

        self.hash_price_phs_min.height.compute_all_time_low_(
            starting_indexes.height,
            &self.hash_price_phs.height,
            exit,
            true,
        )?;

        self.hash_value_ths_min.height.compute_all_time_low_(
            starting_indexes.height,
            &self.hash_value_ths.height,
            exit,
            true,
        )?;

        self.hash_value_phs_min.height.compute_all_time_low_(
            starting_indexes.height,
            &self.hash_value_phs.height,
            exit,
            true,
        )?;

        self.hash_price_rebound
            .height
            .compute_percentage_difference(
                starting_indexes.height,
                &self.hash_price_phs.height,
                &self.hash_price_phs_min.height,
                exit,
            )?;

        self.hash_value_rebound
            .height
            .compute_percentage_difference(
                starting_indexes.height,
                &self.hash_value_phs.height,
                &self.hash_value_phs_min.height,
                exit,
            )?;

        Ok(())
    }
}
