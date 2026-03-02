use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, PoolSlug, StoredF32, StoredU16, StoredU32};
use vecdb::{AnyVec, BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, VecIndex, Version};

use crate::{
    blocks,
    indexes::{self, ComputeIndexes},
    internal::{
        ComputedFromHeightCumulativeSum, ComputedFromHeight, MaskSats, PercentageU32F32,
        RollingWindows, ValueFromHeightCumulativeSum,
    },
    mining, prices,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    slug: PoolSlug,

    pub blocks_mined: ComputedFromHeightCumulativeSum<StoredU32, M>,
    pub blocks_mined_sum: RollingWindows<StoredU32, M>,
    pub subsidy: ValueFromHeightCumulativeSum<M>,
    pub fee: ValueFromHeightCumulativeSum<M>,
    pub coinbase: ValueFromHeightCumulativeSum<M>,
    pub dominance: ComputedFromHeight<StoredF32, M>,
    pub dominance_rolling: RollingWindows<StoredF32, M>,
    pub blocks_since_block: ComputedFromHeight<StoredU32, M>,
    pub days_since_block: ComputedFromHeight<StoredU16, M>,
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

        let blocks_mined_sum =
            RollingWindows::forced_import(db, &suffix("blocks_mined_sum"), version, indexes)?;

        let subsidy =
            ValueFromHeightCumulativeSum::forced_import(db, &suffix("subsidy"), version, indexes)?;

        let fee =
            ValueFromHeightCumulativeSum::forced_import(db, &suffix("fee"), version, indexes)?;

        let coinbase =
            ValueFromHeightCumulativeSum::forced_import(db, &suffix("coinbase"), version, indexes)?;

        let dominance =
            ComputedFromHeight::forced_import(db, &suffix("dominance"), version, indexes)?;
        let dominance_rolling =
            RollingWindows::forced_import(db, &suffix("dominance"), version, indexes)?;

        Ok(Self {
            dominance,
            dominance_rolling,
            slug,
            blocks_mined,
            blocks_mined_sum,
            coinbase,
            subsidy,
            fee,
            blocks_since_block: ComputedFromHeight::forced_import(
                db,
                &suffix("blocks_since_block"),
                version,
                indexes,
            )?,
            days_since_block: ComputedFromHeight::forced_import(
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

        self.blocks_mined_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
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

        for ((dom, mined), total) in self
            .dominance_rolling
            .as_mut_array()
            .into_iter()
            .zip(self.blocks_mined_sum.as_array())
            .zip(blocks.count.block_count_sum.as_array())
        {
            dom.compute_binary::<StoredU32, StoredU32, PercentageU32F32>(
                starting_indexes.height,
                &mined.height,
                &total.height,
                exit,
            )?;
        }

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
            let resume_from = self
                .blocks_since_block
                .height
                .len()
                .min(starting_indexes.height.to_usize());
            let mut prev = if resume_from > 0 {
                self.blocks_since_block
                    .height
                    .collect_one_at(resume_from - 1)
                    .unwrap()
            } else {
                StoredU32::ZERO
            };
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
