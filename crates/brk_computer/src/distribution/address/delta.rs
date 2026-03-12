use brk_cohort::ByAddressType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredI64, StoredU64, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{WindowStarts, RollingDelta},
};

use super::AddressCountsVecs;

#[derive(Traversable)]
pub struct DeltaVecs<M: StorageMode = Rw> {
    pub all: RollingDelta<StoredU64, StoredI64, M>,
    #[traversable(flatten)]
    pub by_addresstype: ByAddressType<RollingDelta<StoredU64, StoredI64, M>>,
}

impl DeltaVecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let version = version + Version::TWO;

        let all = RollingDelta::forced_import(db, "address_count", version, indexes)?;

        let by_addresstype = ByAddressType::new_with_name(|name| {
            RollingDelta::forced_import(db, &format!("{name}_address_count"), version, indexes)
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
        address_count: &AddressCountsVecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all
            .compute(max_from, windows, &address_count.all.height, exit)?;

        for ((_, growth), (_, addr)) in self
            .by_addresstype
            .iter_mut()
            .zip(address_count.by_addresstype.iter())
        {
            growth.compute(max_from, windows, &addr.height, exit)?;
        }

        Ok(())
    }
}
