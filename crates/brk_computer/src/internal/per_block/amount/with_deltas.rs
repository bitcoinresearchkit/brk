use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Sats, SatsSigned, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{AmountPerBlock, LazyRollingDeltasFromHeight, WindowStartVec, Windows},
};

#[derive(Deref, DerefMut, Traversable)]
pub struct AmountPerBlockWithDeltas<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: AmountPerBlock<M>,
    pub delta: LazyRollingDeltasFromHeight<Sats, SatsSigned, BasisPointsSigned32>,
}

impl AmountPerBlockWithDeltas {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let inner = AmountPerBlock::forced_import(db, name, version, indexes)?;

        let delta = LazyRollingDeltasFromHeight::new(
            &format!("{name}_delta"),
            version + Version::ONE,
            &inner.sats.height,
            cached_starts,
            indexes,
        );

        Ok(Self { inner, delta })
    }
}
