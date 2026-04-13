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
            by_type: ByAddrType::new_with_name(|name| {
                PerBlockCumulativeRolling::forced_import(
                    db,
                    &format!("tx_count_with_{name}_in"),
                    version,
                    indexes,
                    cached_starts,
                )
            })?,
            percent: ByAddrType::new_with_name(|name| {
                PercentCumulativeRolling::forced_import(
                    db,
                    &format!("tx_count_with_{name}_in_rel_to_all"),
                    version,
                    indexes,
                )
            })?,
        })
    }
}
