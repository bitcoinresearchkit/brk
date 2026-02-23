use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    EmptyAddressData, EmptyAddressIndex, FundedAddressData, FundedAddressIndex, Height,
};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, BytesVec, Rw, Stamp, StorageMode, WritableVec,
};

/// Storage for both funded and empty address data.
#[derive(Traversable)]
pub struct AddressesDataVecs<M: StorageMode = Rw> {
    pub funded: M::Stored<BytesVec<FundedAddressIndex, FundedAddressData>>,
    pub empty: M::Stored<BytesVec<EmptyAddressIndex, EmptyAddressData>>,
}

impl AddressesDataVecs {
    /// Get minimum stamped height across funded and empty data.
    pub(crate) fn min_stamped_height(&self) -> Height {
        Height::from(self.funded.stamp())
            .incremented()
            .min(Height::from(self.empty.stamp()).incremented())
    }

    /// Rollback both funded and empty data to before the given stamp.
    pub(crate) fn rollback_before(&mut self, stamp: Stamp) -> Result<[Stamp; 2]> {
        Ok([
            self.funded.rollback_before(stamp)?,
            self.empty.rollback_before(stamp)?,
        ])
    }

    /// Reset both funded and empty data.
    pub(crate) fn reset(&mut self) -> Result<()> {
        self.funded.reset()?;
        self.empty.reset()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.funded as &mut dyn AnyStoredVec,
            &mut self.empty as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }
}
