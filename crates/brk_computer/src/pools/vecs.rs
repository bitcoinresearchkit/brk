use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolId, Pools, Sats, StoredF32, StoredU16, StoredU32};
use vecdb::{Database, Exit, GenericStoredVec, IterableVec, VecIndex, Version};

use crate::{
    chain,
    grouped::{
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes::{self, Indexes},
    price,
    utils::OptionExt,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    id: PoolId,

    pub indexes_to_blocks_mined: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_1w_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1m_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1y_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_fee: ComputedValueVecsFromHeight,
    pub indexes_to_coinbase: ComputedValueVecsFromHeight,
    pub indexes_to_dominance: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_1d_dominance: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_1w_dominance: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_1m_dominance: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_1y_dominance: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_days_since_block: ComputedVecsFromDateIndex<StoredU16>,
}

impl Vecs {
    pub fn forced_import(
        db: &Database,
        id: PoolId,
        pools: &Pools,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let pool = pools.get(id);
        let name = pool.serialized_id();
        let suffix = |s: &str| format!("{name}_{s}");
        let compute_dollars = price.is_some();
        let version = parent_version + Version::ZERO;

        let last = VecBuilderOptions::default().add_last();
        let sum_cum = VecBuilderOptions::default().add_sum().add_cumulative();

        macro_rules! import_di {
            ($name:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    db, &suffix($name), Source::Compute, version, indexes, last.clone(),
                )?
            };
        }

        Ok(Self {
            id,
            indexes_to_blocks_mined: ComputedVecsFromHeight::forced_import(
                db, &suffix("blocks_mined"), Source::Compute, version, indexes, sum_cum.clone(),
            )?,
            indexes_to_1w_blocks_mined: import_di!("1w_blocks_mined"),
            indexes_to_1m_blocks_mined: import_di!("1m_blocks_mined"),
            indexes_to_1y_blocks_mined: import_di!("1y_blocks_mined"),
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                db, &suffix("subsidy"), Source::Compute, version, sum_cum.clone(), compute_dollars, indexes,
            )?,
            indexes_to_fee: ComputedValueVecsFromHeight::forced_import(
                db, &suffix("fee"), Source::Compute, version, sum_cum.clone(), compute_dollars, indexes,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                db, &suffix("coinbase"), Source::Compute, version, sum_cum, compute_dollars, indexes,
            )?,
            indexes_to_dominance: import_di!("dominance"),
            indexes_to_1d_dominance: import_di!("1d_dominance"),
            indexes_to_1w_dominance: import_di!("1w_dominance"),
            indexes_to_1m_dominance: import_di!("1m_dominance"),
            indexes_to_1y_dominance: import_di!("1y_dominance"),
            indexes_to_days_since_block: import_di!("days_since_block"),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        height_to_pool: &impl IterableVec<Height, PoolId>,
        chain: &chain::Vecs,
        price: Option<&price::Vecs>,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_blocks_mined
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    height_to_pool,
                    |(h, id, ..)| {
                        (
                            h,
                            if id == self.id {
                                StoredU32::ONE
                            } else {
                                StoredU32::ZERO
                            },
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1w_blocks_mined
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.unwrap_sum(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_blocks_mined
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.unwrap_sum(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_blocks_mined
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.unwrap_sum(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        let height_to_blocks_mined = self.indexes_to_blocks_mined.height.u();

        self.indexes_to_subsidy
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    height_to_blocks_mined,
                    chain.indexes_to_subsidy.sats.height.u(),
                    |(h, mined, sats, ..)| {
                        (
                            h,
                            if mined == StoredU32::ONE {
                                sats
                            } else {
                                Sats::ZERO
                            },
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_fee
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    height_to_blocks_mined,
                    chain.indexes_to_fee.sats.height.unwrap_sum(),
                    |(h, mined, sats, ..)| {
                        (
                            h,
                            if mined == StoredU32::ONE {
                                sats
                            } else {
                                Sats::ZERO
                            },
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_coinbase
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    height_to_blocks_mined,
                    chain.indexes_to_coinbase.sats.height.u(),
                    |(h, mined, sats, ..)| {
                        (
                            h,
                            if mined == StoredU32::ONE {
                                sats
                            } else {
                                Sats::ZERO
                            },
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.unwrap_cumulative(),
                    chain.indexes_to_block_count.dateindex.unwrap_cumulative(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1d_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.unwrap_sum(),
                    chain.indexes_to_block_count.dateindex.unwrap_sum(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1w_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_1w_blocks_mined.dateindex.u(),
                    chain.indexes_to_1w_block_count.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_1m_blocks_mined.dateindex.u(),
                    chain.indexes_to_1m_block_count.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_1y_blocks_mined.dateindex.u(),
                    chain.indexes_to_1y_block_count.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_since_block
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.unwrap_sum(),
                    self.indexes_to_blocks_mined.dateindex.unwrap_cumulative(),
                    |(i, sum, cumulative, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
                            } else {
                                StoredU16::ZERO
                            });
                        }
                        let days = if !cumulative.is_zero() && sum.is_zero() {
                            prev.unwrap() + StoredU16::ONE
                        } else {
                            StoredU16::ZERO
                        };
                        prev.replace(days);
                        (i, days)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
