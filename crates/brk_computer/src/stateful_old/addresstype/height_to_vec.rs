use std::collections::BTreeMap;

use brk_types::Height;
use derive_deref::{Deref, DerefMut};

use crate::stateful::AddressTypeToVec;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct HeightToAddressTypeToVec<T>(pub BTreeMap<Height, AddressTypeToVec<T>>);
