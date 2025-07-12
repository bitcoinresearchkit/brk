use derive_deref::{Deref, DerefMut};

use super::ByAddressType;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct AddressTypeToVec<T>(ByAddressType<Vec<T>>);
