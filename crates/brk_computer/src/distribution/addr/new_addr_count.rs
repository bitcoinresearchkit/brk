use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PerBlockCumulativeRolling, WindowStartVec, Windows, WithAddrTypes},
};

use super::TotalAddrCountVecs;

/// New address count per block (global + per-type).
#[derive(Deref, DerefMut, Traversable)]
pub struct NewAddrCountVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PerBlockCumulativeRolling<StoredU64, StoredU64, M>>,
);

impl NewAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<
            PerBlockCumulativeRolling<StoredU64, StoredU64>,
        >::forced_import(
            db,
            "new_addr_count",
            version,
            indexes,
            cached_starts,
        )?))
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        total_addr_count: &TotalAddrCountVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.0.all.compute(max_from, exit, |height_vec| {
            Ok(height_vec.compute_change(max_from, &total_addr_count.all.height, 1, exit)?)
        })?;

        for ((_, new), (_, total)) in self
            .0
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
