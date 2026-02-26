//! New address count: delta of total_addr_count (global + per-type)

//! New address count: delta of total_addr_count (global + per-type)

use brk_cohort::{ByAddressType, zip_by_addresstype};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use vecdb::{Database, Exit, Ident, Rw, StorageMode};

use crate::{indexes, internal::{LazyComputedFromHeightFull, WindowStarts}};

use super::TotalAddrCountVecs;

/// New address count per block (global + per-type)
#[derive(Traversable)]
pub struct NewAddrCountVecs<M: StorageMode = Rw> {
    pub all: LazyComputedFromHeightFull<StoredU64, StoredU64, M>,
    #[traversable(flatten)]
    pub by_addresstype: ByAddressType<LazyComputedFromHeightFull<StoredU64, StoredU64, M>>,
}

impl NewAddrCountVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        total_addr_count: &TotalAddrCountVecs,
    ) -> Result<Self> {
        let all = LazyComputedFromHeightFull::forced_import::<Ident>(
            db,
            "new_addr_count",
            version,
            &total_addr_count.all.height,
            indexes,
        )?;

        let by_addresstype: ByAddressType<LazyComputedFromHeightFull<StoredU64, StoredU64>> =
            zip_by_addresstype(&total_addr_count.by_addresstype, |name, total| {
                LazyComputedFromHeightFull::forced_import::<Ident>(
                    db,
                    &format!("{name}_new_addr_count"),
                    version,
                    &total.height,
                    indexes,
                )
            })?;

        Ok(Self {
            all,
            by_addresstype,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        windows: &WindowStarts<'_>,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute(max_from, windows, exit)?;
        for vecs in self.by_addresstype.values_mut() {
            vecs.compute(max_from, windows, exit)?;
        }
        Ok(())
    }
}
