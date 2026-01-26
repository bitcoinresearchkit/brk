//! New address count: delta of total_addr_count (global + per-type)

use brk_cohort::{ByAddressType, zip_by_addresstype};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, TypedVecIterator};

use crate::{ComputeIndexes, indexes, internal::LazyComputedFromHeightFull};

use super::TotalAddrCountVecs;

/// New addresses by type - lazy delta with stored dateindex stats
pub type NewAddrCountByType = ByAddressType<LazyComputedFromHeightFull<StoredU64, StoredU64>>;

/// New address count per block (global + per-type)
#[derive(Clone, Traversable)]
pub struct NewAddrCountVecs {
    pub all: LazyComputedFromHeightFull<StoredU64, StoredU64>,
    #[traversable(flatten)]
    pub by_addresstype: NewAddrCountByType,
}

impl NewAddrCountVecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        total_addr_count: &TotalAddrCountVecs,
    ) -> Result<Self> {
        let all = LazyComputedFromHeightFull::forced_import_with_init(
            db,
            "new_addr_count",
            version,
            total_addr_count.all.height.clone(),
            indexes,
            delta_init_fn,
        )?;

        let by_addresstype: NewAddrCountByType = zip_by_addresstype(
            &total_addr_count.by_addresstype,
            |name, total| {
                LazyComputedFromHeightFull::forced_import_with_init(
                    db,
                    &format!("{name}_new_addr_count"),
                    version,
                    total.height.clone(),
                    indexes,
                    delta_init_fn,
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

/// Delta init function: value[h] = source[h] - source[h-1]
fn delta_init_fn(
    h: Height,
    total_iter: &mut dyn TypedVecIterator<I = Height, T = StoredU64, Item = StoredU64>,
) -> Option<StoredU64> {
    let current: u64 = total_iter.get(h)?.into();
    let prev: u64 = h
        .decremented()
        .and_then(|prev_h| total_iter.get(prev_h))
        .map(|v: StoredU64| v.into())
        .unwrap_or(0);
    Some(StoredU64::from(current.saturating_sub(prev)))
}
