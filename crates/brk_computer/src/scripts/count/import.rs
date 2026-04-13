use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{PerBlockCumulativeRolling, WindowStartVec, Windows},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let p2a = PerBlockCumulativeRolling::forced_import(
            db,
            "p2a_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2ms = PerBlockCumulativeRolling::forced_import(
            db,
            "p2ms_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2pk33 = PerBlockCumulativeRolling::forced_import(
            db,
            "p2pk33_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2pk65 = PerBlockCumulativeRolling::forced_import(
            db,
            "p2pk65_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2pkh = PerBlockCumulativeRolling::forced_import(
            db,
            "p2pkh_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2sh = PerBlockCumulativeRolling::forced_import(
            db,
            "p2sh_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2tr = PerBlockCumulativeRolling::forced_import(
            db,
            "p2tr_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2wpkh = PerBlockCumulativeRolling::forced_import(
            db,
            "p2wpkh_count",
            version,
            indexes,
            cached_starts,
        )?;
        let p2wsh = PerBlockCumulativeRolling::forced_import(
            db,
            "p2wsh_count",
            version,
            indexes,
            cached_starts,
        )?;
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
            addr_output_count: PerBlockCumulativeRolling::forced_import(
                db,
                "addr_output_count",
                version,
                indexes,
                cached_starts,
            )?,
            op_return: PerBlockCumulativeRolling::forced_import(
                db,
                "op_return_count",
                version,
                indexes,
                cached_starts,
            )?,
            empty_output: PerBlockCumulativeRolling::forced_import(
                db,
                "empty_output_count",
                version,
                indexes,
                cached_starts,
            )?,
            unknown_output: PerBlockCumulativeRolling::forced_import(
                db,
                "unknown_output_count",
                version,
                indexes,
                cached_starts,
            )?,
        })
    }
}
