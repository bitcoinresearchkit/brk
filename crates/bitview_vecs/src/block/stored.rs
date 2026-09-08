use bitview_traversable::Traversable;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    BinaryTransform, Budgeted, CacheBudget, CachedVec, CachedVecStrategy, Database, EagerVec,
    ImportableVec, PcoVec, PcoVecValue, ReadableVec, Rw, StorageMode, VecValue,
};

use crate::{IndexSources, Resolutions};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct PerBlock<T, M: StorageMode = Rw, S: CachedVecStrategy = Budgeted>
where
    T: PcoVecValue + PartialOrd + JsonSchema,
{
    pub height: CachedVec<M::Stored<EagerVec<PcoVec<Height, T>>>, S>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub resolutions: Box<Resolutions<T>>,
}

impl<T, S: CachedVecStrategy> PerBlock<T, Rw, S>
where
    T: PcoVecValue + PartialOrd + JsonSchema + 'static,
{
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        name: &str,
        version: Version,
        indexes: &IndexSources,
    ) -> Result<Self> {
        let height = S::wrap(
            EagerVec::<PcoVec<Height, T>>::forced_import(db, name, version)?,
            cache,
        );

        let resolutions = Resolutions::from_source(name, &height, version, indexes);

        Ok(Self {
            height,
            resolutions: Box::new(resolutions),
        })
    }

    /// Eagerly compute this vec as a binary transform of two sources.
    pub fn compute_binary<S1T, S2T, F>(
        &mut self,
        max_from: Height,
        source1: &impl ReadableVec<Height, S1T>,
        source2: &impl ReadableVec<Height, S2T>,
        exit: &Exit,
    ) -> Result<()>
    where
        S1T: VecValue,
        S2T: VecValue,
        F: BinaryTransform<S1T, S2T, T>,
    {
        self.height
            .compute_binary::<S1T, S2T, F>(max_from, source1, source2, exit)?;
        Ok(())
    }
}
