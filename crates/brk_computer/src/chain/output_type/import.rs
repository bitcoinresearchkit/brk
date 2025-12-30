use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    grouped::{ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes,
};

impl Vecs {
    pub fn forced_import(db: &Database, version: Version, indexes: &indexes::Vecs) -> Result<Self> {
        let v0 = Version::ZERO;
        let last = || VecBuilderOptions::default().add_last();
        let full_stats = || {
            VecBuilderOptions::default()
                .add_average()
                .add_minmax()
                .add_percentiles()
                .add_sum()
                .add_cumulative()
        };

        Ok(Self {
            indexes_to_p2a_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2a_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2ms_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2ms_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2pk33_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2pk33_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2pk65_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2pk65_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2pkh_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2pkh_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2sh_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2sh_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2tr_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2tr_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2wpkh_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2wpkh_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
            indexes_to_p2wsh_count: ComputedVecsFromHeight::forced_import(
                db,
                "p2wsh_count",
                Source::Compute,
                version + v0,
                indexes,
                full_stats(),
            )?,
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
            indexes_to_exact_utxo_count: ComputedVecsFromHeight::forced_import(
                db,
                "exact_utxo_count",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
        })
    }
}
