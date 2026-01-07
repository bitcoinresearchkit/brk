use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolSlug, Sats, StoredF32, StoredU16, StoredU32};
use vecdb::{
    Database, Exit, GenericStoredVec, IterableCloneableVec, IterableVec, LazyVecFrom2, VecIndex,
    Version,
};

use crate::{
    blocks,
    indexes::{self, ComputeIndexes},
    internal::{
        BinaryBlockSumCum, BinaryDateLast, ComputedBlockSumCum, ComputedDateLast,
        DerivedValueBlockSumCum, DollarsPlus, MaskSats, PercentageU32F32, SatsPlus,
        SatsPlusToBitcoin, ValueBinaryBlock,
    },
    price, transactions,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    slug: PoolSlug,

    pub indexes_to_blocks_mined: ComputedBlockSumCum<StoredU32>,
    pub indexes_to_1w_blocks_mined: ComputedDateLast<StoredU32>,
    pub indexes_to_1m_blocks_mined: ComputedDateLast<StoredU32>,
    pub indexes_to_1y_blocks_mined: ComputedDateLast<StoredU32>,
    pub height_to_subsidy: LazyVecFrom2<Height, Sats, Height, StoredU32, Height, Sats>,
    pub height_to_fee: LazyVecFrom2<Height, Sats, Height, StoredU32, Height, Sats>,
    pub indexes_to_subsidy: DerivedValueBlockSumCum,
    pub indexes_to_fee: DerivedValueBlockSumCum,
    pub indexes_to_coinbase: ValueBinaryBlock,
    pub indexes_to_dominance: BinaryBlockSumCum<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1d_dominance: BinaryBlockSumCum<StoredF32, StoredU32, StoredU32>,
    // KISS: both sources are ComputedVecsDateLast
    pub indexes_to_1w_dominance: BinaryDateLast<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1m_dominance: BinaryDateLast<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1y_dominance: BinaryDateLast<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_days_since_block: ComputedDateLast<StoredU16>,
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

        let indexes_to_blocks_mined =
            ComputedBlockSumCum::forced_import(db, &suffix("blocks_mined"), version, indexes)?;

        let indexes_to_1w_blocks_mined =
            ComputedDateLast::forced_import(db, &suffix("1w_blocks_mined"), version, indexes)?;
        let indexes_to_1m_blocks_mined =
            ComputedDateLast::forced_import(db, &suffix("1m_blocks_mined"), version, indexes)?;
        let indexes_to_1y_blocks_mined =
            ComputedDateLast::forced_import(db, &suffix("1y_blocks_mined"), version, indexes)?;

        // KISS: height is now a concrete field (no Option)
        let height_to_subsidy = LazyVecFrom2::transformed::<MaskSats>(
            &suffix("height_subsidy"),
            version,
            indexes_to_blocks_mined.height.boxed_clone(),
            blocks.rewards.indexes_to_subsidy.sats.height.boxed_clone(),
        );

        let indexes_to_subsidy = DerivedValueBlockSumCum::forced_import(
            db,
            &suffix("subsidy"),
            version,
            indexes,
            height_to_subsidy.boxed_clone(),
            price,
        )?;

        // KISS: height.sum_cum.sum.0 is now a concrete field
        let height_to_fee = LazyVecFrom2::transformed::<MaskSats>(
            &suffix("height_fee"),
            version,
            indexes_to_blocks_mined.height.boxed_clone(),
            transactions
                .fees
                .indexes_to_fee
                .sats
                .height
                .sum_cum
                .sum
                .0
                .boxed_clone(),
        );

        let indexes_to_fee = DerivedValueBlockSumCum::forced_import(
            db,
            &suffix("fee"),
            version,
            indexes,
            height_to_fee.boxed_clone(),
            price,
        )?;

        Ok(Self {
            indexes_to_dominance: BinaryBlockSumCum::from_computed::<PercentageU32F32>(
                &suffix("dominance"),
                version,
                indexes_to_blocks_mined.height.boxed_clone(),
                blocks.count.indexes_to_block_count.height.boxed_clone(),
                &indexes_to_blocks_mined,
                &blocks.count.indexes_to_block_count,
            ),
            indexes_to_1d_dominance: BinaryBlockSumCum::from_computed::<PercentageU32F32>(
                &suffix("1d_dominance"),
                version,
                indexes_to_blocks_mined.height.boxed_clone(),
                blocks.count.indexes_to_block_count.height.boxed_clone(),
                &indexes_to_blocks_mined,
                &blocks.count.indexes_to_block_count,
            ),
            indexes_to_1w_dominance: BinaryDateLast::from_computed_both_last::<PercentageU32F32>(
                &suffix("1w_dominance"),
                version,
                &indexes_to_1w_blocks_mined,
                &blocks.count.indexes_to_1w_block_count,
            ),
            indexes_to_1m_dominance: BinaryDateLast::from_computed_both_last::<PercentageU32F32>(
                &suffix("1m_dominance"),
                version,
                &indexes_to_1m_blocks_mined,
                &blocks.count.indexes_to_1m_block_count,
            ),
            indexes_to_1y_dominance: BinaryDateLast::from_computed_both_last::<PercentageU32F32>(
                &suffix("1y_dominance"),
                version,
                &indexes_to_1y_blocks_mined,
                &blocks.count.indexes_to_1y_block_count,
            ),
            slug,
            indexes_to_blocks_mined,
            indexes_to_1w_blocks_mined,
            indexes_to_1m_blocks_mined,
            indexes_to_1y_blocks_mined,
            indexes_to_coinbase: ValueBinaryBlock::from_derived::<
                SatsPlus,
                SatsPlusToBitcoin,
                DollarsPlus,
            >(
                &suffix("coinbase"),
                version,
                height_to_subsidy.boxed_clone(),
                height_to_fee.boxed_clone(),
                &indexes_to_subsidy,
                &indexes_to_fee,
            ),
            height_to_subsidy,
            height_to_fee,
            indexes_to_subsidy,
            indexes_to_fee,
            indexes_to_days_since_block: ComputedDateLast::forced_import(
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

        self.indexes_to_1w_blocks_mined
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.sum.inner(),
                    7,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1m_blocks_mined
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.sum.inner(),
                    30,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_1y_blocks_mined
            .compute_all(starting_indexes, exit, |v| {
                v.compute_sum(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.sum.inner(),
                    365,
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_subsidy.derive_from(
            indexes,
            starting_indexes,
            &self.height_to_subsidy,
            exit,
        )?;

        self.indexes_to_fee
            .derive_from(indexes, starting_indexes, &self.height_to_fee, exit)?;

        self.indexes_to_days_since_block
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform2(
                    starting_indexes.dateindex,
                    self.indexes_to_blocks_mined.dateindex.sum.inner(),
                    self.indexes_to_blocks_mined.dateindex.cumulative.inner(),
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
