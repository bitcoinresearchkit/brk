use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{ComputedPerBlockSum, WindowStarts},
};

use super::TotalAddressCountVecs;

/// New address count per block (global + per-type)
#[derive(Traversable)]
pub struct NewAddressCountVecs<M: StorageMode = Rw> {
    pub all: ComputedPerBlockSum<StoredU64, M>,
    #[traversable(flatten)]
    pub by_addresstype: ByAddressType<ComputedPerBlockSum<StoredU64, M>>,
}

impl NewAddressCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = ComputedPerBlockSum::forced_import(db, "new_address_count", version, indexes)?;

        let by_addresstype: ByAddressType<ComputedPerBlockSum<StoredU64>> =
            ByAddressType::new_with_name(|name| {
                ComputedPerBlockSum::forced_import(
                    db,
                    &format!("{name}_new_address_count"),
                    version,
                    indexes,
                )
            })?;

        Ok(Self {
            all,
            by_addresstype,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        total_address_count: &TotalAddressCountVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute(max_from, windows, exit, |height_vec| {
            Ok(height_vec.compute_change(max_from, &total_address_count.all.height, 1, exit)?)
        })?;

        for ((_, new), (_, total)) in self
            .by_addresstype
            .iter_mut()
            .zip(total_address_count.by_addresstype.iter())
        {
            new.compute(max_from, windows, exit, |height_vec| {
                Ok(height_vec.compute_change(max_from, &total.height, 1, exit)?)
            })?;
        }

        Ok(())
    }
}
