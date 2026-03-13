use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{indexes, internal::PerBlock};

use super::AddressCountsVecs;

/// Total address count (global + per-type) with all derived indexes
#[derive(Traversable)]
pub struct TotalAddressCountVecs<M: StorageMode = Rw> {
    pub all: PerBlock<StoredU64, M>,
    #[traversable(flatten)]
    pub by_address_type: ByAddressType<PerBlock<StoredU64, M>>,
}

impl TotalAddressCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = PerBlock::forced_import(db, "total_address_count", version, indexes)?;

        let by_address_type: ByAddressType<PerBlock<StoredU64>> =
            ByAddressType::new_with_name(|name| {
                PerBlock::forced_import(
                    db,
                    &format!("{name}_total_address_count"),
                    version,
                    indexes,
                )
            })?;

        Ok(Self {
            all,
            by_address_type,
        })
    }

    /// Eagerly compute total = address_count + empty_address_count.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        address_count: &AddressCountsVecs,
        empty_address_count: &AddressCountsVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.height.compute_add(
            max_from,
            &address_count.all.height,
            &empty_address_count.all.height,
            exit,
        )?;

        for ((_, total), ((_, addr), (_, empty))) in self.by_address_type.iter_mut().zip(
            address_count
                .by_address_type
                .iter()
                .zip(empty_address_count.by_address_type.iter()),
        ) {
            total
                .height
                .compute_add(max_from, &addr.height, &empty.height, exit)?;
        }

        Ok(())
    }
}
