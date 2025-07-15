use brk_core::{ByAddressType, Height, Result, StoredUsize};
use brk_exit::Exit;
use brk_vec::EagerVec;
use derive_deref::{Deref, DerefMut};

use crate::stateful::addresstype_to_addresscount::AddressTypeToAddressCount;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct AddressTypeToHeightToAddressCount(ByAddressType<EagerVec<Height, StoredUsize>>);

impl From<ByAddressType<EagerVec<Height, StoredUsize>>> for AddressTypeToHeightToAddressCount {
    fn from(value: ByAddressType<EagerVec<Height, StoredUsize>>) -> Self {
        Self(value)
    }
}

impl AddressTypeToHeightToAddressCount {
    pub fn forced_push_at(
        &mut self,
        height: Height,
        addresstype_to_usize: &AddressTypeToAddressCount,
        exit: &Exit,
    ) -> Result<()> {
        self.p2pk65
            .forced_push_at(height, addresstype_to_usize.p2pk65.into(), exit)?;
        self.p2pk33
            .forced_push_at(height, addresstype_to_usize.p2pk33.into(), exit)?;
        self.p2pkh
            .forced_push_at(height, addresstype_to_usize.p2pkh.into(), exit)?;
        self.p2sh
            .forced_push_at(height, addresstype_to_usize.p2sh.into(), exit)?;
        self.p2wpkh
            .forced_push_at(height, addresstype_to_usize.p2wpkh.into(), exit)?;
        self.p2wsh
            .forced_push_at(height, addresstype_to_usize.p2wsh.into(), exit)?;
        self.p2tr
            .forced_push_at(height, addresstype_to_usize.p2tr.into(), exit)?;
        self.p2a
            .forced_push_at(height, addresstype_to_usize.p2a.into(), exit)?;

        Ok(())
    }
}
