//! Storage for address data (loaded and empty addresses).

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    EmptyAddressData, EmptyAddressIndex, Height, LoadedAddressData, LoadedAddressIndex, Version,
};
use vecdb::{
    AnyStoredVec, BytesVec, Database, GenericStoredVec, ImportOptions, ImportableVec, Stamp,
};

const SAVED_STAMPED_CHANGES: u16 = 10;

/// Storage for both loaded and empty address data.
#[derive(Clone, Traversable)]
pub struct AddressesDataVecs {
    pub loaded: BytesVec<LoadedAddressIndex, LoadedAddressData>,
    pub empty: BytesVec<EmptyAddressIndex, EmptyAddressData>,
}

impl AddressesDataVecs {
    /// Import from database.
    pub fn forced_import(db: &Database, version: Version) -> Result<Self> {
        Ok(Self {
            loaded: BytesVec::forced_import_with(
                ImportOptions::new(db, "loadedaddressdata", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
            empty: BytesVec::forced_import_with(
                ImportOptions::new(db, "emptyaddressdata", version)
                    .with_saved_stamped_changes(SAVED_STAMPED_CHANGES),
            )?,
        })
    }

    /// Get minimum stamped height across loaded and empty data.
    pub fn min_stamped_height(&self) -> Height {
        Height::from(self.loaded.stamp())
            .incremented()
            .min(Height::from(self.empty.stamp()).incremented())
    }

    /// Rollback both loaded and empty data to before the given stamp.
    pub fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 2]> {
        Ok([
            self.loaded.rollback_before(stamp)?,
            self.empty.rollback_before(stamp)?,
        ])
    }

    /// Reset both loaded and empty data.
    pub fn reset(&mut self) -> Result<()> {
        self.loaded.reset()?;
        self.empty.reset()?;
        Ok(())
    }

    /// Flush both loaded and empty data with stamp.
    pub fn write(&mut self, stamp: Stamp, with_changes: bool) -> Result<()> {
        self.loaded
            .stamped_write_maybe_with_changes(stamp, with_changes)?;
        self.empty
            .stamped_write_maybe_with_changes(stamp, with_changes)?;
        Ok(())
    }
}
