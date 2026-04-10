use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Indexes, PoolSlug, StoredU64};
use derive_more::{Deref, DerefMut};
use vecdb::{BinaryTransform, Database, Exit, ReadableVec, Rw, StorageMode, Version};

use crate::{
    blocks, indexes,
    internal::{
        AmountPerBlockCumulativeRolling, MaskSats, PercentRollingWindows, RatioU64Bp16,
        WindowStartVec, Windows,
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

    pub rewards: AmountPerBlockCumulativeRolling<M>,
    #[traversable(rename = "dominance")]
    pub dominance_rolling: PercentRollingWindows<BasisPoints16, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        slug: PoolSlug,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);

        let base = minor::Vecs::forced_import(db, slug, version, indexes, cached_starts)?;

        let rewards = AmountPerBlockCumulativeRolling::forced_import(
            db,
            &suffix("rewards"),
            version,
            indexes,
            cached_starts,
        )?;

        let dominance_rolling =
            PercentRollingWindows::forced_import(db, &suffix("dominance"), version, indexes)?;

        Ok(Self {
            base,
            rewards,
            dominance_rolling,
        })
    }

    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        pool: &impl ReadableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        mining: &mining::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.base.compute(starting_indexes, pool, blocks, exit)?;

        for (dom, (mined, total)) in self.dominance_rolling.as_mut_array().into_iter().zip(
            self.base
                .blocks_mined
                .sum
                .as_array()
                .into_iter()
                .zip(blocks.count.total.sum.as_array()),
        ) {
            dom.compute_binary::<StoredU64, StoredU64, RatioU64Bp16>(
                starting_indexes.height,
                &mined.height,
                &total.height,
                exit,
            )?;
        }

        self.rewards
            .compute(starting_indexes.height, prices, exit, |vec| {
                Ok(vec.compute_transform2(
                    starting_indexes.height,
                    &self.base.blocks_mined.block,
                    &mining.rewards.coinbase.block.sats,
                    |(h, mask, val, ..)| (h, MaskSats::apply(mask, val)),
                    exit,
                )?)
            })?;

        Ok(())
    }
}
