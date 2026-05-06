use brk_error::Result;
use brk_indexer::Indexer;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, PoolSlug, StoredU32, StoredU64};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode, Version};

use crate::{
    blocks, indexes,
    internal::{PerBlockCumulativeRolling, PercentPerBlock, RatioU64Bp16, WindowStartVec, Windows},
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    slug: PoolSlug,

    pub blocks_mined: PerBlockCumulativeRolling<StoredU32, StoredU64, M>,
    pub dominance: PercentPerBlock<BasisPoints16, M>,
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

        let blocks_mined = PerBlockCumulativeRolling::forced_import(
            db,
            &suffix("blocks_mined"),
            version + Version::ONE,
            indexes,
            cached_starts,
        )?;

        let dominance = PercentPerBlock::forced_import(db, &suffix("dominance"), version, indexes)?;

        Ok(Self {
            slug,
            blocks_mined,
            dominance,
        })
    }

    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        pool: &impl ReadableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;

        self.blocks_mined.compute(starting_height, exit, |vec| {
            vec.compute_transform(
                starting_height,
                pool,
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
            .compute_binary::<StoredU64, StoredU64, RatioU64Bp16>(
                starting_height,
                &self.blocks_mined.cumulative.height,
                &blocks.count.total.cumulative.height,
                exit,
            )?;

        Ok(())
    }
}
