//! Growth rate: new_addr_count / addr_count (global + per-type)

use brk_cohort::{ByAddressType, zip2_by_addresstype};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, StoredU64, Version};
use vecdb::ReadableCloneableVec;

use crate::{
    indexes,
    internal::{LazyBinaryComputedFromHeightDistribution, RatioU64F32},
};

use super::{AddrCountsVecs, NewAddrCountVecs};

/// Growth rate by type - lazy ratio with distribution stats
pub type GrowthRateByType =
    ByAddressType<LazyBinaryComputedFromHeightDistribution<StoredF32, StoredU64, StoredU64>>;

/// Growth rate: new_addr_count / addr_count (global + per-type)
#[derive(Clone, Traversable)]
pub struct GrowthRateVecs {
    pub all: LazyBinaryComputedFromHeightDistribution<StoredF32, StoredU64, StoredU64>,
    #[traversable(flatten)]
    pub by_addresstype: GrowthRateByType,
}

impl GrowthRateVecs {
    pub(crate) fn forced_import(
        version: Version,
        indexes: &indexes::Vecs,
        new_addr_count: &NewAddrCountVecs,
        addr_count: &AddrCountsVecs,
    ) -> Result<Self> {
        let all = make_growth_rate(
            "growth_rate",
            version,
            indexes,
            &new_addr_count.all.height,
            &addr_count.all.count.height,
        );

        let by_addresstype: GrowthRateByType = zip2_by_addresstype(
            &new_addr_count.by_addresstype,
            &addr_count.by_addresstype,
            |name, new, addr| {
                Ok(make_growth_rate(
                    &format!("{name}_growth_rate"),
                    version,
                    indexes,
                    &new.height,
                    &addr.count.height,
                ))
            },
        )?;

        Ok(Self { all, by_addresstype })
    }
}

fn make_growth_rate<V1, V2>(
    name: &str,
    version: Version,
    indexes: &indexes::Vecs,
    new: &V1,
    addr: &V2,
) -> LazyBinaryComputedFromHeightDistribution<StoredF32, StoredU64, StoredU64>
where
    V1: ReadableCloneableVec<Height, StoredU64>,
    V2: ReadableCloneableVec<Height, StoredU64>,
{
    LazyBinaryComputedFromHeightDistribution::<StoredF32, StoredU64, StoredU64>::forced_import::<
        RatioU64F32,
    >(name, version, new.read_only_boxed_clone(), addr.read_only_boxed_clone(), indexes)
}
