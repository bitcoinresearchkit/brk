use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{CentsSats, CentsSquaredSats, Height, Indexes, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, BytesVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::distribution::{metrics::ImportConfig, state::UnrealizedState};

use super::UnrealizedCore;

#[derive(Deref, DerefMut, Traversable)]
pub struct UnrealizedBase<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: UnrealizedCore<M>,

    #[traversable(hidden)]
    pub invested_capital_in_profit_raw: M::Stored<BytesVec<Height, CentsSats>>,
    #[traversable(hidden)]
    pub invested_capital_in_loss_raw: M::Stored<BytesVec<Height, CentsSats>>,
    #[traversable(hidden)]
    pub investor_cap_in_profit_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
    #[traversable(hidden)]
    pub investor_cap_in_loss_raw: M::Stored<BytesVec<Height, CentsSquaredSats>>,
}

impl UnrealizedBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;

        let core = UnrealizedCore::forced_import(cfg)?;

        let invested_capital_in_profit_raw =
            cfg.import("invested_capital_in_profit_raw", v0)?;
        let invested_capital_in_loss_raw = cfg.import("invested_capital_in_loss_raw", v0)?;
        let investor_cap_in_profit_raw = cfg.import("investor_cap_in_profit_raw", v0)?;
        let investor_cap_in_loss_raw = cfg.import("investor_cap_in_loss_raw", v0)?;

        Ok(Self {
            core,
            invested_capital_in_profit_raw,
            invested_capital_in_loss_raw,
            investor_cap_in_profit_raw,
            investor_cap_in_loss_raw,
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.core
            .min_stateful_len()
            .min(self.invested_capital_in_profit_raw.len())
            .min(self.invested_capital_in_loss_raw.len())
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &UnrealizedState) {
        self.core.push_state(state);

        self.invested_capital_in_profit_raw
            .push(CentsSats::new(state.invested_capital_in_profit_raw));
        self.invested_capital_in_loss_raw
            .push(CentsSats::new(state.invested_capital_in_loss_raw));
        self.investor_cap_in_profit_raw
            .push(CentsSquaredSats::new(state.investor_cap_in_profit_raw));
        self.investor_cap_in_loss_raw
            .push(CentsSquaredSats::new(state.investor_cap_in_loss_raw));
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.invested_capital_in_profit_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.invested_capital_in_loss_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_in_profit_raw as &mut dyn AnyStoredVec);
        vecs.push(&mut self.investor_cap_in_loss_raw as &mut dyn AnyStoredVec);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let core_refs: Vec<&UnrealizedCore> =
            others.iter().map(|o| &o.core).collect();
        self.core
            .compute_from_stateful(starting_indexes, &core_refs, exit)?;

        let start = self
            .invested_capital_in_profit_raw
            .len()
            .min(self.invested_capital_in_loss_raw.len())
            .min(self.investor_cap_in_profit_raw.len())
            .min(self.investor_cap_in_loss_raw.len());
        let end = others
            .iter()
            .map(|o| o.invested_capital_in_profit_raw.len())
            .min()
            .unwrap_or(0);

        let invested_profit_ranges: Vec<Vec<CentsSats>> = others
            .iter()
            .map(|o| {
                o.invested_capital_in_profit_raw
                    .collect_range_at(start, end)
            })
            .collect();
        let invested_loss_ranges: Vec<Vec<CentsSats>> = others
            .iter()
            .map(|o| o.invested_capital_in_loss_raw.collect_range_at(start, end))
            .collect();
        let investor_profit_ranges: Vec<Vec<CentsSquaredSats>> = others
            .iter()
            .map(|o| o.investor_cap_in_profit_raw.collect_range_at(start, end))
            .collect();
        let investor_loss_ranges: Vec<Vec<CentsSquaredSats>> = others
            .iter()
            .map(|o| o.investor_cap_in_loss_raw.collect_range_at(start, end))
            .collect();

        self.invested_capital_in_profit_raw
            .truncate_if_needed_at(start)?;
        self.invested_capital_in_loss_raw
            .truncate_if_needed_at(start)?;
        self.investor_cap_in_profit_raw
            .truncate_if_needed_at(start)?;
        self.investor_cap_in_loss_raw
            .truncate_if_needed_at(start)?;

        for i in start..end {
            let local_i = i - start;

            let mut sum_invested_profit = CentsSats::ZERO;
            let mut sum_invested_loss = CentsSats::ZERO;
            let mut sum_investor_profit = CentsSquaredSats::ZERO;
            let mut sum_investor_loss = CentsSquaredSats::ZERO;

            for idx in 0..others.len() {
                sum_invested_profit += invested_profit_ranges[idx][local_i];
                sum_invested_loss += invested_loss_ranges[idx][local_i];
                sum_investor_profit += investor_profit_ranges[idx][local_i];
                sum_investor_loss += investor_loss_ranges[idx][local_i];
            }

            self.invested_capital_in_profit_raw
                .push(sum_invested_profit);
            self.invested_capital_in_loss_raw
                .push(sum_invested_loss);
            self.investor_cap_in_profit_raw
                .push(sum_investor_profit);
            self.investor_cap_in_loss_raw
                .push(sum_investor_loss);
        }

        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest(starting_indexes, exit)?;
        Ok(())
    }
}
