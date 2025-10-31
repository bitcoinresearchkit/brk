use brk_grouper::ByAddressType;
use brk_types::Height;
use derive_deref::{Deref, DerefMut};
use vecdb::VecIterator;

use super::AddressTypeToHeightToAddressCount;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToAddressCount(ByAddressType<u64>);

impl From<(&AddressTypeToHeightToAddressCount, Height)> for AddressTypeToAddressCount {
    #[inline]
    fn from((groups, starting_height): (&AddressTypeToHeightToAddressCount, Height)) -> Self {
        if let Some(prev_height) = starting_height.decremented() {
            Self(ByAddressType {
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
            })
        } else {
            Default::default()
        }
    }
}
