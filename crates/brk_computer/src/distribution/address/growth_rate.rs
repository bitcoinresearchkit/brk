//! Growth rate: new_addr_count / addr_count (global + per-type)

use brk_cohort::{ByAddressType, zip2_by_addresstype};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF32, StoredU64, Version};
use vecdb::{Database, Exit, IterableCloneableVec};

use crate::{
    ComputeIndexes, indexes,
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
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        new_addr_count: &NewAddrCountVecs,
        addr_count: &AddrCountsVecs,
    ) -> Result<Self> {
        let all = make_growth_rate(
            db,
            "growth_rate",
            version,
            indexes,
            &new_addr_count.all.height,
            &addr_count.all.count.height,
        )?;

        let by_addresstype: GrowthRateByType = zip2_by_addresstype(
            &new_addr_count.by_addresstype,
            &addr_count.by_addresstype,
            |name, new, addr| {
                make_growth_rate(
                    db,
                    &format!("{name}_growth_rate"),
                    version,
                    indexes,
                    &new.height,
                    &addr.count.height,
                )
            },
        )?;

        Ok(Self { all, by_addresstype })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.all.derive_from(indexes, starting_indexes, exit)?;
        for vecs in self.by_addresstype.values_mut() {
            vecs.derive_from(indexes, starting_indexes, exit)?;
        }
        Ok(())
    }
}

fn make_growth_rate<V1, V2>(
    db: &Database,
    name: &str,
    version: Version,
    indexes: &indexes::Vecs,
    new: &V1,
    addr: &V2,
) -> Result<LazyBinaryComputedFromHeightDistribution<StoredF32, StoredU64, StoredU64>>
where
    V1: IterableCloneableVec<Height, StoredU64>,
    V2: IterableCloneableVec<Height, StoredU64>,
{
    LazyBinaryComputedFromHeightDistribution::<StoredF32, StoredU64, StoredU64>::forced_import::<
        RatioU64F32,
    >(db, name, version, new.boxed_clone(), addr.boxed_clone(), indexes)
}
