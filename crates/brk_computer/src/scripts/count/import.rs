use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlockCumulativeWithSums},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let p2a =
            PerBlockCumulativeWithSums::forced_import(db, "p2a_count", version, indexes, cached_starts)?;
        let p2ms =
            PerBlockCumulativeWithSums::forced_import(db, "p2ms_count", version, indexes, cached_starts)?;
        let p2pk33 =
            PerBlockCumulativeWithSums::forced_import(db, "p2pk33_count", version, indexes, cached_starts)?;
        let p2pk65 =
            PerBlockCumulativeWithSums::forced_import(db, "p2pk65_count", version, indexes, cached_starts)?;
        let p2pkh =
            PerBlockCumulativeWithSums::forced_import(db, "p2pkh_count", version, indexes, cached_starts)?;
        let p2sh =
            PerBlockCumulativeWithSums::forced_import(db, "p2sh_count", version, indexes, cached_starts)?;
        let p2tr =
            PerBlockCumulativeWithSums::forced_import(db, "p2tr_count", version, indexes, cached_starts)?;
        let p2wpkh =
            PerBlockCumulativeWithSums::forced_import(db, "p2wpkh_count", version, indexes, cached_starts)?;
        let p2wsh =
            PerBlockCumulativeWithSums::forced_import(db, "p2wsh_count", version, indexes, cached_starts)?;
        let segwit =
            PerBlockCumulativeWithSums::forced_import(db, "segwit_count", version, indexes, cached_starts)?;

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
            opreturn: PerBlockCumulativeWithSums::forced_import(
                db,
                "opreturn_count",
                version,
                indexes,
                cached_starts,
            )?,
            emptyoutput: PerBlockCumulativeWithSums::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
                cached_starts,
            )?,
            unknownoutput: PerBlockCumulativeWithSums::forced_import(
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
