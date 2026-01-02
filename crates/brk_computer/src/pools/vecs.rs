use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolSlug, Sats, StoredF32, StoredU16, StoredU32};
use vecdb::{
    Database, Exit, GenericStoredVec, IterableCloneableVec, IterableVec, LazyVecFrom2, VecIndex,
    Version,
};

use crate::{
    blocks, transactions,
    internal::{
        ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, ComputedVecsFromHeight,
        DollarsPlus, LazyValueVecsFrom2FromHeight, LazyVecsFrom2FromDateIndex,
        LazyVecsFrom2FromHeight, MaskSats, PercentageU32F32, SatsPlus, SatsPlusToBitcoin, Source,
        VecBuilderOptions,
    },
    indexes::{self, ComputeIndexes},
    price,
};

#[derive(Clone, Traversable)]
pub struct Vecs {
    slug: PoolSlug,

    pub indexes_to_blocks_mined: ComputedVecsFromHeight<StoredU32>,
    pub indexes_to_1w_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1m_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    pub indexes_to_1y_blocks_mined: ComputedVecsFromDateIndex<StoredU32>,
    pub height_to_subsidy: LazyVecFrom2<Height, Sats, Height, StoredU32, Height, Sats>,
    pub height_to_fee: LazyVecFrom2<Height, Sats, Height, StoredU32, Height, Sats>,
    pub indexes_to_subsidy: ComputedValueVecsFromHeight,
    pub indexes_to_fee: ComputedValueVecsFromHeight,
    pub indexes_to_coinbase: LazyValueVecsFrom2FromHeight,
    pub indexes_to_dominance: LazyVecsFrom2FromHeight<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1d_dominance: LazyVecsFrom2FromHeight<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1w_dominance: LazyVecsFrom2FromDateIndex<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1m_dominance: LazyVecsFrom2FromDateIndex<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_1y_dominance: LazyVecsFrom2FromDateIndex<StoredF32, StoredU32, StoredU32>,
    pub indexes_to_days_since_block: ComputedVecsFromDateIndex<StoredU16>,
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
        let compute_dollars = price.is_some();
        let version = parent_version + Version::ZERO;

        let last = VecBuilderOptions::default().add_last();
        let sum_cum = VecBuilderOptions::default().add_sum().add_cumulative();

        macro_rules! import_di {
            ($name:expr) => {
                ComputedVecsFromDateIndex::forced_import(
                    db,
                    &suffix($name),
                    Source::Compute,
                    version,
                    indexes,
                    last.clone(),
                )?
            };
        }

        let indexes_to_blocks_mined = ComputedVecsFromHeight::forced_import(
            db,
            &suffix("blocks_mined"),
            Source::Compute,
            version,
            indexes,
            sum_cum,
        )?;

        let indexes_to_1w_blocks_mined = import_di!("1w_blocks_mined");
        let indexes_to_1m_blocks_mined = import_di!("1m_blocks_mined");
        let indexes_to_1y_blocks_mined = import_di!("1y_blocks_mined");

        let height_to_subsidy = LazyVecFrom2::transformed::<MaskSats>(
            &suffix("subsidy"),
            version,
            indexes_to_blocks_mined
                .height
                .as_ref()
                .unwrap()
                .boxed_clone(),
            blocks
                .rewards.indexes_to_subsidy
                .sats
                .height
                .as_ref()
                .unwrap()
                .boxed_clone(),
        );

        let indexes_to_subsidy = ComputedValueVecsFromHeight::forced_import(
            db,
            &suffix("subsidy"),
            Source::Vec(height_to_subsidy.boxed_clone()),
            version,
            sum_cum,
            compute_dollars,
            indexes,
        )?;

        let height_to_fee = LazyVecFrom2::transformed::<MaskSats>(
            &suffix("fee"),
            version,
            indexes_to_blocks_mined
                .height
                .as_ref()
                .unwrap()
                .boxed_clone(),
            transactions.fees.indexes_to_fee.sats.height.unwrap_sum().boxed_clone(),
        );

        let indexes_to_fee = ComputedValueVecsFromHeight::forced_import(
            db,
            &suffix("fee"),
            Source::Vec(height_to_fee.boxed_clone()),
            version,
            sum_cum,
            compute_dollars,
            indexes,
        )?;

        Ok(Self {
            indexes_to_dominance: LazyVecsFrom2FromHeight::from_computed::<PercentageU32F32>(
                &suffix("dominance"),
                version,
                indexes_to_blocks_mined
                    .height
                    .as_ref()
                    .unwrap()
                    .boxed_clone(),
                blocks
                    .count.indexes_to_block_count
                    .height
                    .as_ref()
                    .unwrap()
                    .boxed_clone(),
                &indexes_to_blocks_mined,
                &blocks.count.indexes_to_block_count,
            ),
            indexes_to_1d_dominance: LazyVecsFrom2FromHeight::from_computed::<PercentageU32F32>(
                &suffix("1d_dominance"),
                version,
                indexes_to_blocks_mined
                    .height
                    .as_ref()
                    .unwrap()
                    .boxed_clone(),
                blocks
                    .count.indexes_to_block_count
                    .height
                    .as_ref()
                    .unwrap()
                    .boxed_clone(),
                &indexes_to_blocks_mined,
                &blocks.count.indexes_to_block_count,
            ),
            indexes_to_1w_dominance: LazyVecsFrom2FromDateIndex::from_computed::<PercentageU32F32>(
                &suffix("1w_dominance"),
                version,
                &indexes_to_1w_blocks_mined,
                &blocks.count.indexes_to_1w_block_count,
            ),
            indexes_to_1m_dominance: LazyVecsFrom2FromDateIndex::from_computed::<PercentageU32F32>(
                &suffix("1m_dominance"),
                version,
                &indexes_to_1m_blocks_mined,
                &blocks.count.indexes_to_1m_block_count,
            ),
            indexes_to_1y_dominance: LazyVecsFrom2FromDateIndex::from_computed::<PercentageU32F32>(
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
            indexes_to_coinbase: LazyValueVecsFrom2FromHeight::from_computed::<
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
            indexes_to_days_since_block: import_di!("days_since_block"),
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_pool: &impl IterableVec<Height, PoolSlug>,
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

        self.indexes_to_subsidy.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_subsidy),
        )?;

        self.indexes_to_fee.compute_rest(
            indexes,
            price,
            starting_indexes,
            exit,
            Some(&self.height_to_fee),
        )?;

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
