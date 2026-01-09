use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{BinaryBlockFull, ComputedBlockFull, PercentageU64F32},
    outputs,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        outputs: &outputs::Vecs,
    ) -> Result<Self> {
        let p2a = ComputedBlockFull::forced_import(db, "p2a_count", version, indexes)?;
        let p2ms = ComputedBlockFull::forced_import(db, "p2ms_count", version, indexes)?;
        let p2pk33 = ComputedBlockFull::forced_import(db, "p2pk33_count", version, indexes)?;
        let p2pk65 = ComputedBlockFull::forced_import(db, "p2pk65_count", version, indexes)?;
        let p2pkh = ComputedBlockFull::forced_import(db, "p2pkh_count", version, indexes)?;
        let p2sh = ComputedBlockFull::forced_import(db, "p2sh_count", version, indexes)?;
        let p2tr = ComputedBlockFull::forced_import(db, "p2tr_count", version, indexes)?;
        let p2wpkh = ComputedBlockFull::forced_import(db, "p2wpkh_count", version, indexes)?;
        let p2wsh = ComputedBlockFull::forced_import(db, "p2wsh_count", version, indexes)?;

        // Aggregate counts (computed from per-type counts)
        let segwit = ComputedBlockFull::forced_import(db, "segwit_count", version, indexes)?;

        // Adoption ratios (lazy)
        // Uses outputs.count.count as denominator (total output count)
        // At height level: per-block ratio; at dateindex level: sum-based ratio (% of new outputs)
        let taproot_adoption = BinaryBlockFull::from_height_and_txindex::<PercentageU64F32>(
            "taproot_adoption",
            version,
            p2tr.height.boxed_clone(),
            outputs.count.total_count.height.sum_cum.sum.0.boxed_clone(),
            &p2tr,
            &outputs.count.total_count,
        );
        let segwit_adoption = BinaryBlockFull::from_height_and_txindex::<PercentageU64F32>(
            "segwit_adoption",
            version,
            segwit.height.boxed_clone(),
            outputs.count.total_count.height.sum_cum.sum.0.boxed_clone(),
            &segwit,
            &outputs.count.total_count,
        );

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
            opreturn: ComputedBlockFull::forced_import(db, "opreturn_count", version, indexes)?,
            emptyoutput: ComputedBlockFull::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
            )?,
            unknownoutput: ComputedBlockFull::forced_import(
                db,
                "unknownoutput_count",
                version,
                indexes,
            )?,
            segwit,
            taproot_adoption,
            segwit_adoption,
        })
    }
}
