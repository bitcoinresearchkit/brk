use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Indexes, PoolSlug, StoredU32, StoredU64};
use vecdb::{
    Database, Exit, ReadableVec, Rw, StorageMode, Version,
};

use crate::{
    blocks, indexes,
    internal::{
        CachedWindowStarts, PerBlockCumulativeWithSums, PercentPerBlock, RatioU64Bp16,
    },
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    slug: PoolSlug,

    pub blocks_mined: PerBlockCumulativeWithSums<StoredU32, StoredU64, M>,
    pub dominance: PercentPerBlock<BasisPoints16, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        slug: PoolSlug,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);

        let blocks_mined = PerBlockCumulativeWithSums::forced_import(
            db,
            &suffix("blocks_mined"),
            version + Version::ONE,
            indexes,
            cached_starts,
        )?;

        let dominance =
            PercentPerBlock::forced_import(db, &suffix("dominance"), version, indexes)?;

        Ok(Self {
            slug,
            blocks_mined,
            dominance,
        })
    }

    pub(crate) fn compute(
        &mut self,
        starting_indexes: &Indexes,
        pool: &impl ReadableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.blocks_mined
            .compute(starting_indexes.height, exit, |vec| {
                vec.compute_transform(
                    starting_indexes.height,
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
                starting_indexes.height,
                &self.blocks_mined.cumulative.height,
                &blocks.count.total.cumulative.height,
                exit,
            )?;

        Ok(())
    }
}
