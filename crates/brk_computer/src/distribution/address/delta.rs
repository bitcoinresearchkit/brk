use brk_cohort::ByAddressType;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, StoredI64, StoredU64, Version};

use crate::{
    indexes,
    internal::{CachedWindowStarts, LazyRollingDeltasFromHeight},
};

use super::AddressCountsVecs;

type AddrDelta = LazyRollingDeltasFromHeight<StoredU64, StoredI64, BasisPointsSigned32>;

#[derive(Clone, Traversable)]
pub struct DeltaVecs {
    pub all: AddrDelta,
    #[traversable(flatten)]
    pub by_address_type: ByAddressType<AddrDelta>,
}

impl DeltaVecs {
    pub(crate) fn new(
        version: Version,
        address_count: &AddressCountsVecs,
        cached_starts: &CachedWindowStarts,
        indexes: &indexes::Vecs,
    ) -> Self {
        let version = version + Version::TWO;

        let all = LazyRollingDeltasFromHeight::new(
            "address_count",
            version,
            &address_count.all.0.height,
            cached_starts,
            indexes,
        );

        let by_address_type = address_count.by_address_type.map_with_name(|name, addr| {
            LazyRollingDeltasFromHeight::new(
                &format!("{name}_address_count"),
                version,
                &addr.0.height,
                cached_starts,
                indexes,
            )
        });

        Self {
            all,
            by_address_type,
        }
    }
}
