use std::collections::BTreeMap;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;

use crate::server::Grouped;

#[derive(Clone, Default, Deref, DerefMut, Debug, Serialize)]
pub struct Paths(pub Grouped<BTreeMap<String, String>>);
