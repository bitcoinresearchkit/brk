use derive_deref::{Deref, DerefMut};

use crate::parser::states::OneShotStates;

use super::SplitByUTXOCohort;

#[derive(Deref, DerefMut, Default)]
pub struct UTXOCohortsOneShotStates(pub SplitByUTXOCohort<OneShotStates>);
