use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    grouped::{ComputedValueVecsFromHeight, ComputedVecsFromDateIndex, Source, VecBuilderOptions},
    indexes,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let last = || VecBuilderOptions::default().add_last();

        Ok(Self {
            height_to_24h_coinbase_sum: EagerVec::forced_import(
                db,
                "24h_coinbase_sum",
                version + v0,
            )?,
            height_to_24h_coinbase_usd_sum: EagerVec::forced_import(
                db,
                "24h_coinbase_usd_sum",
                version + v0,
            )?,
            indexes_to_coinbase: ComputedValueVecsFromHeight::forced_import(
                db,
                "coinbase",
                Source::Compute,
                version + v0,
                VecBuilderOptions::default()
                    .add_sum()
                    .add_cumulative()
                    .add_percentiles()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_subsidy: ComputedValueVecsFromHeight::forced_import(
                db,
                "subsidy",
                Source::Compute,
                version + v0,
                VecBuilderOptions::default()
                    .add_percentiles()
                    .add_sum()
                    .add_cumulative()
                    .add_minmax()
                    .add_average(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_unclaimed_rewards: ComputedValueVecsFromHeight::forced_import(
                db,
                "unclaimed_rewards",
                Source::Compute,
                version + v0,
                VecBuilderOptions::default().add_sum().add_cumulative(),
                compute_dollars,
                indexes,
            )?,
            dateindex_to_fee_dominance: EagerVec::forced_import(db, "fee_dominance", version + v0)?,
            dateindex_to_subsidy_dominance: EagerVec::forced_import(
                db,
                "subsidy_dominance",
                version + v0,
            )?,
            indexes_to_subsidy_usd_1y_sma: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        "subsidy_usd_1y_sma",
                        Source::Compute,
                        version + v0,
                        indexes,
                        last(),
                    )
                })
                .transpose()?,
            indexes_to_puell_multiple: compute_dollars
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        db,
                        "puell_multiple",
                        Source::Compute,
                        version + v0,
                        indexes,
                        last(),
                    )
                })
                .transpose()?,
            indexes_to_inflation_rate: ComputedVecsFromDateIndex::forced_import(
                db,
                "inflation_rate",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
        })
    }
}
