use derive_deref::{Deref, DerefMut};

use crate::structs::{Addressbytes, Addressindex, Database, DatabaseTrait, Height, Version};

#[derive(Deref, DerefMut)]
pub struct AddressindexToAddressbytes(Database);

impl AddressindexToAddressbytes {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "addressindex_to_addressbytes",
            Self::version(),
        )?))
    }

    pub fn get(&self, addressindex: Addressindex) -> color_eyre::Result<Option<Addressbytes>> {
        if let Some(address) = self.0.get(addressindex.into())?.map(Addressbytes::try_from) {
            Ok(Some(address?))
        } else {
            Ok(None)
        }
    }

    pub fn insert(
        &mut self,
        addressindex: Addressindex,
        addressbytes: &Addressbytes,
        height: Height,
    ) {
        self.0
            .insert(addressindex.into(), addressbytes.into(), height)
    }

    pub fn remove(&mut self, addressindex: Addressindex) {
        self.0.remove(addressindex.into())
    }
}

impl DatabaseTrait for AddressindexToAddressbytes {
    fn version() -> Version {
        Version::from(1)
    }
}
