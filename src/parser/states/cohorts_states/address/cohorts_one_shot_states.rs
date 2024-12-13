use derive_deref::{Deref, DerefMut};

use crate::parser::states::OneShotStates;

use super::SplitByAddressCohort;

#[derive(Deref, DerefMut, Default)]
pub struct AddressCohortsOneShotStates(pub SplitByAddressCohort<OneShotStates>);
