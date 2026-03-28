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

        let blocks_to_halving =
            PerBlock::forced_import(db, "blocks_to_halving", version + v2, indexes)?;

        let days_to_halving = LazyPerBlock::from_computed::<BlocksToDaysF32>(
            "days_to_halving",
            version + v2,
            blocks_to_halving.height.read_only_boxed_clone(),
            &blocks_to_halving,
        );

        Ok(Self {
            epoch: PerBlock::forced_import(db, "halving_epoch", version, indexes)?,
            blocks_to_halving,
            days_to_halving,
        })
    }
}
