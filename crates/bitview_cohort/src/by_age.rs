#[cfg(feature = "storage")]
use bitview_traversable::Traversable;
use rayon::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AgeId, AgeRange, CohortId, OverAge, UnderAge};

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "storage", derive(Traversable))]
pub struct ByAge<T> {
    pub range: AgeRange<T>,
    pub under: UnderAge<T>,
    pub over: OverAge<T>,
}

impl<T> ByAge<T> {
    pub fn from_fn(mut create: impl FnMut(AgeId) -> T) -> Self {
        Self {
            range: AgeRange::from_fn(|id| create(AgeId::Range(id))),
            under: UnderAge::from_fn(|id| create(AgeId::Under(id))),
            over: OverAge::from_fn(|id| create(AgeId::Over(id))),
        }
    }

    pub fn try_from_fn<E>(mut create: impl FnMut(AgeId) -> Result<T, E>) -> Result<Self, E> {
        Ok(Self {
            range: AgeRange::try_from_fn(|id| create(AgeId::Range(id)))?,
            under: UnderAge::try_from_fn(|id| create(AgeId::Under(id)))?,
            over: OverAge::try_from_fn(|id| create(AgeId::Over(id)))?,
        })
    }

    pub fn new(mut create: impl FnMut(CohortId) -> T) -> Self {
        Self::from_fn(|id| create(CohortId::Age(id)))
    }

    pub fn try_new<E>(mut create: impl FnMut(CohortId) -> Result<T, E>) -> Result<Self, E> {
        Self::try_from_fn(|id| create(CohortId::Age(id)))
    }

    pub fn get(&self, id: AgeId) -> &T {
        match id {
            AgeId::Range(id) => id.select(&self.range),
            AgeId::Under(id) => id.select(&self.under),
            AgeId::Over(id) => id.select(&self.over),
        }
    }

    pub fn map_with_id<U>(&self, mut map: impl FnMut(CohortId, &T) -> U) -> ByAge<U> {
        ByAge::from_fn(|id| map(CohortId::Age(id), self.get(id)))
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
