use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

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
        let v2 = Version::TWO;
        let last = || VecBuilderOptions::default().add_last();

        Ok(Self {
            indexes_to_sent_sum: ComputedValueVecsFromHeight::forced_import(
                db,
                "sent_sum",
                Source::Compute,
                version + v0,
                VecBuilderOptions::default().add_sum(),
                compute_dollars,
                indexes,
            )?,
            indexes_to_annualized_volume: ComputedVecsFromDateIndex::forced_import(
                db,
                "annualized_volume",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_annualized_volume_btc: ComputedVecsFromDateIndex::forced_import(
                db,
                "annualized_volume_btc",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_annualized_volume_usd: ComputedVecsFromDateIndex::forced_import(
                db,
                "annualized_volume_usd",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_tx_btc_velocity: ComputedVecsFromDateIndex::forced_import(
                db,
                "tx_btc_velocity",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_tx_usd_velocity: ComputedVecsFromDateIndex::forced_import(
                db,
                "tx_usd_velocity",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_tx_per_sec: ComputedVecsFromDateIndex::forced_import(
                db,
                "tx_per_sec",
                Source::Compute,
                version + v2,
                indexes,
                last(),
            )?,
            indexes_to_outputs_per_sec: ComputedVecsFromDateIndex::forced_import(
                db,
                "outputs_per_sec",
                Source::Compute,
                version + v2,
                indexes,
                last(),
            )?,
            indexes_to_inputs_per_sec: ComputedVecsFromDateIndex::forced_import(
                db,
                "inputs_per_sec",
                Source::Compute,
                version + v2,
                indexes,
                last(),
            )?,
        })
    }
}
