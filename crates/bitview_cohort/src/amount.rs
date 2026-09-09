#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use rayon::prelude::*;
use schemars::JsonSchema;
use serde::Serialize;

use crate::{AmountId, AmountRange, CohortId, OverAmount, UnderAmount};

#[derive(Debug, Default, Clone, Serialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct Amount<T> {
    pub range: AmountRange<T>,
    pub under: UnderAmount<T>,
    pub over: OverAmount<T>,
}

impl<T> Amount<T> {
    pub fn from_fn(mut create: impl FnMut(AmountId) -> T) -> Self {
        Self {
            range: AmountRange::from_fn(|id| create(AmountId::Range(id))),
            under: UnderAmount::from_fn(|id| create(AmountId::Under(id))),
            over: OverAmount::from_fn(|id| create(AmountId::Over(id))),
        }
    }

    pub fn try_from_fn<E>(mut create: impl FnMut(AmountId) -> Result<T, E>) -> Result<Self, E> {
        Ok(Self {
            range: AmountRange::try_from_fn(|id| create(AmountId::Range(id)))?,
            under: UnderAmount::try_from_fn(|id| create(AmountId::Under(id)))?,
            over: OverAmount::try_from_fn(|id| create(AmountId::Over(id)))?,
        })
    }

    pub fn new(mut create: impl FnMut(CohortId) -> T) -> Self {
        Self::from_fn(|id| create(CohortId::Amount(id)))
    }

    pub fn try_new<E>(mut create: impl FnMut(CohortId) -> Result<T, E>) -> Result<Self, E> {
        Self::try_from_fn(|id| create(CohortId::Amount(id)))
    }

    pub fn get(&self, id: AmountId) -> &T {
        match id {
            AmountId::Range(id) => id.select(&self.range),
            AmountId::Under(id) => id.select(&self.under),
            AmountId::Over(id) => id.select(&self.over),
        }
    }

    pub fn map_with_id<U>(&self, mut map: impl FnMut(CohortId, &T) -> U) -> Amount<U> {
        Amount::from_fn(|id| map(CohortId::Amount(id), self.get(id)))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.range
            .iter()
            .chain(self.under.iter())
            .chain(self.over.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.range
            .iter_mut()
            .chain(self.under.iter_mut())
            .chain(self.over.iter_mut())
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut T>
    where
        T: Send + Sync,
    {
        self.range
            .par_iter_mut()
            .chain(self.under.par_iter_mut())
            .chain(self.over.par_iter_mut())
    }
}
