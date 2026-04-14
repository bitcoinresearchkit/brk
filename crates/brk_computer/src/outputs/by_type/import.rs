use brk_cohort::ByType;
use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::Database;

use super::{Vecs, WithOutputTypes};
use crate::{
    indexes,
    internal::{PerBlockCumulativeRolling, PercentCumulativeRolling, WindowStartVec, Windows},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let output_count = WithOutputTypes::<
            PerBlockCumulativeRolling<StoredU64, StoredU64>,
        >::forced_import_with(
            db,
            "output_count_bis",
            |t| format!("{t}_output_count"),
            version,
            indexes,
            cached_starts,
        )?;
        let tx_count = WithOutputTypes::<
            PerBlockCumulativeRolling<StoredU64, StoredU64>,
        >::forced_import_with(
            db,
            "tx_count_bis",
            |t| format!("tx_count_with_{t}_output"),
            version,
            indexes,
            cached_starts,
        )?;

        let spendable_output_count = PerBlockCumulativeRolling::forced_import(
            db,
            "spendable_output_count",
            version,
            indexes,
            cached_starts,
        )?;

        let output_share = ByType::try_new(|_, name| {
            PercentCumulativeRolling::forced_import(
                db,
                &format!("{name}_output_share"),
                version,
                indexes,
            )
        })?;

        let tx_share = ByType::try_new(|_, name| {
            PercentCumulativeRolling::forced_import(
                db,
                &format!("tx_share_with_{name}_output"),
                version,
                indexes,
            )
        })?;
        Ok(Self {
            output_count,
            spendable_output_count,
            output_share,
            tx_count,
            tx_share,
        })
    }
}
