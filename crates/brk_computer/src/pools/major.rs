use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Indexes, PoolSlug, StoredU32};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, Version};

use crate::{
    blocks, indexes,
    internal::{
        AmountPerBlockCumulativeSum, MaskSats, PercentRollingWindows, RatioU32Bp16,
        RollingWindows,
    },
    mining, prices,
};

use super::minor;

#[derive(Deref, DerefMut, Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: minor::Vecs<M>,

    #[traversable(wrap = "blocks_mined", rename = "sum")]
    pub blocks_mined_sum: RollingWindows<StoredU32, M>,
    pub rewards: AmountPerBlockCumulativeSum<M>,
    #[traversable(rename = "dominance")]
    pub dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        slug: PoolSlug,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);

        let base = minor::Vecs::forced_import(db, slug, version, indexes)?;

        let blocks_mined_sum =
            RollingWindows::forced_import(db, &suffix("blocks_mined"), version, indexes)?;

        let rewards =
            AmountPerBlockCumulativeSum::forced_import(db, &suffix("rewards"), version, indexes)?;

        let dominance_rolling =
            PercentRollingWindows::forced_import(db, &suffix("dominance"), version, indexes)?;

        Ok(Self {
            base,
            blocks_mined_sum,
            rewards,
            dominance_rolling,
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
        self.base
            .compute(starting_indexes, height_to_pool, blocks, exit)?;

        let window_starts = blocks.lookback.window_starts();

        self.blocks_mined_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.base.blocks_mined.raw.height,
            exit,
        )?;

        for (dom, (mined, total)) in self.dominance_rolling.as_mut_array().into_iter().zip(
            self.blocks_mined_sum
                .as_array()
                .into_iter()
                .zip(blocks.count.total.sum.as_array()),
        ) {
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
                    &self.base.blocks_mined.raw.height,
                    &mining.rewards.coinbase.base.sats.height,
                    |(h, mask, val, ..)| (h, MaskSats::apply(mask, val)),
                    exit,
                )?)
            },
        )?;

        Ok(())
    }
}
