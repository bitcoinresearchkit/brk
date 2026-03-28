use brk_cohort::ByAddrType;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, StoredI64, StoredU64, Version};

use crate::{
    indexes,
    internal::{CachedWindowStarts, LazyRollingDeltasFromHeight},
};

use super::AddrCountsVecs;

type AddrDelta = LazyRollingDeltasFromHeight<StoredU64, StoredI64, BasisPointsSigned32>;

#[derive(Clone, Traversable)]
pub struct DeltaVecs {
    pub all: AddrDelta,
    #[traversable(flatten)]
    pub by_addr_type: ByAddrType<AddrDelta>,
}

impl DeltaVecs {
    pub(crate) fn new(
        version: Version,
        addr_count: &AddrCountsVecs,
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let version = version + Version::TWO;

        let all = LazyRollingDeltasFromHeight::new(
            "addr_count",
            version,
            &addr_count.all.0.height,
            cached_starts,
            indexes,
        );

        let by_addr_type = addr_count.by_addr_type.map_with_name(|name, addr| {
            LazyRollingDeltasFromHeight::new(
                &format!("{name}_addr_count"),
                version,
                &addr.0.height,
                cached_starts,
                indexes,
            )
        });

        Self { all, by_addr_type }
    }
}
