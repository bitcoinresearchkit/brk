use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightCumSum, ComputedFromHeightLast},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let p2a = ComputedFromHeightCumSum::forced_import(db, "p2a_count", version, indexes)?;
        let p2ms = ComputedFromHeightCumSum::forced_import(db, "p2ms_count", version, indexes)?;
        let p2pk33 =
            ComputedFromHeightCumSum::forced_import(db, "p2pk33_count", version, indexes)?;
        let p2pk65 =
            ComputedFromHeightCumSum::forced_import(db, "p2pk65_count", version, indexes)?;
        let p2pkh = ComputedFromHeightCumSum::forced_import(db, "p2pkh_count", version, indexes)?;
        let p2sh = ComputedFromHeightCumSum::forced_import(db, "p2sh_count", version, indexes)?;
        let p2tr = ComputedFromHeightCumSum::forced_import(db, "p2tr_count", version, indexes)?;
        let p2wpkh =
            ComputedFromHeightCumSum::forced_import(db, "p2wpkh_count", version, indexes)?;
        let p2wsh = ComputedFromHeightCumSum::forced_import(db, "p2wsh_count", version, indexes)?;
        let segwit =
            ComputedFromHeightCumSum::forced_import(db, "segwit_count", version, indexes)?;

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
            opreturn: ComputedFromHeightCumSum::forced_import(
                db,
                "opreturn_count",
                version,
                indexes,
            )?,
            emptyoutput: ComputedFromHeightCumSum::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
            )?,
            unknownoutput: ComputedFromHeightCumSum::forced_import(
                db,
                "unknownoutput_count",
                version,
                indexes,
            )?,
            segwit,
            taproot_adoption: ComputedFromHeightLast::forced_import(
                db,
                "taproot_adoption",
                version,
                indexes,
            )?,
            segwit_adoption: ComputedFromHeightLast::forced_import(
                db,
                "segwit_adoption",
                version,
                indexes,
            )?,
        })
    }
}
