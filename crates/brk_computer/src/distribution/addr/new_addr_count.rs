use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlockCumulativeRolling},
};

use super::TotalAddrCountVecs;

/// New address count per block (global + per-type)
#[derive(Traversable)]
pub struct NewAddrCountVecs<M: StorageMode = Rw> {
    pub all: PerBlockCumulativeRolling<StoredU64, StoredU64, M>,
    #[traversable(flatten)]
    pub by_addr_type: ByAddrType<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
}

impl NewAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let all = PerBlockCumulativeRolling::forced_import(
            db,
            "new_addr_count",
            version,
            indexes,
            cached_starts,
        )?;

        let by_addr_type = ByAddrType::new_with_name(|name| {
            PerBlockCumulativeRolling::forced_import(
                db,
                &format!("{name}_new_addr_count"),
                version,
                indexes,
                cached_starts,
            )
        })?;

        Ok(Self { all, by_addr_type })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        total_addr_count: &TotalAddrCountVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute(max_from, exit, |height_vec| {
            Ok(height_vec.compute_change(max_from, &total_addr_count.all.height, 1, exit)?)
        })?;

        for ((_, new), (_, total)) in self
            .by_addr_type
            .iter_mut()
            .zip(total_addr_count.by_addr_type.iter())
        {
            new.compute(max_from, exit, |height_vec| {
                Ok(height_vec.compute_change(max_from, &total.height, 1, exit)?)
            })?;
        }

        Ok(())
    }
}
