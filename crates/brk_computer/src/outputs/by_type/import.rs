use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
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
        Ok(Self {
            output_count: ByAddrType::new_with_name(|name| {
                PerBlockCumulativeRolling::forced_import(
                    db,
                    &format!("{name}_output_count"),
                    version,
                    indexes,
                    cached_starts,
                )
            })?,
            tx_count: ByAddrType::new_with_name(|name| {
                PerBlockCumulativeRolling::forced_import(
                    db,
                    &format!("tx_count_with_{name}_out"),
                    version,
                    indexes,
                    cached_starts,
                )
            })?,
            tx_percent: ByAddrType::new_with_name(|name| {
                PercentCumulativeRolling::forced_import(
                    db,
                    &format!("tx_count_with_{name}_out_rel_to_all"),
                    version,
                    indexes,
                )
            })?,
        })
    }
}
