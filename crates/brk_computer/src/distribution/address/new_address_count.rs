use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CachedWindowStarts, PerBlockCumulativeWithSums},
};

use super::TotalAddressCountVecs;

/// New address count per block (global + per-type)
#[derive(Traversable)]
pub struct NewAddressCountVecs<M: StorageMode = Rw> {
    pub all: PerBlockCumulativeWithSums<StoredU64, StoredU64, M>,
    #[traversable(flatten)]
    pub by_address_type: ByAddressType<PerBlockCumulativeWithSums<StoredU64, StoredU64, M>>,
}

impl NewAddressCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let all = PerBlockCumulativeWithSums::forced_import(
            db,
            "new_address_count",
            version,
            indexes,
            cached_starts,
        )?;

        let by_address_type = ByAddressType::new_with_name(|name| {
            PerBlockCumulativeWithSums::forced_import(
                db,
                &format!("{name}_new_address_count"),
                version,
                indexes,
                cached_starts,
            )
        })?;

        Ok(Self {
            all,
            by_address_type,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        total_address_count: &TotalAddressCountVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute(max_from, exit, |height_vec| {
            Ok(height_vec.compute_change(max_from, &total_address_count.all.height, 1, exit)?)
        })?;

        for ((_, new), (_, total)) in self
            .by_address_type
            .iter_mut()
            .zip(total_address_count.by_address_type.iter())
        {
            new.compute(max_from, exit, |height_vec| {
                Ok(height_vec.compute_change(max_from, &total.height, 1, exit)?)
            })?;
        }

        Ok(())
    }
}
