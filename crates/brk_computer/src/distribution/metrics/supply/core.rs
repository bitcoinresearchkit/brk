use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{distribution::state::UnrealizedState, prices};

use crate::internal::{
    AmountPerBlock, HalveCents, HalveDollars, HalveSats, HalveSatsToBitcoin, LazyAmountPerBlock,
};

use crate::distribution::metrics::ImportConfig;

use super::SupplyBase;

/// Core supply metrics: total + halved + in_profit/in_loss (4 stored vecs).
#[derive(Deref, DerefMut, Traversable)]
pub struct SupplyCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: SupplyBase<M>,

    pub half: LazyAmountPerBlock,
    pub in_profit: AmountPerBlock<M>,
    pub in_loss: AmountPerBlock<M>,
}

impl SupplyCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let base = SupplyBase::forced_import(cfg)?;

        let half = LazyAmountPerBlock::from_block_source::<
            HalveSats,
            HalveSatsToBitcoin,
            HalveCents,
            HalveDollars,
        >(&cfg.name("supply_half"), &base.total, cfg.version);

        Ok(Self {
            base,
            half,
            in_profit: cfg.import("supply_in_profit", v0)?,
            in_loss: cfg.import("supply_in_loss", v0)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.base
            .min_len()
            .min(self.in_profit.sats.height.len())
            .min(self.in_loss.sats.height.len())
    }

    #[inline(always)]
    pub(crate) fn push_profitability(&mut self, state: &UnrealizedState) {
        self.in_profit.sats.height.push(state.supply_in_profit);
        self.in_loss.sats.height.push(state.supply_in_loss);
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.base.collect_vecs_mut();
        vecs.push(&mut self.in_profit.sats.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.in_profit.cents.height);
        vecs.push(&mut self.in_loss.sats.height);
        vecs.push(&mut self.in_loss.cents.height);
        vecs
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.base.compute(prices, max_from, exit)?;
        self.in_profit.compute(prices, max_from, exit)?;
        self.in_loss.compute(prices, max_from, exit)?;
        Ok(())
    }

    pub(crate) fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        Ok(())
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let base_refs: Vec<&SupplyBase> = others.iter().map(|o| &o.base).collect();
        self.base
            .compute_from_stateful(starting_indexes, &base_refs, exit)?;
        sum_others!(self, starting_indexes, others, exit; in_profit.sats.height);
        sum_others!(self, starting_indexes, others, exit; in_loss.sats.height);
        Ok(())
    }
}
