use std::{
    mem,
    ops::{Add, AddAssign},
};

use brk_core::{Height, OutputType, Result, StoredUsize};
use brk_exit::Exit;
use brk_vec::{EagerVec, VecIterator};

use super::GroupFilter;

#[derive(Default, Clone, Debug)]
pub struct GroupedByAddressType<T> {
    pub p2pk65: T,
    pub p2pk33: T,
    pub p2pkh: T,
    pub p2sh: T,
    pub p2wpkh: T,
    pub p2wsh: T,
    pub p2tr: T,
    pub p2a: T,
}

impl<T> GroupedByAddressType<T> {
    pub fn get(&self, address_type: OutputType) -> Option<&T> {
        match address_type {
            OutputType::P2PK65 => Some(&self.p2pk65),
            OutputType::P2PK33 => Some(&self.p2pk33),
            OutputType::P2PKH => Some(&self.p2pkh),
            OutputType::P2SH => Some(&self.p2sh),
            OutputType::P2WPKH => Some(&self.p2wpkh),
            OutputType::P2WSH => Some(&self.p2wsh),
            OutputType::P2TR => Some(&self.p2tr),
            OutputType::P2A => Some(&self.p2a),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, address_type: OutputType) -> Option<&mut T> {
        match address_type {
            OutputType::P2PK65 => Some(&mut self.p2pk65),
            OutputType::P2PK33 => Some(&mut self.p2pk33),
            OutputType::P2PKH => Some(&mut self.p2pkh),
            OutputType::P2SH => Some(&mut self.p2sh),
            OutputType::P2WPKH => Some(&mut self.p2wpkh),
            OutputType::P2WSH => Some(&mut self.p2wsh),
            OutputType::P2TR => Some(&mut self.p2tr),
            OutputType::P2A => Some(&mut self.p2a),
            _ => None,
        }
    }

    pub fn as_mut_vec(&mut self) -> [&mut T; 8] {
        [
            &mut self.p2pk65,
            &mut self.p2pk33,
            &mut self.p2pkh,
            &mut self.p2sh,
            &mut self.p2wpkh,
            &mut self.p2wsh,
            &mut self.p2tr,
            &mut self.p2a,
        ]
    }

    pub fn as_typed_vec(&self) -> [(OutputType, &T); 8] {
        [
            (OutputType::P2PK65, &self.p2pk65),
            (OutputType::P2PK33, &self.p2pk33),
            (OutputType::P2PKH, &self.p2pkh),
            (OutputType::P2SH, &self.p2sh),
            (OutputType::P2WPKH, &self.p2wpkh),
            (OutputType::P2WSH, &self.p2wsh),
            (OutputType::P2TR, &self.p2tr),
            (OutputType::P2A, &self.p2a),
        ]
    }

    pub fn as_mut_typed_vec(&mut self) -> [(OutputType, &mut T); 8] {
        [
            (OutputType::P2PK65, &mut self.p2pk65),
            (OutputType::P2PK33, &mut self.p2pk33),
            (OutputType::P2PKH, &mut self.p2pkh),
            (OutputType::P2SH, &mut self.p2sh),
            (OutputType::P2WPKH, &mut self.p2wpkh),
            (OutputType::P2WSH, &mut self.p2wsh),
            (OutputType::P2TR, &mut self.p2tr),
            (OutputType::P2A, &mut self.p2a),
        ]
    }

    pub fn into_typed_vec(&mut self) -> [(OutputType, T); 8]
    where
        T: Default,
    {
        [
            (OutputType::P2PK65, mem::take(&mut self.p2pk65)),
            (OutputType::P2PK33, mem::take(&mut self.p2pk33)),
            (OutputType::P2PKH, mem::take(&mut self.p2pkh)),
            (OutputType::P2SH, mem::take(&mut self.p2sh)),
            (OutputType::P2WPKH, mem::take(&mut self.p2wpkh)),
            (OutputType::P2WSH, mem::take(&mut self.p2wsh)),
            (OutputType::P2TR, mem::take(&mut self.p2tr)),
            (OutputType::P2A, mem::take(&mut self.p2a)),
        ]
    }
}

impl<T> GroupedByAddressType<(GroupFilter, T)> {
    pub fn vecs(&self) -> [&T; 8] {
        [
            &self.p2pk65.1,
            &self.p2pk33.1,
            &self.p2pkh.1,
            &self.p2sh.1,
            &self.p2wpkh.1,
            &self.p2wsh.1,
            &self.p2tr.1,
            &self.p2a.1,
        ]
    }
}

impl<T> From<GroupedByAddressType<T>> for GroupedByAddressType<(GroupFilter, T)> {
    fn from(value: GroupedByAddressType<T>) -> Self {
        Self {
            p2pk65: (GroupFilter::Type(OutputType::P2PK65), value.p2pk65),
            p2pk33: (GroupFilter::Type(OutputType::P2PK33), value.p2pk33),
            p2pkh: (GroupFilter::Type(OutputType::P2PKH), value.p2pkh),
            p2sh: (GroupFilter::Type(OutputType::P2SH), value.p2sh),
            p2wpkh: (GroupFilter::Type(OutputType::P2WPKH), value.p2wpkh),
            p2wsh: (GroupFilter::Type(OutputType::P2WSH), value.p2wsh),
            p2tr: (GroupFilter::Type(OutputType::P2TR), value.p2tr),
            p2a: (GroupFilter::Type(OutputType::P2A), value.p2a),
        }
    }
}

impl<T> Add for GroupedByAddressType<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            p2pk65: self.p2pk65 + rhs.p2pk65,
            p2pk33: self.p2pk33 + rhs.p2pk33,
            p2pkh: self.p2pkh + rhs.p2pkh,
            p2sh: self.p2sh + rhs.p2sh,
            p2wpkh: self.p2wpkh + rhs.p2wpkh,
            p2wsh: self.p2wsh + rhs.p2wsh,
            p2tr: self.p2tr + rhs.p2tr,
            p2a: self.p2a + rhs.p2a,
        }
    }
}

impl<T> AddAssign for GroupedByAddressType<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.p2pk65 += rhs.p2pk65;
        self.p2pk33 += rhs.p2pk33;
        self.p2pkh += rhs.p2pkh;
        self.p2sh += rhs.p2sh;
        self.p2wpkh += rhs.p2wpkh;
        self.p2wsh += rhs.p2wsh;
        self.p2tr += rhs.p2tr;
        self.p2a += rhs.p2a;
    }
}

impl From<(&GroupedByAddressType<EagerVec<Height, StoredUsize>>, Height)>
    for GroupedByAddressType<usize>
{
    fn from(
        (groups, starting_height): (&GroupedByAddressType<EagerVec<Height, StoredUsize>>, Height),
    ) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            Self {
                p2pk65: groups
                    .p2pk65
                    .into_iter()
                    .unwrap_get_inner(prev_height)
                    .into(),
                p2pk33: groups
                    .p2pk33
                    .into_iter()
                    .unwrap_get_inner(prev_height)
                    .into(),
                p2pkh: groups
                    .p2pkh
                    .into_iter()
                    .unwrap_get_inner(prev_height)
                    .into(),
                p2sh: groups.p2sh.into_iter().unwrap_get_inner(prev_height).into(),
                p2wpkh: groups
                    .p2wpkh
                    .into_iter()
                    .unwrap_get_inner(prev_height)
                    .into(),
                p2wsh: groups
                    .p2wsh
                    .into_iter()
                    .unwrap_get_inner(prev_height)
                    .into(),
                p2tr: groups.p2tr.into_iter().unwrap_get_inner(prev_height).into(),
                p2a: groups.p2a.into_iter().unwrap_get_inner(prev_height).into(),
            }
        } else {
            Default::default()
        }
    }
}

impl GroupedByAddressType<EagerVec<Height, StoredUsize>> {
    pub fn forced_push_at(
        &mut self,
        height: Height,
        addresstype_to_usize: &GroupedByAddressType<usize>,
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
