use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Sats, SatsSigned, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Rw, StorageMode};

use crate::{
    indexes,
    internal::{LazyRollingDeltasAmountFromHeight, ValuePerBlock, WindowStartVec, Windows},
};

#[derive(Deref, DerefMut, Traversable)]
pub struct ValuePerBlockWithDeltas<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ValuePerBlock<M>,
    pub delta: LazyRollingDeltasAmountFromHeight<Sats, SatsSigned, BasisPointsSigned32>,
}

impl ValuePerBlockWithDeltas {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let inner = ValuePerBlock::forced_import(db, name, version, indexes)?;

        let delta = LazyRollingDeltasAmountFromHeight::new(
            &format!("{name}_delta"),
            version + Version::ONE,
            &inner.sats.height,
            cached_starts,
            indexes,
        );

        Ok(Self { inner, delta })
    }
}
