use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::{StoredF32, StoredF64};
use vecdb::Exit;

use super::Vecs;
use crate::{
    chain::{block, coinbase, ONE_TERA_HASH, TARGET_BLOCKS_PER_DAY_F64},
    indexes,
    utils::OptionExt,
    ComputeIndexes,
};

impl Vecs {
    pub fn compute(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        block_vecs: &block::Vecs,
        coinbase_vecs: &coinbase::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_difficulty.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&indexer.vecs.block.height_to_difficulty),
        )?;

        self.indexes_to_difficulty_as_hash
            .compute_all(indexes, starting_indexes, exit, |v| {
                let multiplier = 2.0_f64.powi(32) / 600.0;
                v.compute_transform(
                    starting_indexes.height,
                    &indexer.vecs.block.height_to_difficulty,
                    |(i, v, ..)| (i, StoredF32::from(*v * multiplier)),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &block_vecs.height_to_24h_block_count,
                    self.indexes_to_difficulty_as_hash.height.u(),
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

        self.indexes_to_hash_rate_1w_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_2m_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    2 * 30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_rate_1y_sma
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sma(
                    starting_indexes.dateindex,
                    self.indexes_to_hash_rate.dateindex.unwrap_last(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_difficulty_adjustment.compute_all(
            indexes,
            starting_indexes,
            exit,
            |v| {
                v.compute_percentage_change(
                    starting_indexes.height,
                    &indexer.vecs.block.height_to_difficulty,
                    1,
                    exit,
                )?;
                Ok(())
            },
        )?;

        self.indexes_to_hash_price_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &coinbase_vecs.height_to_24h_coinbase_usd_sum,
                    self.indexes_to_hash_rate.height.u(),
                    |(i, coinbase_sum, hashrate, ..)| {
                        (i, (*coinbase_sum / (*hashrate / ONE_TERA_HASH)).into())
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_hash_price_ths.height.u(),
                    |(i, price, ..)| (i, (*price * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_ths
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform2(
                    starting_indexes.height,
                    &coinbase_vecs.height_to_24h_coinbase_sum,
                    self.indexes_to_hash_rate.height.u(),
                    |(i, coinbase_sum, hashrate, ..)| {
                        (
                            i,
                            (*coinbase_sum as f64 / (*hashrate / ONE_TERA_HASH)).into(),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_phs
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_transform(
                    starting_indexes.height,
                    self.indexes_to_hash_value_ths.height.u(),
                    |(i, value, ..)| (i, (*value * 1000.0).into()),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_price_ths.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_price_phs.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_ths_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_value_ths.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_phs_min
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_all_time_low_(
                    starting_indexes.height,
                    self.indexes_to_hash_value_phs.height.u(),
                    exit,
                    true,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_price_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    self.indexes_to_hash_price_phs.height.u(),
                    self.indexes_to_hash_price_phs_min.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_hash_value_rebound
            .compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage_difference(
                    starting_indexes.height,
                    self.indexes_to_hash_value_phs.height.u(),
                    self.indexes_to_hash_value_phs_min.height.u(),
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
