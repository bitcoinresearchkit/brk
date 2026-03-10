use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{distribution::state::{CohortState, RealizedOps}, prices};

use crate::internal::{
    AmountPerBlock, HalveCents, HalveDollars, HalveSats, HalveSatsToBitcoin,
    LazyAmountPerBlock,
};

use crate::distribution::metrics::ImportConfig;

/// Base supply metrics: total supply only (2 stored vecs).
#[derive(Traversable)]
pub struct SupplyBase<M: StorageMode = Rw> {
    pub total: AmountPerBlock<M>,
    pub halved: LazyAmountPerBlock,
}

impl SupplyBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = cfg.import("supply", Version::ZERO)?;

        let supply_halved = LazyAmountPerBlock::from_block_source::<
            HalveSats,
            HalveSatsToBitcoin,
            HalveCents,
            HalveDollars,
        >(&cfg.name("supply_halved"), &supply, cfg.version);

        Ok(Self {
            total: supply,
            halved: supply_halved,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.total.sats.height.len()
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState<impl RealizedOps>) -> Result<()> {
        self.total.sats.height.truncate_push(height, state.supply.value)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.total.sats.height as &mut dyn AnyStoredVec,
            &mut self.total.cents.height as &mut dyn AnyStoredVec,
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
