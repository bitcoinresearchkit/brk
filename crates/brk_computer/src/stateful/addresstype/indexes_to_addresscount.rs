use brk_error::Result;
use brk_structs::{ByAddressType, StoredU64};
use brk_vecs::IVecs;
use derive_deref::{Deref, DerefMut};
use vecdb::Exit;

use crate::{Indexes, grouped::ComputedVecsFromHeight, indexes};

use super::AddressTypeToHeightToAddressCount;

#[derive(Clone, Deref, DerefMut, IVecs)]
pub struct AddressTypeToIndexesToAddressCount(ByAddressType<ComputedVecsFromHeight<StoredU64>>);

impl From<ByAddressType<ComputedVecsFromHeight<StoredU64>>> for AddressTypeToIndexesToAddressCount {
    fn from(value: ByAddressType<ComputedVecsFromHeight<StoredU64>>) -> Self {
        Self(value)
    }
}

impl AddressTypeToIndexesToAddressCount {
    pub fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        addresstype_to_height_to_addresscount: &AddressTypeToHeightToAddressCount,
    ) -> Result<()> {
        self.p2pk65.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2pk65),
        )?;
        self.p2pk33.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2pk33),
        )?;
        self.p2pkh.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2pkh),
        )?;
        self.p2sh.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2sh),
        )?;
        self.p2wpkh.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2wpkh),
        )?;
        self.p2wsh.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2wsh),
        )?;
        self.p2tr.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2tr),
        )?;
        self.p2a.compute_rest(
            indexes,
            starting_indexes,
            exit,
            Some(&addresstype_to_height_to_addresscount.p2a),
        )?;
        Ok(())
    }
}
