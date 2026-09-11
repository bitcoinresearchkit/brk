use crate::{Result, VecValue};

use super::{Cache, CacheBudget, CachePolicy};

/// No retained data. The default for stored vectors.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoCache;

impl CachePolicy for NoCache {
    type State<T: VecValue> = Self;

    #[inline(always)]
    fn create<T: VecValue>(_: Option<&'static CacheBudget>) -> Result<Self> {
        Ok(Self)
    }

    #[inline(always)]
    fn cache<T: VecValue>(_: &Self) -> Option<&Cache<T>> {
        None
    }
}
