use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, StoredI64, StoredU64, Version};
use derive_more::{Deref, DerefMut};

use crate::{
    indexes,
    internal::{LazyRollingDeltasFromHeight, WindowStartVec, Windows},
};

use super::{AddrCountsVecs, WithAddrTypes};

type AddrDelta = LazyRollingDeltasFromHeight<StoredU64, StoredI64, BasisPointsSigned32>;

#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct DeltaVecs(#[traversable(flatten)] pub WithAddrTypes<AddrDelta>);

impl DeltaVecs {
    pub(crate) fn new(
        version: Version,
        addr_count: &AddrCountsVecs,
        cached_starts: &Windows<&WindowStartVec>,
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

        Self(WithAddrTypes { all, by_addr_type })
    }
}
