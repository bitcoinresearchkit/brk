use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedVecsFromHeight, LazyVecsFrom2FromHeight, PercentageU64F32, Source,
        VecBuilderOptions,
    },
    outputs,
    utils::OptionExt,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        outputs: &outputs::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

        let indexes_to_p2a_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2a_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2ms_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2ms_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2pk33_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2pk33_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2pk65_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2pk65_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2pkh_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2pkh_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2sh_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2sh_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2tr_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2tr_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2wpkh_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2wpkh_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;
        let indexes_to_p2wsh_count = ComputedVecsFromHeight::forced_import(
            db,
            "p2wsh_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;

        // Aggregate counts (computed from per-type counts)
        let indexes_to_segwit_count = ComputedVecsFromHeight::forced_import(
            db,
            "segwit_count",
            Source::Compute,
            version + v0,
            indexes,
            full_stats(),
        )?;

        // Adoption ratios (lazy)
        // Uses outputs.count.indexes_to_count as denominator (total output count)
        // At height level: per-block ratio; at dateindex level: sum-based ratio (% of new outputs)
        let indexes_to_taproot_adoption =
            LazyVecsFrom2FromHeight::from_height_and_txindex::<PercentageU64F32>(
                "taproot_adoption",
                version + v0,
                indexes_to_p2tr_count.height.u().boxed_clone(),
                outputs.count.indexes_to_count.height.sum.u().boxed_clone(),
                &indexes_to_p2tr_count,
                &outputs.count.indexes_to_count,
            );
        let indexes_to_segwit_adoption =
            LazyVecsFrom2FromHeight::from_height_and_txindex::<PercentageU64F32>(
                "segwit_adoption",
                version + v0,
                indexes_to_segwit_count.height.u().boxed_clone(),
                outputs.count.indexes_to_count.height.sum.u().boxed_clone(),
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
            indexes_to_opreturn_count: ComputedVecsFromHeight::forced_import(
                db,
                "opreturn_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_emptyoutput_count: ComputedVecsFromHeight::forced_import(
                db,
                "emptyoutput_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_unknownoutput_count: ComputedVecsFromHeight::forced_import(
                db,
                "unknownoutput_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_segwit_count,
            indexes_to_taproot_adoption,
            indexes_to_segwit_adoption,
        })
    }
}
