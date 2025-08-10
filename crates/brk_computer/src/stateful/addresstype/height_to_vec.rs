use std::collections::BTreeMap;

use brk_structs::Height;
use derive_deref::{Deref, DerefMut};

use crate::stateful::AddressTypeToVec;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct HeightToAddressTypeToVec<T>(pub BTreeMap<Height, AddressTypeToVec<T>>);
