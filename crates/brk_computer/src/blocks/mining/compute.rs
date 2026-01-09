use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredF32, StoredF64};
use vecdb::Exit;

use super::super::{ONE_TERA_HASH, TARGET_BLOCKS_PER_DAY_F64, count, rewards};
use super::Vecs;
use crate::{ComputeIndexes, indexes};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        count_vecs: &count::Vecs,
        rewards_vecs: &rewards::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.difficulty.derive_from(
            indexes,
            starting_indexes,
            &indexer.vecs.blocks.difficulty,
            exit,
        )?;

        self.difficulty_as_hash
            .compute_all(indexes, starting_indexes, exit, |v| {
                let multiplier = 2.0_f64.powi(32) / 600.0;
                v.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.blocks.difficulty,
                    |(i, v, ..)| (i, StoredF32::from(*v * multiplier)),
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_rate
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &count_vecs._24h_block_count.height,
                    &self.difficulty_as_hash.height,
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
                Ok(())
            })?;

        self.hash_rate_1w_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.hash_rate.dateindex.inner(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_rate_1m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.hash_rate.dateindex.inner(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_rate_2m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.hash_rate.dateindex.inner(),
                    2 * 30,
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_rate_1y_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.hash_rate.dateindex.inner(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.difficulty_adjustment
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_change(
                    starting_indexes.height,
                    &indexer.vecs.blocks.difficulty,
                    1,
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_price_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    rewards_vecs._24h_coinbase_sum.dollars.as_ref().unwrap(),
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
                Ok(())
            })?;

        self.hash_price_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.hash_price_ths.height,
                    |(i, price, ..)| (i, (*price * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_value_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &rewards_vecs._24h_coinbase_sum.sats,
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
                Ok(())
            })?;

        self.hash_value_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    &self.hash_value_ths.height,
                    |(i, value, ..)| (i, (*value * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_price_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    &self.hash_price_ths.height,
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.hash_price_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    &self.hash_price_phs.height,
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.hash_value_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    &self.hash_value_ths.height,
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.hash_value_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    &self.hash_value_phs.height,
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.hash_price_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    &self.hash_price_phs.height,
                    &self.hash_price_phs_min.height,
                    exit,
                )?;
                Ok(())
            })?;

        self.hash_value_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    &self.hash_value_phs.height,
                    &self.hash_value_phs_min.height,
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
