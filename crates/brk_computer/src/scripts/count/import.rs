use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::ComputedPerBlockCumulativeSum};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let p2a =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2a_count", version, indexes)?;
        let p2ms =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2ms_count", version, indexes)?;
        let p2pk33 =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2pk33_count", version, indexes)?;
        let p2pk65 =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2pk65_count", version, indexes)?;
        let p2pkh =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2pkh_count", version, indexes)?;
        let p2sh =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2sh_count", version, indexes)?;
        let p2tr =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2tr_count", version, indexes)?;
        let p2wpkh =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2wpkh_count", version, indexes)?;
        let p2wsh =
            ComputedPerBlockCumulativeSum::forced_import(db, "p2wsh_count", version, indexes)?;
        let segwit =
            ComputedPerBlockCumulativeSum::forced_import(db, "segwit_count", version, indexes)?;

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
            opreturn: ComputedPerBlockCumulativeSum::forced_import(
                db,
                "opreturn_count",
                version,
                indexes,
            )?,
            emptyoutput: ComputedPerBlockCumulativeSum::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
            )?,
            unknownoutput: ComputedPerBlockCumulativeSum::forced_import(
                db,
                "unknownoutput_count",
                version,
                indexes,
            )?,
            segwit,
        })
    }
}
