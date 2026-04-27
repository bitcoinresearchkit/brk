use brk_cohort::SpendableType;
use brk_error::Result;
use brk_types::{StoredU64, Version};
use vecdb::Database;

use super::{Vecs, WithInputTypes};
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
        let input_count =
            WithInputTypes::<PerBlockCumulativeRolling<StoredU64, StoredU64>>::forced_import_with(
                db,
                "input_count_bis",
                |t| format!("{t}_prevout_count"),
                version,
                indexes,
                cached_starts,
            )?;
        let tx_count =
            WithInputTypes::<PerBlockCumulativeRolling<StoredU64, StoredU64>>::forced_import_with(
                db,
                "non_coinbase_tx_count",
                |t| format!("tx_count_with_{t}_prevout"),
                version,
                indexes,
                cached_starts,
            )?;

        let input_share = SpendableType::try_new(|_, name| {
            PercentCumulativeRolling::forced_import(
                db,
                &format!("{name}_prevout_share"),
                version,
                indexes,
            )
        })?;

        let tx_share = SpendableType::try_new(|_, name| {
            PercentCumulativeRolling::forced_import(
                db,
                &format!("tx_share_with_{name}_prevout"),
                version,
                indexes,
            )
        })?;
        Ok(Self {
            input_count,
            input_share,
            tx_count,
            tx_share,
        })
    }
}
