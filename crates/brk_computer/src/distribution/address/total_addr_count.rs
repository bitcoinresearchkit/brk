//! Total address count: addr_count + empty_addr_count (global + per-type)

use brk_cohort::{ByAddressType, zip2_by_addresstype};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{StoredU64, Version};
use vecdb::ReadableCloneableVec;

use crate::{indexes, internal::{LazyBinaryComputedFromHeightLast, U64Plus}};

use super::AddrCountsVecs;

/// Total addresses by type - lazy sum with all derived indexes
pub type TotalAddrCountByType =
    ByAddressType<LazyBinaryComputedFromHeightLast<StoredU64, StoredU64, StoredU64>>;

/// Total address count (global + per-type) with all derived indexes
#[derive(Clone, Traversable)]
pub struct TotalAddrCountVecs {
    pub all: LazyBinaryComputedFromHeightLast<StoredU64, StoredU64, StoredU64>,
    #[traversable(flatten)]
    pub by_addresstype: TotalAddrCountByType,
}

impl TotalAddrCountVecs {
    pub(crate) fn forced_import(
        version: Version,
        indexes: &indexes::Vecs,
        addr_count: &AddrCountsVecs,
        empty_addr_count: &AddrCountsVecs,
    ) -> Result<Self> {
        let all = LazyBinaryComputedFromHeightLast::forced_import::<U64Plus>(
            "total_addr_count",
            version,
            addr_count.all.count.height.read_only_boxed_clone(),
            empty_addr_count.all.count.height.read_only_boxed_clone(),
            indexes,
        );

        let by_addresstype: TotalAddrCountByType = zip2_by_addresstype(
            &addr_count.by_addresstype,
            &empty_addr_count.by_addresstype,
            |name, addr, empty| {
                Ok(LazyBinaryComputedFromHeightLast::forced_import::<U64Plus>(
                    &format!("{name}_total_addr_count"),
                    version,
                    addr.count.height.read_only_boxed_clone(),
                    empty.count.height.read_only_boxed_clone(),
                    indexes,
                ))
            },
        )?;

        Ok(Self { all, by_addresstype })
    }

}
