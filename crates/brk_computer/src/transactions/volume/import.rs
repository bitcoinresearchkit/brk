use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{AmountPerBlockCumulativeWithSums, CachedWindowStarts, PerBlock},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let v2 = Version::TWO;
        Ok(Self {
            transfer_volume: AmountPerBlockCumulativeWithSums::forced_import(
                db,
                "transfer_volume_bis",
                version,
                indexes,
                cached_starts,
            )?,
            tx_per_sec: PerBlock::forced_import(db, "tx_per_sec", version + v2, indexes)?,
            outputs_per_sec: PerBlock::forced_import(
                db,
                "outputs_per_sec",
                version + v2,
                indexes,
            )?,
            inputs_per_sec: PerBlock::forced_import(
                db,
                "inputs_per_sec",
                version + v2,
                indexes,
            )?,
        })
    }
}
