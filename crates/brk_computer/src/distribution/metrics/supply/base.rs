use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, BasisPointsSigned32, Height, Indexes, Sats, SatsSigned, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{
    distribution::state::{CohortState, CostBasisOps, RealizedOps},
    prices,
};

use crate::internal::{
    AmountPerBlock, LazyRollingDeltasFromHeight, PercentPerBlock, RatioSatsBp16,
};

use crate::distribution::metrics::ImportConfig;

/// Base supply metrics: total supply + dominance (share of circulating).
#[derive(Traversable)]
pub struct SupplyBase<M: StorageMode = Rw> {
    pub total: AmountPerBlock<M>,
    pub delta: LazyRollingDeltasFromHeight<Sats, SatsSigned, BasisPointsSigned32>,
    #[traversable(rename = "dominance")]
    pub dominance: PercentPerBlock<BasisPoints16, M>,
}

impl SupplyBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply: AmountPerBlock = cfg.import("supply", Version::ZERO)?;

        let delta = LazyRollingDeltasFromHeight::new(
            &cfg.name("supply_delta"),
            cfg.version + Version::ONE,
            &supply.sats.height,
            cfg.cached_starts,
            cfg.indexes,
        );

        let dominance = cfg.import("supply_dominance", Version::ZERO)?;

        Ok(Self {
            total: supply,
            delta,
            dominance,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.total.sats.height.len()
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &CohortState<impl RealizedOps, impl CostBasisOps>) {
        self.total.sats.height.push(state.supply.value);
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.total.sats.height as &mut dyn AnyStoredVec,
            &mut self.total.cents.height,
            &mut self.dominance.bps.height,
        ]
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.total.compute(prices, max_from, exit)
    }

    pub(crate) fn compute_dominance(
        &mut self,
        max_from: Height,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.dominance
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                &self.total.sats.height,
                all_supply_sats,
                exit,
            )
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.total.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.total.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
