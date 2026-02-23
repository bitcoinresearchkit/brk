use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolSlug, Sats, StoredF32, StoredU16, StoredU32};
use vecdb::{
    Database, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, Version,
};

use crate::{
    blocks,
    indexes::{self, ComputeIndexes},
    internal::{
        ComputedFromHeightLast, ComputedFromHeightSumCum, DollarsPlus,
        LazyBinaryFromHeightLast, LazyValueFromHeightSumCum, MaskSats, PercentageU32F32, SatsPlus,
        SatsPlusToBitcoin, ValueBinaryFromHeight,
    },
    mining, prices, transactions,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    slug: PoolSlug,

    pub blocks_mined: ComputedFromHeightSumCum<StoredU32, M>,
    pub blocks_mined_24h_sum: ComputedFromHeightLast<StoredU32, M>,
    pub blocks_mined_1w_sum: ComputedFromHeightLast<StoredU32, M>,
    pub blocks_mined_1m_sum: ComputedFromHeightLast<StoredU32, M>,
    pub blocks_mined_1y_sum: ComputedFromHeightLast<StoredU32, M>,
    pub subsidy: LazyValueFromHeightSumCum<StoredU32, Sats, M>,
    pub fee: LazyValueFromHeightSumCum<StoredU32, Sats, M>,
    pub coinbase: ValueBinaryFromHeight,
    pub dominance: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,

    pub dominance_24h: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub dominance_1w: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub dominance_1m: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub dominance_1y: LazyBinaryFromHeightLast<StoredF32, StoredU32, StoredU32>,
    pub blocks_since_block: ComputedFromHeightLast<StoredU32, M>,
    pub days_since_block: ComputedFromHeightLast<StoredU16, M>,
}

impl Vecs {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn forced_import(
        db: &Database,
        slug: PoolSlug,
        parent_version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        mining: &mining::Vecs,
        transactions: &transactions::Vecs,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);
        let version = parent_version;

        let blocks_mined =
            ComputedFromHeightSumCum::forced_import(db, &suffix("blocks_mined"), version, indexes)?;

        let blocks_mined_24h_sum = ComputedFromHeightLast::forced_import(
            db,
            &suffix("blocks_mined_24h_sum"),
            version,
            indexes,
        )?;
        let blocks_mined_1w_sum = ComputedFromHeightLast::forced_import(
            db,
            &suffix("blocks_mined_1w_sum"),
            version,
            indexes,
        )?;
        let blocks_mined_1m_sum = ComputedFromHeightLast::forced_import(
            db,
            &suffix("blocks_mined_1m_sum"),
            version,
            indexes,
        )?;
        let blocks_mined_1y_sum = ComputedFromHeightLast::forced_import(
            db,
            &suffix("blocks_mined_1y_sum"),
            version,
            indexes,
        )?;

        let subsidy = LazyValueFromHeightSumCum::forced_import::<MaskSats>(
            db,
            &suffix("subsidy"),
            version,
            indexes,
            blocks_mined.height.read_only_boxed_clone(),
            mining.rewards.subsidy.sats.height.read_only_boxed_clone(),
            prices,
        )?;

        let fee = LazyValueFromHeightSumCum::forced_import::<MaskSats>(
            db,
            &suffix("fee"),
            version,
            indexes,
            blocks_mined.height.read_only_boxed_clone(),
            transactions.fees.fee.sats.height.boxed_sum(),
            prices,
        )?;

        Ok(Self {
            dominance: LazyBinaryFromHeightLast::from_computed_sum_cum::<PercentageU32F32>(
                &suffix("dominance"),
                version,
                &blocks_mined,
                &blocks.count.block_count,
            ),
            dominance_24h: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("dominance_24h"),
                version,
                &blocks_mined_24h_sum,
                &blocks.count.block_count_24h_sum,
            ),
            dominance_1w: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("dominance_1w"),
                version,
                &blocks_mined_1w_sum,
                &blocks.count.block_count_1w_sum,
            ),
            dominance_1m: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("dominance_1m"),
                version,
                &blocks_mined_1m_sum,
                &blocks.count.block_count_1m_sum,
            ),
            dominance_1y: LazyBinaryFromHeightLast::from_computed_last::<PercentageU32F32>(
                &suffix("dominance_1y"),
                version,
                &blocks_mined_1y_sum,
                &blocks.count.block_count_1y_sum,
            ),
            slug,
            blocks_mined,
            blocks_mined_24h_sum,
            blocks_mined_1w_sum,
            blocks_mined_1m_sum,
            blocks_mined_1y_sum,
            coinbase: ValueBinaryFromHeight::from_lazy::<
                SatsPlus,
                SatsPlusToBitcoin,
                DollarsPlus,
                StoredU32,
                Sats,
            >(&suffix("coinbase"), version, &subsidy, &fee),
            subsidy,
            fee,
            blocks_since_block: ComputedFromHeightLast::forced_import(
                db,
                &suffix("blocks_since_block"),
                version,
                indexes,
            )?,
            days_since_block: ComputedFromHeightLast::forced_import(
                db,
                &suffix("days_since_block"),
                version,
                indexes,
            )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        height_to_pool: &impl ReadableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.blocks_mined
            .compute(starting_indexes, exit, |vec| {
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
        self.blocks_mined_24h_sum.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &self.blocks_mined.height,
            exit,
        )?;

        self.blocks_mined_1w_sum.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            &self.blocks_mined.height,
            exit,
        )?;

        self.blocks_mined_1m_sum.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.blocks_mined.height,
            exit,
        )?;

        self.blocks_mined_1y_sum.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            &self.blocks_mined.height,
            exit,
        )?;

        self.subsidy.compute_cumulative(starting_indexes, exit)?;

        self.fee.compute_cumulative(starting_indexes, exit)?;

        {
            let mut prev = StoredU32::ZERO;
            self.blocks_since_block.height.compute_transform(
                starting_indexes.height,
                &self.blocks_mined.height,
                |(h, mined, ..)| {
                    let blocks = if mined.is_zero() {
                        prev + StoredU32::ONE
                    } else {
                        StoredU32::ZERO
                    };
                    prev = blocks;
                    (h, blocks)
                },
                exit,
            )?;
        }

        self.days_since_block.height.compute_transform(
            starting_indexes.height,
            &self.blocks_since_block.height,
            |(h, blocks, ..)| (h, StoredU16::from(u16::try_from(*blocks).unwrap_or(u16::MAX))),
            exit,
        )?;

        Ok(())
    }
}
