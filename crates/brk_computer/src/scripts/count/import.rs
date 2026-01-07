use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedBlockFull, BinaryBlockFull, PercentageU64F32,
    },
    outputs,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        outputs: &outputs::Vecs,
    ) -> Result<Self> {
        let indexes_to_p2a_count = ComputedBlockFull::forced_import(
            db,
            "p2a_count",
            version,
            indexes,
        )?;
        let indexes_to_p2ms_count = ComputedBlockFull::forced_import(
            db,
            "p2ms_count",
            version,
            indexes,
        )?;
        let indexes_to_p2pk33_count = ComputedBlockFull::forced_import(
            db,
            "p2pk33_count",
            version,
            indexes,
        )?;
        let indexes_to_p2pk65_count = ComputedBlockFull::forced_import(
            db,
            "p2pk65_count",
            version,
            indexes,
        )?;
        let indexes_to_p2pkh_count = ComputedBlockFull::forced_import(
            db,
            "p2pkh_count",
            version,
            indexes,
        )?;
        let indexes_to_p2sh_count = ComputedBlockFull::forced_import(
            db,
            "p2sh_count",
            version,
            indexes,
        )?;
        let indexes_to_p2tr_count = ComputedBlockFull::forced_import(
            db,
            "p2tr_count",
            version,
            indexes,
        )?;
        let indexes_to_p2wpkh_count = ComputedBlockFull::forced_import(
            db,
            "p2wpkh_count",
            version,
            indexes,
        )?;
        let indexes_to_p2wsh_count = ComputedBlockFull::forced_import(
            db,
            "p2wsh_count",
            version,
            indexes,
        )?;

        // Aggregate counts (computed from per-type counts)
        let indexes_to_segwit_count = ComputedBlockFull::forced_import(
            db,
            "segwit_count",
            version,
            indexes,
        )?;

        // Adoption ratios (lazy)
        // Uses outputs.count.indexes_to_count as denominator (total output count)
        // At height level: per-block ratio; at dateindex level: sum-based ratio (% of new outputs)
        let indexes_to_taproot_adoption =
            BinaryBlockFull::from_height_and_txindex::<PercentageU64F32>(
                "taproot_adoption",
                version,
                indexes_to_p2tr_count.height.boxed_clone(),
                outputs.count.indexes_to_count.height.sum_cum.sum.0.boxed_clone(),
                &indexes_to_p2tr_count,
                &outputs.count.indexes_to_count,
            );
        let indexes_to_segwit_adoption =
            BinaryBlockFull::from_height_and_txindex::<PercentageU64F32>(
                "segwit_adoption",
                version,
                indexes_to_segwit_count.height.boxed_clone(),
                outputs.count.indexes_to_count.height.sum_cum.sum.0.boxed_clone(),
                &indexes_to_segwit_count,
                &outputs.count.indexes_to_count,
            );

        Ok(Self {
            indexes_to_p2a_count,
            indexes_to_p2ms_count,
            indexes_to_p2pk33_count,
            indexes_to_p2pk65_count,
            indexes_to_p2pkh_count,
            indexes_to_p2sh_count,
            indexes_to_p2tr_count,
            indexes_to_p2wpkh_count,
            indexes_to_p2wsh_count,
            indexes_to_opreturn_count: ComputedBlockFull::forced_import(
                db,
                "opreturn_count",
                version,
                indexes,
            )?,
            indexes_to_emptyoutput_count: ComputedBlockFull::forced_import(
                db,
                "emptyoutput_count",
                version,
                indexes,
            )?,
            indexes_to_unknownoutput_count: ComputedBlockFull::forced_import(
                db,
                "unknownoutput_count",
                version,
                indexes,
            )?,
            indexes_to_segwit_count,
            indexes_to_taproot_adoption,
            indexes_to_segwit_adoption,
        })
    }
}
