use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Indexes, PoolSlug, StoredU32};
use vecdb::{
    BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, Version,
};

use crate::{
    blocks, indexes,
    internal::{
        ComputedFromHeightCumulativeSum, MaskSats, PercentFromHeight,
        PercentRollingWindows, RatioU32Bp16, ValueFromHeightCumulativeSum,
    },
    mining, prices,
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    slug: PoolSlug,

    pub blocks_mined: ComputedFromHeightCumulativeSum<StoredU32, M>,
    pub rewards: ValueFromHeightCumulativeSum<M>,
    pub dominance: PercentFromHeight<BasisPoints16, M>,
    pub dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
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

        let rewards =
            ValueFromHeightCumulativeSum::forced_import(db, &suffix("rewards"), version, indexes)?;

        let dominance =
            PercentFromHeight::forced_import(db, &suffix("dominance"), version, indexes)?;
        let dominance_rolling =
            PercentRollingWindows::forced_import(db, &suffix("dominance"), version, indexes)?;

        Ok(Self {
            dominance,
            dominance_rolling,
            slug,
            blocks_mined,
            rewards,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
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

        self.dominance
            .compute_binary::<StoredU32, StoredU32, RatioU32Bp16>(
                starting_indexes.height,
                &self.blocks_mined.cumulative.height,
                &blocks.count.block_count.cumulative.height,
                exit,
            )?;

        for ((dom, mined), total) in self
            .dominance_rolling
            .as_mut_array()
            .into_iter()
            .zip(self.blocks_mined.sum.as_array())
            .zip(blocks.count.block_count_sum.as_array())
        {
            dom.compute_binary::<StoredU32, StoredU32, RatioU32Bp16>(
                starting_indexes.height,
                &mined.height,
                &total.height,
                exit,
            )?;
        }

        self.rewards.compute(
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

        Ok(())
    }
}
