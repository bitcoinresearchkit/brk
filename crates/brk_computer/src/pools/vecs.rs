use allocative::Allocative;
use brk_error::Result;
use brk_structs::{Height, PoolId, Pools, Sats, StoredF32, StoredU16, StoredU32};
use vecdb::{AnyCollectableVec, AnyIterableVec, Database, Exit, StoredIndex, VecIterator, Version};

use crate::{
    chain,
    grouped::{
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source,
        VecBuilderOptions,
    },
    indexes::{self, Indexes},
    price,
};

#[derive(Clone, Allocative)]
pub struct Vecs {
    id: PoolId,

    indexes_to_blocks_mined: ComputedVecsFromHeight<StoredU32>,
    indexes_to_1w_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    indexes_to_1m_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    indexes_to_1y_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    indexes_to_subsidy: ComputedValueVecsFromHeight,
    indexes_to_fee: ComputedValueVecsFromHeight,
    indexes_to_coinbase: ComputedValueVecsFromHeight,
    indexes_to_dominance: ComputedVecsFromDateIndex<StoredF32>,
    indexes_to_1d_dominance: ComputedVecsFromDateIndex<StoredF32>,
    indexes_to_1w_dominance: ComputedVecsFromDateIndex<StoredF32>,
    indexes_to_1m_dominance: ComputedVecsFromDateIndex<StoredF32>,
    indexes_to_1y_dominance: ComputedVecsFromDateIndex<StoredF32>,
    indexes_to_days_since_block: ComputedVecsFromDateIndex<StoredU16>,
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

        Ok(Self {
            id,
            indexes_to_blocks_mined: ComputedVecsFromHeight::forced_import(
                db,
                &suffix("blocks_mined"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_sum().add_cumulative(),
            )?,
            indexes_to_1w_blocks_mined: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1w_blocks_mined"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1m_blocks_mined: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1m_blocks_mined"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1y_blocks_mined: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1y_blocks_mined"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                db,
                &suffix("subsidy"),
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_fee: ComputedValueVecsFromHeight::forced_import(
                db,
                &suffix("fee"),
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                db,
                &suffix("coinbase"),
                Source::Compute,
                version + Version::ZERO,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_dominance: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("dominance"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1d_dominance: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1d_dominance"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1w_dominance: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1w_dominance"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1m_dominance: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1m_dominance"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_1y_dominance: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("1y_dominance"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
            indexes_to_days_since_block: ComputedVecsFromDateIndex::forced_import(
                db,
                &suffix("days_since_block"),
                Source::Compute,
                version + Version::ZERO,
                indexes,
                VecBuilderOptions::default().add_last(),
            )?,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        height_to_pool: &impl AnyIterableVec<Height, PoolId>,
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

        let height_to_blocks_mined = self.indexes_to_blocks_mined.height.as_ref().unwrap();

        self.indexes_to_subsidy
            .compute_all(indexes, price, starting_indexes, exit, |vec| {
                vec.compute_transform2(
                    starting_indexes.height,
                    height_to_blocks_mined,
                    chain.indexes_to_subsidy.sats.height.as_ref().unwrap(),
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
                    chain.indexes_to_coinbase.sats.height.as_ref().unwrap(),
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
                    self.indexes_to_1w_blocks_mined.dateindex.as_ref().unwrap(),
                    chain.indexes_to_1w_block_count.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_1m_blocks_mined.dateindex.as_ref().unwrap(),
                    chain.indexes_to_1m_block_count.dateindex.as_ref().unwrap(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_dominance
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    self.indexes_to_1y_blocks_mined.dateindex.as_ref().unwrap(),
                    chain.indexes_to_1y_block_count.dateindex.as_ref().unwrap(),
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
                            let i = i.unwrap_to_usize();
                            prev.replace(if i > 0 {
                                slf.into_iter().unwrap_get_inner_(i - 1)
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

    pub fn iter_any_collectable(&self) -> impl Iterator<Item = &dyn AnyCollectableVec> {
        let mut iter: Box<dyn Iterator<Item = &dyn AnyCollectableVec>> =
            Box::new(std::iter::empty());

        iter = Box::new(iter.chain(self.indexes_to_blocks_mined.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1w_blocks_mined.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1m_blocks_mined.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1y_blocks_mined.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_subsidy.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_fee.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_coinbase.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_dominance.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1d_dominance.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1w_dominance.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1m_dominance.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_1y_dominance.iter_any_collectable()));
        iter = Box::new(iter.chain(self.indexes_to_days_since_block.iter_any_collectable()));

        iter
    }
}
