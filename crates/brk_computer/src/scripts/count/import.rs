use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedFromHeightCumulativeSum};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let p2a = ComputedFromHeightCumulativeSum::forced_import(db, "p2a_count", version, indexes)?;
        let p2ms = ComputedFromHeightCumulativeSum::forced_import(db, "p2ms_count", version, indexes)?;
        let p2pk33 =
            ComputedFromHeightCumulativeSum::forced_import(db, "p2pk33_count", version, indexes)?;
        let p2pk65 =
            ComputedFromHeightCumulativeSum::forced_import(db, "p2pk65_count", version, indexes)?;
        let p2pkh = ComputedFromHeightCumulativeSum::forced_import(db, "p2pkh_count", version, indexes)?;
        let p2sh = ComputedFromHeightCumulativeSum::forced_import(db, "p2sh_count", version, indexes)?;
        let p2tr = ComputedFromHeightCumulativeSum::forced_import(db, "p2tr_count", version, indexes)?;
        let p2wpkh =
            ComputedFromHeightCumulativeSum::forced_import(db, "p2wpkh_count", version, indexes)?;
        let p2wsh = ComputedFromHeightCumulativeSum::forced_import(db, "p2wsh_count", version, indexes)?;
        let segwit =
            ComputedFromHeightCumulativeSum::forced_import(db, "segwit_count", version, indexes)?;

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
            opreturn: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "opreturn_count",
                version,
                indexes,
            )?,
            emptyoutput: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
            )?,
            unknownoutput: ComputedFromHeightCumulativeSum::forced_import(
                db,
                "unknownoutput_count",
                version,
                indexes,
            )?,
            segwit,
        })
    }
}
