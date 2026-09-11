use std::fmt::Debug;

use crate::{Result, VecValue};

use super::{Cache, CacheBudget};

/// Selects retention at the canonical source, including its read-only clones.
/// `NoCache` has no state, allocation, registration, or read-side locking.
pub trait CachePolicy: Clone + Debug + Send + Sync + 'static {
    type State<T: VecValue>: Clone + Debug + Send + Sync;

    fn create<T: VecValue>(budget: Option<&'static CacheBudget>) -> Result<Self::State<T>>;
    fn cache<T: VecValue>(state: &Self::State<T>) -> Option<&Cache<T>>;
}
