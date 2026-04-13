//! Generic `all` + per-`AddrType` container, mirrors the `WithSth` pattern
//! along the address-type axis. Used by every metric that tracks one
//! aggregate value alongside a per-address-type breakdown.

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Version};
use rayon::prelude::*;
use schemars::JsonSchema;
use vecdb::{AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, WritableVec};

use crate::{
    indexes,
    internal::{NumericValue, PerBlock, PerBlockCumulativeRolling, WindowStartVec, Windows},
};

/// `all` aggregate plus per-`AddrType` breakdown.
#[derive(Clone, Traversable)]
pub struct WithAddrTypes<T> {
    pub all: T,
    #[traversable(flatten)]
    pub by_addr_type: ByAddrType<T>,
}

impl<T> WithAddrTypes<PerBlock<T>>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = PerBlock::forced_import(db, name, version, indexes)?;
        let by_addr_type = ByAddrType::new_with_name(|type_name| {
            PerBlock::forced_import(db, &format!("{type_name}_{name}"), version, indexes)
        })?;
        Ok(Self { all, by_addr_type })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.by_addr_type
            .values()
            .map(|v| v.height.len())
            .min()
            .unwrap()
            .min(self.all.height.len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.all.height as &mut dyn AnyStoredVec).chain(
            self.by_addr_type
                .par_values_mut()
                .map(|v| &mut v.height as &mut dyn AnyStoredVec),
        )
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.height.reset()?;
        for v in self.by_addr_type.values_mut() {
            v.height.reset()?;
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height<U>(&mut self, total: U, per_type: impl IntoIterator<Item = U>)
    where
        U: Into<T>,
    {
        self.all.height.push(total.into());
        for (v, value) in self.by_addr_type.values_mut().zip(per_type) {
            v.height.push(value.into());
        }
    }

    /// Compute `all.height` as the per-block sum of the per-type vecs.
    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let sources: Vec<&EagerVec<PcoVec<Height, T>>> =
            self.by_addr_type.values().map(|v| &v.height).collect();
        self.all
            .height
            .compute_sum_of_others(starting_indexes.height, &sources, exit)?;
        Ok(())
    }
}

impl<T, C> WithAddrTypes<PerBlockCumulativeRolling<T, C>>
where
    T: NumericValue + JsonSchema + Into<C>,
    C: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let all = PerBlockCumulativeRolling::forced_import(
            db,
            name,
            version,
            indexes,
            cached_starts,
        )?;
        let by_addr_type = ByAddrType::new_with_name(|type_name| {
            PerBlockCumulativeRolling::forced_import(
                db,
                &format!("{type_name}_{name}"),
                version,
                indexes,
                cached_starts,
            )
        })?;
        Ok(Self { all, by_addr_type })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.by_addr_type
            .values()
            .map(|v| v.block.len())
            .min()
            .unwrap()
            .min(self.all.block.len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.all.block as &mut dyn AnyStoredVec).chain(
            self.by_addr_type
                .par_values_mut()
                .map(|v| &mut v.block as &mut dyn AnyStoredVec),
        )
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.block.reset()?;
        for v in self.by_addr_type.values_mut() {
            v.block.reset()?;
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) fn push_height<U>(&mut self, total: U, per_type: impl IntoIterator<Item = U>)
    where
        U: Into<T>,
    {
        self.all.block.push(total.into());
        for (v, value) in self.by_addr_type.values_mut().zip(per_type) {
            v.block.push(value.into());
        }
    }

    /// Finalize `cumulative` / `sum` / `average` for `all` and every per-type vec.
    pub(crate) fn compute_rest(&mut self, max_from: Height, exit: &Exit) -> Result<()> {
        self.all.compute_rest(max_from, exit)?;
        for v in self.by_addr_type.values_mut() {
            v.compute_rest(max_from, exit)?;
        }
        Ok(())
    }
}
