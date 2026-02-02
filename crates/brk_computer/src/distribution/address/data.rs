use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    EmptyAddressData, EmptyAddressIndex, FundedAddressData, FundedAddressIndex, Height, Version,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportOptions, ImportableVec, Stamp,
};

const SAVED_STAMPED_CHANGES: u16 = 10;

/// Storage for both funded and empty address data.
#[derive(Clone, Traversable)]
pub struct AddressesDataVecs {
    pub funded: BytesVec<FundedAddressIndex, FundedAddressData>,
    pub empty: BytesVec<EmptyAddressIndex, EmptyAddressData>,
}

impl AddressesDataVecs {
    /// Import from database.
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            funded: BytesVec::forced_import_with(
                ImportOptions::new(db, "fundedaddressdata", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            empty: BytesVec::forced_import_with(
                ImportOptions::new(db, "emptyaddressdata", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
        })
    }

    /// Get minimum stamped height across funded and empty data.
    pub fn min_stamped_height(&self) -> Height {
        Height::from(self.funded.stamp())
            .incremented()
            .min(Height::from(self.empty.stamp()).incremented())
    }

    /// Rollback both funded and empty data to before the given stamp.
    pub fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 2]> {
        Ok([
            self.funded.rollback_before(stamp)?,
            self.empty.rollback_before(stamp)?,
        ])
    }

    /// Reset both funded and empty data.
    pub fn reset(&mut self) -> Result<()> {
        self.funded.reset()?;
        self.empty.reset()?;
        Ok(())
    }

    /// Flush both funded and empty data with stamp.
    pub fn write(&mut self, stamp: Stamp, with_changes: bool) -> Result<()> {
        self.funded
            .stamped_write_maybe_with_changes(stamp, with_changes)?;
        self.empty
            .stamped_write_maybe_with_changes(stamp, with_changes)?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.funded as &mut dyn AnyStoredVec,
            &mut self.empty as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }
}
