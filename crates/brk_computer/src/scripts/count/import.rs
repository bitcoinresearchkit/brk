use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, ComputedPerBlockCumulativeWithSums},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let p2a =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2a_count", version, indexes, cached_starts)?;
        let p2ms =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2ms_count", version, indexes, cached_starts)?;
        let p2pk33 =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2pk33_count", version, indexes, cached_starts)?;
        let p2pk65 =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2pk65_count", version, indexes, cached_starts)?;
        let p2pkh =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2pkh_count", version, indexes, cached_starts)?;
        let p2sh =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2sh_count", version, indexes, cached_starts)?;
        let p2tr =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2tr_count", version, indexes, cached_starts)?;
        let p2wpkh =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2wpkh_count", version, indexes, cached_starts)?;
        let p2wsh =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "p2wsh_count", version, indexes, cached_starts)?;
        let segwit =
            ComputedPerBlockCumulativeWithSums::forced_import(db, "segwit_count", version, indexes, cached_starts)?;

        Ok(Self {
            p2a,
            p2ms,
            p2pk33,
            p2pk65,
            p2pkh,
            p2sh,
            p2tr,
            p2wpkh,
            p2wsh,
            opreturn: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "opreturn_count",
                version,
                indexes,
                cached_starts,
            )?,
            emptyoutput: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
                cached_starts,
            )?,
            unknownoutput: ComputedPerBlockCumulativeWithSums::forced_import(
                db,
                "unknownoutput_count",
                version,
                indexes,
                cached_starts,
            )?,
            segwit,
        })
    }
}
