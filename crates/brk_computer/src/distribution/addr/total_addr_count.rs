use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{indexes, internal::PerBlock};

use super::{AddrCountsVecs, WithAddrTypes};

/// Total address count (global + per-type) with all derived indexes.
#[derive(Deref, DerefMut, Traversable)]
pub struct TotalAddrCountVecs<M: StorageMode = Rw>(
    #[traversable(flatten)] pub WithAddrTypes<PerBlock<StoredU64, M>>,
);

impl TotalAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(WithAddrTypes::<PerBlock<StoredU64>>::forced_import(
            db,
            "total_addr_count",
            version,
            indexes,
        )?))
    }

    /// Eagerly compute total = addr_count + empty_addr_count.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        addr_count: &AddrCountsVecs,
        empty_addr_count: &AddrCountsVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.0.all.height.compute_add(
            max_from,
            &addr_count.all.height,
            &empty_addr_count.all.height,
            exit,
        )?;

        for ((_, total), ((_, addr), (_, empty))) in self.0.by_addr_type.iter_mut().zip(
            addr_count
                .by_addr_type
                .iter()
                .zip(empty_addr_count.by_addr_type.iter()),
        ) {
            total
                .height
                .compute_add(max_from, &addr.height, &empty.height, exit)?;
        }

        Ok(())
    }
}
