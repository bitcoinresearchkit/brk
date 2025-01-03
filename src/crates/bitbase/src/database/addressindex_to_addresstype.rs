use derive_deref::{Deref, DerefMut};

use crate::structs::{Addressindex, Addresstype, Database, DatabaseTrait, Height, Version};

#[derive(Deref, DerefMut)]
pub struct AddressindexToAddresstype(Database);

impl AddressindexToAddresstype {
    pub fn import() -> color_eyre::Result<Self> {
        Ok(Self(Database::import(
            "addressindex_to_addresstype",
            Self::version(),
        )?))
    }

    // pub fn get(&self, addressindex: Addressindex) -> color_eyre::Result<Option<Addresstype>> {
    //     if let Some(addresstype) = self.0.get(addressindex.into())?.map(Addresstype::try_from) {
    //         Ok(Some(addresstype?))
    //     } else {
    //         Ok(None)
    //     }
    // }

    pub fn insert(&mut self, addressindex: Addressindex, addresstype: Addresstype, height: Height) {
        self.0
            .insert(addressindex.into(), addresstype.into(), height)
    }

    pub fn remove(&mut self, addressindex: Addressindex) {
        self.0.remove(addressindex.into())
    }
}

impl DatabaseTrait for AddressindexToAddresstype {
    fn version() -> Version {
        Version::from(1)
    }
}
