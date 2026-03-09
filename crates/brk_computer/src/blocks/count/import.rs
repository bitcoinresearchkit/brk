use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{BlockCountTarget, ComputedPerBlockCumulativeSum, ConstantVecs},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            block_count_target: ConstantVecs::new::<BlockCountTarget>(
                "block_count_target",
                version,
                indexes,
            ),
            block_count: ComputedPerBlockCumulativeSum::forced_import(
                db,
                "block_count",
                version,
                indexes,
            )?,
        })
    }
}
