use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Height, Indexes, PoolSlug, StoredU32};
use vecdb::{
    Database, Exit, ReadableVec, Rw, StorageMode, Version,
};

use crate::{
    blocks, indexes,
    internal::{ComputedPerBlockCumulative, PercentPerBlock, RatioU32Bp16},
};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(skip)]
    slug: PoolSlug,

    pub blocks_mined: ComputedPerBlockCumulative<StoredU32, M>,
    pub dominance: PercentPerBlock<BasisPoints16, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        slug: PoolSlug,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let suffix = |s: &str| format!("{}_{s}", slug);

        let blocks_mined = ComputedPerBlockCumulative::forced_import(
            db,
            &suffix("blocks_mined"),
            version,
            indexes,
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
        height_to_pool: &impl ReadableVec<Height, PoolSlug>,
        blocks: &blocks::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.blocks_mined
            .compute(starting_indexes.height, exit, |vec| {
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
                &blocks.count.total.cumulative.height,
                exit,
            )?;

        Ok(())
    }
}
