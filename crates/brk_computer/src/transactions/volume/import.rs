use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedDateLast, ValueBlockSum},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        Ok(Self {
            indexes_to_sent_sum: ValueBlockSum::forced_import(
                db,
                "sent_sum",
                version,
                indexes,
                compute_dollars,
            )?,
            indexes_to_annualized_volume: ComputedDateLast::forced_import(
                db,
                "annualized_volume",
                version,
                indexes,
            )?,
            indexes_to_annualized_volume_btc: ComputedDateLast::forced_import(
                db,
                "annualized_volume_btc",
                version,
                indexes,
            )?,
            indexes_to_annualized_volume_usd: ComputedDateLast::forced_import(
                db,
                "annualized_volume_usd",
                version,
                indexes,
            )?,
            indexes_to_tx_per_sec: ComputedDateLast::forced_import(
                db,
                "tx_per_sec",
                version + v2,
                indexes,
            )?,
            indexes_to_outputs_per_sec: ComputedDateLast::forced_import(
                db,
                "outputs_per_sec",
                version + v2,
                indexes,
            )?,
            indexes_to_inputs_per_sec: ComputedDateLast::forced_import(
                db,
                "inputs_per_sec",
                version + v2,
                indexes,
            )?,
        })
    }
}
