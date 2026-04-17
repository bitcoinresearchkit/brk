//! Generic `all` + per-`AddrType` container, mirrors the `WithSth` pattern
//! along the address-type axis. Used by every metric that tracks one
//! aggregate value alongside a per-address-type breakdown.

use brk_cohort::ByAddrType;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, Version};
use rayon::prelude::*;
use schemars::JsonSchema;
use vecdb::{AnyStoredVec, AnyVec, Database, EagerVec, Exit, PcoVec, WritableVec};

use crate::{indexes, prices};

use super::{
    AmountPerBlock, BpsType, NumericValue, PerBlock, PerBlockCumulativeRolling, PercentPerBlock,
    WindowStartVec, Windows,
};

use crate::distribution::metrics::AvgAmountMetrics;

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

impl WithAddrTypes<AmountPerBlock> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = AmountPerBlock::forced_import(db, name, version, indexes)?;
        let by_addr_type = ByAddrType::new_with_name(|type_name| {
            AmountPerBlock::forced_import(db, &format!("{type_name}_{name}"), version, indexes)
        })?;
        Ok(Self { all, by_addr_type })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.by_addr_type
            .values()
            .map(|v| v.sats.height.len())
            .min()
            .unwrap()
            .min(self.all.sats.height.len())
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        rayon::iter::once(&mut self.all.sats.height as &mut dyn AnyStoredVec).chain(
            self.by_addr_type
                .par_values_mut()
                .map(|v| &mut v.sats.height as &mut dyn AnyStoredVec),
        )
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.sats.height.reset()?;
        self.all.cents.height.reset()?;
        for v in self.by_addr_type.values_mut() {
            v.sats.height.reset()?;
            v.cents.height.reset()?;
        }
        Ok(())
    }

    /// Push the stateful sats value for `all` and each per-type. Cents are
    /// derived post-hoc from sats × price in [`Self::compute_rest`].
    #[inline(always)]
    pub(crate) fn push_height<U>(&mut self, total: U, per_type: impl IntoIterator<Item = U>)
    where
        U: Into<Sats>,
    {
        self.all.sats.height.push(total.into());
        for (v, value) in self.by_addr_type.values_mut().zip(per_type) {
            v.sats.height.push(value.into());
        }
    }

    /// Derive cents (and thus lazy btc/usd) for `all` and every per-type vec
    /// from the stateful sats values × spot price.
    pub(crate) fn compute_rest(
        &mut self,
        max_from: Height,
        prices: &prices::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        self.all.compute(prices, max_from, exit)?;
        for v in self.by_addr_type.values_mut() {
            v.compute(prices, max_from, exit)?;
        }
        Ok(())
    }
}

impl WithAddrTypes<AvgAmountMetrics> {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = AvgAmountMetrics::forced_import(db, "", version, indexes)?;
        let by_addr_type = ByAddrType::new_with_name(|type_name| {
            AvgAmountMetrics::forced_import(db, type_name, version, indexes)
        })?;
        Ok(Self { all, by_addr_type })
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.all.collect_vecs_mut();
        for v in self.by_addr_type.values_mut() {
            vecs.extend(v.collect_vecs_mut());
        }
        vecs
    }

    pub(crate) fn par_iter_height_mut(
        &mut self,
    ) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.collect_vecs_mut().into_par_iter()
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.reset_height()?;
        for v in self.by_addr_type.values_mut() {
            v.reset_height()?;
        }
        Ok(())
    }
}

impl<B: BpsType> WithAddrTypes<PercentPerBlock<B>> {
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let all = PercentPerBlock::forced_import(db, name, version, indexes)?;
        let by_addr_type = ByAddrType::new_with_name(|type_name| {
            PercentPerBlock::forced_import(db, &format!("{type_name}_{name}"), version, indexes)
        })?;
        Ok(Self { all, by_addr_type })
    }

    pub(crate) fn reset_height(&mut self) -> Result<()> {
        self.all.bps.height.reset()?;
        for v in self.by_addr_type.values_mut() {
            v.bps.height.reset()?;
        }
        Ok(())
    }
}
