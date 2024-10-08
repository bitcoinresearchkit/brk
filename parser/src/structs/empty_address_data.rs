use allocative::Allocative;
use sanakirja::{direct_repr, Storable, UnsizedStorable};

use super::{AddressData, AddressType, Amount};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Allocative)]
pub struct EmptyAddressData(u64);
direct_repr!(EmptyAddressData);

const SHIFT: u64 = 5;
const AND: u64 = (1 << SHIFT) - 1;
const MAX: u64 = (u64::MAX - 1) >> 5;

impl EmptyAddressData {
    pub fn from_non_empty(non_empty: &AddressData) -> Self {
        if non_empty.sent != non_empty.received {
            dbg!(&non_empty);
            panic!("Trying to convert not empty wallet to empty !");
        }

        let transfered = non_empty.sent.to_sat();

        if transfered >= MAX {
            panic!("Too large !");
        }

        Self((transfered << SHIFT) + (non_empty.address_type as u64))
    }

    pub fn address_type(&self) -> AddressType {
        (self.0 & AND).try_into().unwrap()
    }

    pub fn transfered(&self) -> Amount {
        Amount::from_sat(self.0 >> SHIFT)
    }
}
