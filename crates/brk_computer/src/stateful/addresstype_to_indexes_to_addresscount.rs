use brk_core::{ByAddressType, StoredUsize};
use brk_exit::Exit;
use brk_vec::AnyCollectableVec;
use derive_deref::{Deref, DerefMut};

use crate::{
    Indexes, grouped::ComputedVecsFromHeight, indexes,
    stateful::addresstype_to_height_to_addresscount::AddressTypeToHeightToAddressCount,
};

#[derive(Clone, Deref, DerefMut)]
pub struct AddressTypeToIndexesToAddressCount(ByAddressType<ComputedVecsFromHeight<StoredUsize>>);

impl From<ByAddressType<ComputedVecsFromHeight<StoredUsize>>>
    for AddressTypeToIndexesToAddressCount
{
    fn from(value: ByAddressType<ComputedVecsFromHeight<StoredUsize>>) -> Self {
        Self(value)
    }
}

impl AddressTypeToIndexesToAddressCount {
    pub fn compute(
        &mut self,
        // height: Height,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
        addresstype_to_height_to_addresscount: &AddressTypeToHeightToAddressCount,
    ) -> color_eyre::Result<()> {
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
    //
    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        self.0
            .as_typed_vec()
            .into_iter()
            .flat_map(|(_, v)| v.vecs())
            .collect::<Vec<_>>()
    }
}
