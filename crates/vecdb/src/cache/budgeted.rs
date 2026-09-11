use crate::{Error, Result, VecValue};

use super::{Cache, CacheBudget, CachePolicy};

/// Retain source-selected ranges within an explicitly supplied shared budget.
#[derive(Clone, Copy, Debug, Default)]
pub struct Budgeted;

impl CachePolicy for Budgeted {
    type State<T: VecValue> = Option<Cache<T>>;

    fn create<T: VecValue>(budget: Option<&'static CacheBudget>) -> Result<Self::State<T>> {
        let budget = budget.ok_or(Error::InvalidArgument(
            "budgeted vectors require a cache budget",
        ))?;
        Ok((budget.limit() > 0).then(|| Cache::new(budget)))
    }

    #[inline(always)]
    fn cache<T: VecValue>(state: &Self::State<T>) -> Option<&Cache<T>> {
        state.as_ref()
    }
}
