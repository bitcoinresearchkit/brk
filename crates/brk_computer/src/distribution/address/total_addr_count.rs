//! Total address count: addr_count + empty_addr_count (global + per-type)

use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{indexes, internal::ComputedFromHeightLast};

use super::AddrCountsVecs;

/// Total address count (global + per-type) with all derived indexes
#[derive(Traversable)]
pub struct TotalAddrCountVecs<M: StorageMode = Rw> {
    pub all: ComputedFromHeightLast<StoredU64, M>,
    #[traversable(flatten)]
    pub by_addresstype: ByAddressType<ComputedFromHeightLast<StoredU64, M>>,
}

impl TotalAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = ComputedFromHeightLast::forced_import(
            db,
            "total_addr_count",
            version,
            indexes,
        )?;

        let by_addresstype: ByAddressType<ComputedFromHeightLast<StoredU64>> = ByAddressType::new_with_name(
            |name| {
                ComputedFromHeightLast::forced_import(
                    db,
                    &format!("{name}_total_addr_count"),
                    version,
                    indexes,
                )
            },
        )?;

        Ok(Self { all, by_addresstype })
    }

    /// Eagerly compute total = addr_count + empty_addr_count.
    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        addr_count: &AddrCountsVecs,
        empty_addr_count: &AddrCountsVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.height.compute_transform2(
            max_from,
            &addr_count.all.count.height,
            &empty_addr_count.all.count.height,
            |(h, a, b, ..)| (h, StoredU64::from(*a + *b)),
            exit,
        )?;

        for ((_, total), ((_, addr), (_, empty))) in self
            .by_addresstype
            .iter_mut()
            .zip(
                addr_count
                    .by_addresstype
                    .iter()
                    .zip(empty_addr_count.by_addresstype.iter()),
            )
        {
            total.height.compute_transform2(
                max_from,
                &addr.count.height,
                &empty.count.height,
                |(h, a, b, ..)| (h, StoredU64::from(*a + *b)),
                exit,
            )?;
        }

        Ok(())
    }
}
