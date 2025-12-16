use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64};
use derive_deref::{Deref, DerefMut};
use vecdb::{PcoVec, EagerVec, GenericStoredVec};

use super::AddressTypeToAddressCount;

#[derive(Debug, Clone, Deref, DerefMut, Traversable)]
pub struct AddressTypeToHeightToAddressCount(ByAddressType<EagerVec<PcoVec<Height, StoredU64>>>);

impl From<ByAddressType<EagerVec<PcoVec<Height, StoredU64>>>> for AddressTypeToHeightToAddressCount {
    #[inline]
    fn from(value: ByAddressType<EagerVec<PcoVec<Height, StoredU64>>>) -> Self {
        Self(value)
    }
}

impl AddressTypeToHeightToAddressCount {
    pub fn truncate_push(
        &mut self,
        height: Height,
        addresstype_to_usize: &AddressTypeToAddressCount,
    ) -> Result<()> {
        self.p2pk65
            .truncate_push(height, addresstype_to_usize.p2pk65.into())?;
        self.p2pk33
            .truncate_push(height, addresstype_to_usize.p2pk33.into())?;
        self.p2pkh
            .truncate_push(height, addresstype_to_usize.p2pkh.into())?;
        self.p2sh
            .truncate_push(height, addresstype_to_usize.p2sh.into())?;
        self.p2wpkh
            .truncate_push(height, addresstype_to_usize.p2wpkh.into())?;
        self.p2wsh
            .truncate_push(height, addresstype_to_usize.p2wsh.into())?;
        self.p2tr
            .truncate_push(height, addresstype_to_usize.p2tr.into())?;
        self.p2a
            .truncate_push(height, addresstype_to_usize.p2a.into())?;

        Ok(())
    }
}
