use color_eyre::eyre::eyre;
use derive_deref::{Deref, DerefMut};

use crate::structs::{Addressbytes, Addressindex, Database, DatabaseTrait, Height, Version};

#[derive(Deref, DerefMut)]
pub struct AddressbytesPrefixToAddressindex(Database);

impl AddressbytesPrefixToAddressindex {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "address_prefix_to_addressindex",
            Self::version(),
        )?))
    }

    pub fn insert(
        &mut self,
        addressbytes: &Addressbytes,
        addressindex: Addressindex,
        height: Height,
    ) -> color_eyre::Result<()> {
        if let Some(_height) =
            self.fetch_update(addressbytes.to_prefix_slice(), addressindex.into(), height)?
        {
            dbg!(addressbytes, addressindex);
            return Err(eyre!("AddressPrefixToAddressindex: key collision"));
        }
        Ok(())
    }

    pub fn remove(&mut self, addressbytes: &Addressbytes) {
        self.0.remove(addressbytes.to_prefix_slice())
    }
}

impl DatabaseTrait for AddressbytesPrefixToAddressindex {
    fn version() -> Version {
        Version::from(1)
    }
}
