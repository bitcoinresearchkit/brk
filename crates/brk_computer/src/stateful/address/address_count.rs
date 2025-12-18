//! Address count types per address type.

use brk_error::Result;
use brk_grouper::ByAddressType;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Version};
use derive_deref::{Deref, DerefMut};
use vecdb::{Database, EagerVec, Exit, GenericStoredVec, ImportableVec, PcoVec, TypedVecIterator};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes,
};

/// Address count per address type (runtime state).
#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToAddressCount(ByAddressType<u64>);

impl From<(&AddressTypeToHeightToAddressCount, Height)> for AddressTypeToAddressCount {
    #[inline]
    fn from((groups, starting_height): (&AddressTypeToHeightToAddressCount, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            Self(ByAddressType {
                p2pk65: groups.p2pk65.into_iter().get_unwrap(prev_height).into(),
                p2pk33: groups.p2pk33.into_iter().get_unwrap(prev_height).into(),
                p2pkh: groups.p2pkh.into_iter().get_unwrap(prev_height).into(),
                p2sh: groups.p2sh.into_iter().get_unwrap(prev_height).into(),
                p2wpkh: groups.p2wpkh.into_iter().get_unwrap(prev_height).into(),
                p2wsh: groups.p2wsh.into_iter().get_unwrap(prev_height).into(),
                p2tr: groups.p2tr.into_iter().get_unwrap(prev_height).into(),
                p2a: groups.p2a.into_iter().get_unwrap(prev_height).into(),
            })
        } else {
            Default::default()
        }
    }
}

/// Address count per address type, indexed by height.
#[derive(Debug, Clone, Deref, DerefMut, Traversable)]
pub struct AddressTypeToHeightToAddressCount(ByAddressType<EagerVec<PcoVec<Height, StoredU64>>>);

impl From<ByAddressType<EagerVec<PcoVec<Height, StoredU64>>>>
    for AddressTypeToHeightToAddressCount
{
    #[inline]
    fn from(value: ByAddressType<EagerVec<PcoVec<Height, StoredU64>>>) -> Self {
        Self(value)
    }
}

impl AddressTypeToHeightToAddressCount {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self::from(ByAddressType::new_with_name(|type_name| {
            Ok(EagerVec::forced_import(
                db,
                &format!("{type_name}_{name}"),
                version,
            )?)
        })?))
    }

    pub fn safe_write(&mut self, exit: &Exit) -> Result<()> {
        use vecdb::AnyStoredVec;
        self.p2pk65.safe_write(exit)?;
        self.p2pk33.safe_write(exit)?;
        self.p2pkh.safe_write(exit)?;
        self.p2sh.safe_write(exit)?;
        self.p2wpkh.safe_write(exit)?;
        self.p2wsh.safe_write(exit)?;
        self.p2tr.safe_write(exit)?;
        self.p2a.safe_write(exit)?;
        Ok(())
    }

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

    pub fn reset(&mut self) -> Result<()> {
        use vecdb::GenericStoredVec;
        self.p2pk65.reset()?;
        self.p2pk33.reset()?;
        self.p2pkh.reset()?;
        self.p2sh.reset()?;
        self.p2wpkh.reset()?;
        self.p2wsh.reset()?;
        self.p2tr.reset()?;
        self.p2a.reset()?;
        Ok(())
    }
}

/// Address count per address type, indexed by various indexes (dateindex, etc.).
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct AddressTypeToIndexesToAddressCount(ByAddressType<ComputedVecsFromHeight<StoredU64>>);

impl From<ByAddressType<ComputedVecsFromHeight<StoredU64>>> for AddressTypeToIndexesToAddressCount {
    #[inline]
    fn from(value: ByAddressType<ComputedVecsFromHeight<StoredU64>>) -> Self {
        Self(value)
    }
}

impl AddressTypeToIndexesToAddressCount {
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self::from(ByAddressType::new_with_name(|type_name| {
            ComputedVecsFromHeight::forced_import(
                db,
                &format!("{type_name}_{name}"),
                Source::None,
                version,
                indexes,
                VecBuilderOptions::default().add_last(),
            )
        })?))
    }

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
