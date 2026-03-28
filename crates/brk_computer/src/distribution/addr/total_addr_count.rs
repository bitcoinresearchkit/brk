use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{indexes, internal::PerBlock};

use super::AddrCountsVecs;

/// Total address count (global + per-type) with all derived indexes
#[derive(Traversable)]
pub struct TotalAddrCountVecs<M: StorageMode = Rw> {
    pub all: PerBlock<StoredU64, M>,
    #[traversable(flatten)]
    pub by_addr_type: ByAddrType<PerBlock<StoredU64, M>>,
}

impl TotalAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = PerBlock::forced_import(db, "total_addr_count", version, indexes)?;

        let by_addr_type: ByAddrType<PerBlock<StoredU64>> = ByAddrType::new_with_name(|name| {
            PerBlock::forced_import(db, &format!("{name}_total_addr_count"), version, indexes)
        })?;

        Ok(Self { all, by_addr_type })
    }

    /// Eagerly compute total = addr_count + empty_addr_count.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        addr_count: &AddrCountsVecs,
        empty_addr_count: &AddrCountsVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.height.compute_add(
            max_from,
            &addr_count.all.height,
            &empty_addr_count.all.height,
            exit,
        )?;

        for ((_, total), ((_, addr), (_, empty))) in self.by_addr_type.iter_mut().zip(
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
