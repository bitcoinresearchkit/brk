use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{BlocksToDaysF32, LazyPerBlock, PerBlock},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        let blocks_before_next = PerBlock::forced_import(
            db, "blocks_before_next_halving", version + v2, indexes,
        )?;

        let days_before_next = LazyPerBlock::from_computed::<BlocksToDaysF32>(
            "days_before_next_halving",
            version + v2,
            blocks_before_next.height.read_only_boxed_clone(),
            &blocks_before_next,
        );

        Ok(Self {
            epoch: PerBlock::forced_import(db, "halving_epoch", version, indexes)?,
            blocks_before_next,
            days_before_next,
        })
    }
}
