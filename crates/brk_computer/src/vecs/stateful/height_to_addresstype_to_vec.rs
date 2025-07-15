use std::collections::BTreeMap;

use brk_core::Height;
use derive_deref::{Deref, DerefMut};

use crate::vecs::stateful::AddressTypeToVec;

#[derive(Debug, Default, Deref, DerefMut)]
pub struct HeightToAddressTypeToVec<T>(pub BTreeMap<Height, AddressTypeToVec<T>>);
