use brk_core::{AddressData, EmptyAddressData};

#[derive(Debug)]
pub enum AnyAddressData {
    AddressData(AddressData),
    EmptyAddressData(EmptyAddressData),
}

impl From<AddressData> for AnyAddressData {
    fn from(value: AddressData) -> Self {
        Self::AddressData(value)
    }
}

impl From<EmptyAddressData> for AnyAddressData {
    fn from(value: EmptyAddressData) -> Self {
        Self::EmptyAddressData(value)
    }
}
