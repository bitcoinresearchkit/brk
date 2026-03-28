use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Indexes, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{
        metrics::ImportConfig,
        state::{CohortState, CostBasisOps, RealizedOps},
    },
    internal::{AmountPerBlockCumulativeRolling, PerBlockCumulativeRolling},
    prices,
};

use super::ActivityMinimal;

#[derive(Deref, DerefMut, Traversable)]
pub struct ActivityCore<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub minimal: ActivityMinimal<M>,

    pub coindays_destroyed: PerBlockCumulativeRolling<StoredF64, StoredF64, M>,
    #[traversable(wrap = "transfer_volume", rename = "in_profit")]
    pub transfer_volume_in_profit: AmountPerBlockCumulativeRolling<M>,
    #[traversable(wrap = "transfer_volume", rename = "in_loss")]
    pub transfer_volume_in_loss: AmountPerBlockCumulativeRolling<M>,
}

impl ActivityCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            minimal: ActivityMinimal::forced_import(cfg)?,
            coindays_destroyed: cfg.import("coindays_destroyed", v1)?,
            transfer_volume_in_profit: cfg.import("transfer_volume_in_profit", v1)?,
            transfer_volume_in_loss: cfg.import("transfer_volume_in_loss", v1)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.minimal
            .min_len()
            .min(self.coindays_destroyed.block.len())
            .min(self.transfer_volume_in_profit.block.sats.len())
            .min(self.transfer_volume_in_loss.block.sats.len())
    }

    #[inline(always)]
    pub(crate) fn push_state(&mut self, state: &CohortState<impl RealizedOps, impl CostBasisOps>) {
        self.minimal.push_state(state);
        self.coindays_destroyed
            .block
            .push(StoredF64::from(Bitcoin::from(state.satdays_destroyed)));
        self.transfer_volume_in_profit
            .block
            .sats
            .push(state.realized.sent_in_profit());
        self.transfer_volume_in_loss
            .block
            .sats
            .push(state.realized.sent_in_loss());
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.minimal.collect_vecs_mut();
        vecs.push(&mut self.coindays_destroyed.block);
        vecs.push(&mut self.transfer_volume_in_profit.inner.block.sats);
        vecs.push(&mut self.transfer_volume_in_profit.inner.block.cents);
        vecs.push(&mut self.transfer_volume_in_loss.inner.block.sats);
        vecs.push(&mut self.transfer_volume_in_loss.inner.block.cents);
        vecs
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
        let minimal_refs: Vec<&ActivityMinimal> = others.iter().map(|o| &o.minimal).collect();
        self.minimal
            .compute_from_stateful(starting_indexes, &minimal_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; coindays_destroyed.block);
        sum_others!(self, starting_indexes, others, exit; transfer_volume_in_profit.block.sats);
        sum_others!(self, starting_indexes, others, exit; transfer_volume_in_loss.block.sats);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.minimal
            .compute_rest_part1(prices, starting_indexes, exit)?;
        self.coindays_destroyed
            .compute_rest(starting_indexes.height, exit)?;
        self.transfer_volume_in_profit
            .compute_rest(starting_indexes.height, prices, exit)?;
        self.transfer_volume_in_loss
            .compute_rest(starting_indexes.height, prices, exit)?;
        Ok(())
    }
}
