use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolSlug, Sats, StoredF32, StoredU16, StoredU32};
use vecdb::{
    Database, Exit, GenericStoredVec, IterableCloneableVec, IterableVec, VecIndex, Version,
};

use crate::{
    blocks,
    indexes::{self, ComputeIndexes},
    internal::{
        ComputedFromHeightLast, ComputedFromHeightSumCum, ComputedFromDateLast, DollarsPlus, LazyBinaryFromHeightLast,
        LazyValueFromHeightSumCum, MaskSats, PercentageU32F32, SatsPlus, SatsPlusToBitcoin,
        ValueBinaryFromHeight,
    },
    price, transactions,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    slug: PoolSlug,

    pub blocks_mined: ComputedFromHeightSumCum<StoredU32>,
    pub _24h_blocks_mined: ComputedFromHeightLast<StoredU32>,
    pub _1w_blocks_mined: ComputedFromHeightLast<StoredU32>,
    pub _1m_blocks_mined: ComputedFromHeightLast<StoredU32>,
    pub _1y_blocks_mined: ComputedFromHeightLast<StoredU32>,
    pub subsidy: LazyValueFromHeightSumCum<StoredU32, Sats>,
    pub fee: LazyValueFromHeightSumCum<StoredU32, Sats>,
    pub coinbase: ValueBinaryFromHeight,
    pub dominance: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,

    pub _24h_dominance: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub _1w_dominance: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub _1m_dominance: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub _1y_dominance: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub days_since_block: ComputedFromDateLast<StoredU16>,
}

impl Vecs {
    pub fn forced_import(
        db: &Database,
        slug: PoolSlug,
        parent_version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        blocks: &blocks::Vecs,
        transactions: &transactions::Vecs,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);
        let version = parent_version;

        let blocks_mined =
            ComputedFromHeightSumCum::forced_import(db, &suffix("blocks_mined"), version, indexes)?;

        let _24h_blocks_mined =
            ComputedFromHeightLast::forced_import(db, &suffix("24h_blocks_mined"), version, indexes)?;
        let _1w_blocks_mined =
            ComputedFromHeightLast::forced_import(db, &suffix("1w_blocks_mined"), version, indexes)?;
        let _1m_blocks_mined =
            ComputedFromHeightLast::forced_import(db, &suffix("1m_blocks_mined"), version, indexes)?;
        let _1y_blocks_mined =
            ComputedFromHeightLast::forced_import(db, &suffix("1y_blocks_mined"), version, indexes)?;

        let subsidy = LazyValueFromHeightSumCum::forced_import::<MaskSats>(
            db,
            &suffix("subsidy"),
            version,
            indexes,
            blocks_mined.height.boxed_clone(),
            blocks.rewards.subsidy.sats.height.boxed_clone(),
            price,
        )?;

        let fee = LazyValueFromHeightSumCum::forced_import::<MaskSats>(
            db,
            &suffix("fee"),
            version,
            indexes,
            blocks_mined.height.boxed_clone(),
            transactions.fees.fee.sats.height.sum_cum.sum.0.boxed_clone(),
            price,
        )?;

        Ok(Self {
            dominance: LazyBinaryFromHeightLast::from_computed_sum_cum::<PercentageU32F32>(
                &suffix("dominance"),
                version,
                &blocks_mined,
                &blocks.count.block_count,
            ),
            _24h_dominance: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("24h_dominance"),
                version,
                &_24h_blocks_mined,
                &blocks.count._24h_block_count,
            ),
            _1w_dominance: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("1w_dominance"),
                version,
                &_1w_blocks_mined,
                &blocks.count._1w_block_count,
            ),
            _1m_dominance: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("1m_dominance"),
                version,
                &_1m_blocks_mined,
                &blocks.count._1m_block_count,
            ),
            _1y_dominance: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("1y_dominance"),
                version,
                &_1y_blocks_mined,
                &blocks.count._1y_block_count,
            ),
            slug,
            blocks_mined,
            _24h_blocks_mined,
            _1w_blocks_mined,
            _1m_blocks_mined,
            _1y_blocks_mined,
            coinbase: ValueBinaryFromHeight::from_lazy::<
                SatsPlus,
                SatsPlusToBitcoin,
                DollarsPlus,
                StoredU32,
                Sats,
            >(&suffix("coinbase"), version, &subsidy, &fee),
            subsidy,
            fee,
            days_since_block: ComputedFromDateLast::forced_import(
                db,
                &suffix("days_since_block"),
                version,
                indexes,
            )?,
        })
    }

    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_pool: &impl IterableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.blocks_mined
            .compute_all(indexes, starting_indexes, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
                    height_to_pool,
                    |(h, id, ..)| {
                        (
                            h,
                            if id == self.slug {
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

        // Compute rolling window blocks mined using the start heights from blocks.count
        let blocks_mined_height = &self.blocks_mined.height.clone();
        self._24h_blocks_mined
            .compute_all(indexes, starting_indexes, exit, |v| {
                Ok(v.compute_rolling_sum(
                    starting_indexes.height,
                    &blocks.count._24h_start,
                    blocks_mined_height,
                    exit,
                )?)
            })?;

        self._1w_blocks_mined
            .compute_all(indexes, starting_indexes, exit, |v| {
                Ok(v.compute_rolling_sum(
                    starting_indexes.height,
                    &blocks.count._1w_start,
                    blocks_mined_height,
                    exit,
                )?)
            })?;

        self._1m_blocks_mined
            .compute_all(indexes, starting_indexes, exit, |v| {
                Ok(v.compute_rolling_sum(
                    starting_indexes.height,
                    &blocks.count._1m_start,
                    blocks_mined_height,
                    exit,
                )?)
            })?;

        self._1y_blocks_mined
            .compute_all(indexes, starting_indexes, exit, |v| {
                Ok(v.compute_rolling_sum(
                    starting_indexes.height,
                    &blocks.count._1y_start,
                    blocks_mined_height,
                    exit,
                )?)
            })?;

        self.subsidy.derive_from(indexes, starting_indexes, exit)?;

        self.fee.derive_from(indexes, starting_indexes, exit)?;

        self.days_since_block
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.blocks_mined.dateindex.sum.inner(),
                    self.blocks_mined.dateindex.cumulative.inner(),
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
