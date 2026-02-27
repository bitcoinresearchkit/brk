use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolSlug, StoredF32, StoredU16, StoredU32};
use vecdb::{BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, Version};

use crate::{
    blocks,
    indexes::{self, ComputeIndexes},
    internal::{
        ComputedFromHeightCumulativeSum, ComputedFromHeightLast, MaskSats, PercentageU32F32,
        ValueFromHeightSumCumulative,
    },
    mining, prices,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    slug: PoolSlug,

    pub blocks_mined: ComputedFromHeightCumulativeSum<StoredU32, M>,
    pub blocks_mined_24h_sum: ComputedFromHeightLast<StoredU32, M>,
    pub blocks_mined_1w_sum: ComputedFromHeightLast<StoredU32, M>,
    pub blocks_mined_1m_sum: ComputedFromHeightLast<StoredU32, M>,
    pub blocks_mined_1y_sum: ComputedFromHeightLast<StoredU32, M>,
    pub subsidy: ValueFromHeightSumCumulative<M>,
    pub fee: ValueFromHeightSumCumulative<M>,
    pub coinbase: ValueFromHeightSumCumulative<M>,
    pub dominance: ComputedFromHeightLast<StoredF32, M>,

    pub dominance_24h: ComputedFromHeightLast<StoredF32, M>,
    pub dominance_1w: ComputedFromHeightLast<StoredF32, M>,
    pub dominance_1m: ComputedFromHeightLast<StoredF32, M>,
    pub dominance_1y: ComputedFromHeightLast<StoredF32, M>,
    pub blocks_since_block: ComputedFromHeightLast<StoredU32, M>,
    pub days_since_block: ComputedFromHeightLast<StoredU16, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        slug: PoolSlug,
        parent_version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);
        let version = parent_version;

        let blocks_mined = ComputedFromHeightCumulativeSum::forced_import(
            db,
            &suffix("blocks_mined"),
            version,
            indexes,
        )?;

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

        let subsidy =
            ValueFromHeightSumCumulative::forced_import(db, &suffix("subsidy"), version, indexes)?;

        let fee =
            ValueFromHeightSumCumulative::forced_import(db, &suffix("fee"), version, indexes)?;

        let coinbase =
            ValueFromHeightSumCumulative::forced_import(db, &suffix("coinbase"), version, indexes)?;

        let dominance =
            ComputedFromHeightLast::forced_import(db, &suffix("dominance"), version, indexes)?;
        let dominance_24h =
            ComputedFromHeightLast::forced_import(db, &suffix("dominance_24h"), version, indexes)?;
        let dominance_1w =
            ComputedFromHeightLast::forced_import(db, &suffix("dominance_1w"), version, indexes)?;
        let dominance_1m =
            ComputedFromHeightLast::forced_import(db, &suffix("dominance_1m"), version, indexes)?;
        let dominance_1y =
            ComputedFromHeightLast::forced_import(db, &suffix("dominance_1y"), version, indexes)?;

        Ok(Self {
            dominance,
            dominance_24h,
            dominance_1w,
            dominance_1m,
            dominance_1y,
            slug,
            blocks_mined,
            blocks_mined_24h_sum,
            blocks_mined_1w_sum,
            blocks_mined_1m_sum,
            blocks_mined_1y_sum,
            coinbase,
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

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        height_to_pool: &impl ReadableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        mining: &mining::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let window_starts = blocks.count.window_starts();

        self.blocks_mined
            .compute(starting_indexes.height, &window_starts, exit, |vec| {
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

        self.dominance
            .compute_binary::<StoredU32, StoredU32, PercentageU32F32>(
                starting_indexes.height,
                &self.blocks_mined.cumulative.height,
                &blocks.count.block_count.cumulative.height,
                exit,
            )?;

        self.dominance_24h
            .compute_binary::<StoredU32, StoredU32, PercentageU32F32>(
                starting_indexes.height,
                &self.blocks_mined_24h_sum.height,
                &blocks.count.block_count_sum._24h.height,
                exit,
            )?;

        self.dominance_1w
            .compute_binary::<StoredU32, StoredU32, PercentageU32F32>(
                starting_indexes.height,
                &self.blocks_mined_1w_sum.height,
                &blocks.count.block_count_sum._7d.height,
                exit,
            )?;

        self.dominance_1m
            .compute_binary::<StoredU32, StoredU32, PercentageU32F32>(
                starting_indexes.height,
                &self.blocks_mined_1m_sum.height,
                &blocks.count.block_count_sum._30d.height,
                exit,
            )?;

        self.dominance_1y
            .compute_binary::<StoredU32, StoredU32, PercentageU32F32>(
                starting_indexes.height,
                &self.blocks_mined_1y_sum.height,
                &blocks.count.block_count_sum._1y.height,
                exit,
            )?;

        self.subsidy.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |vec| {
                Ok(vec.compute_transform2(
                    starting_indexes.height,
                    &self.blocks_mined.height,
                    &mining.rewards.subsidy.base.sats.height,
                    |(h, mask, val, ..)| (h, MaskSats::apply(mask, val)),
                    exit,
                )?)
            },
        )?;

        self.fee.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |vec| {
                Ok(vec.compute_transform2(
                    starting_indexes.height,
                    &self.blocks_mined.height,
                    &mining.rewards.fees.base.sats.height,
                    |(h, mask, val, ..)| (h, MaskSats::apply(mask, val)),
                    exit,
                )?)
            },
        )?;

        self.coinbase.compute(
            starting_indexes.height,
            &window_starts,
            prices,
            exit,
            |vec| {
                Ok(vec.compute_transform2(
                    starting_indexes.height,
                    &self.blocks_mined.height,
                    &mining.rewards.coinbase.base.sats.height,
                    |(h, mask, val, ..)| (h, MaskSats::apply(mask, val)),
                    exit,
                )?)
            },
        )?;

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
            |(h, blocks, ..)| {
                (
                    h,
                    StoredU16::from(u16::try_from(*blocks).unwrap_or(u16::MAX)),
                )
            },
            exit,
        )?;

        Ok(())
    }
}
