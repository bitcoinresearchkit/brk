use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, ReadableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, ComputedHeightDerivedCumFull, RollingDistribution},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let weight = ComputedHeightDerivedCumFull::forced_import(
            db,
            "block_weight",
            indexer.vecs.blocks.weight.read_only_boxed_clone(),
            version,
            indexes,
        )?;

        let fullness =
            ComputedFromHeightLast::forced_import(db, "block_fullness", version, indexes)?;

        let fullness_rolling =
            RollingDistribution::forced_import(db, "block_fullness", version, indexes)?;

        Ok(Self {
            weight,
            fullness,
            fullness_rolling,
        })
    }
}
