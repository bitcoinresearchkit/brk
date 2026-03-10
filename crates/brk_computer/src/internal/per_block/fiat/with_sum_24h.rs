//! FiatPerBlockWithSum24h - FiatPerBlock raw + RollingWindow24hFiatPerBlock sum.

use std::ops::SubAssign;

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode};

use crate::{
    indexes,
    internal::{CentsType, FiatPerBlock, RollingWindow24h},
};

/// Single 24h rolling window backed by FiatPerBlock (cents + lazy usd).
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct RollingWindow24hFiatPerBlock<C: CentsType, M: StorageMode = Rw>(
    pub RollingWindow24h<FiatPerBlock<C, M>>,
);

impl<C: CentsType> RollingWindow24hFiatPerBlock<C> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self(RollingWindow24h {
            _24h: FiatPerBlock::forced_import(db, &format!("{name}_24h"), version, indexes)?,
        }))
    }

    pub(crate) fn compute_rolling_sum(
        &mut self,
        max_from: Height,
        height_24h_ago: &impl ReadableVec<Height, Height>,
        source: &impl ReadableVec<Height, C>,
        exit: &Exit,
    ) -> Result<()>
    where
        C: Default + SubAssign,
    {
        self._24h
            .cents
            .height
            .compute_rolling_sum(max_from, height_24h_ago, source, exit)?;
        Ok(())
    }
}

/// Fiat per-block value (cents + usd) with 24h rolling sum (also fiat).
#[derive(Traversable)]
pub struct FiatPerBlockWithSum24h<C: CentsType, M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub raw: FiatPerBlock<C, M>,
    pub sum: RollingWindow24hFiatPerBlock<C, M>,
}
