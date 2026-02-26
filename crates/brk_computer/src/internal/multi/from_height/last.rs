//! ComputedFromHeight using only Last aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    BinaryTransform, Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableCloneableVec,
    ReadableVec, Rw, StorageMode, VecValue,
};

use crate::indexes;

use crate::internal::{ComputedHeightDerivedLast, ComputedVecValue, NumericValue};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightLast<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<ComputedHeightDerivedLast<T>>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightLast<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;

        let rest = ComputedHeightDerivedLast::forced_import(
            name,
            height.read_only_boxed_clone(),
            v,
            indexes,
        );

        Ok(Self {
            height,
            rest: Box::new(rest),
        })
    }

    /// Eagerly compute this vec as a binary transform of two sources.
    pub(crate) fn compute_binary<S1T, S2T, F>(
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
        self.height.compute_transform2(
            max_from,
            source1,
            source2,
            |(h, s1, s2, ..)| (h, F::apply(s1, s2)),
            exit,
        )?;
        Ok(())
    }
}
