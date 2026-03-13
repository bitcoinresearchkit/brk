use brk_cohort::ByAddressType;
use brk_traversable::Traversable;
use brk_types::{BasisPoints32, StoredI64, StoredU64, Version};

use crate::{
    indexes,
    internal::{CachedWindowStarts, LazyRollingDeltasFromHeight},
};

use super::AddressCountsVecs;

type AddrDelta = LazyRollingDeltasFromHeight<StoredU64, StoredI64, BasisPoints32>;

#[derive(Clone, Traversable)]
pub struct DeltaVecs {
    pub all: AddrDelta,
    #[traversable(flatten)]
    pub by_addresstype: ByAddressType<AddrDelta>,
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

        let by_addresstype = address_count.by_addresstype.map_with_name(|name, addr| {
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
            by_addresstype,
        }
    }
}
