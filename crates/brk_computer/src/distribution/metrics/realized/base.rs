use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    blocks,
    distribution::state::RealizedOps,
    internal::{ComputedPerBlock, RollingWindow24h},
};

use crate::distribution::metrics::ImportConfig;

use super::RealizedCore;

#[derive(Deref, DerefMut, Traversable)]
pub struct RealizedBase<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: RealizedCore<M>,

    pub sent_in_profit: ComputedPerBlock<Sats, M>,
    pub sent_in_loss: ComputedPerBlock<Sats, M>,

    pub sent_in_profit_sum: RollingWindow24h<Sats, M>,
    pub sent_in_loss_sum: RollingWindow24h<Sats, M>,
}

impl RealizedBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;

        Ok(Self {
            core: RealizedCore::forced_import(cfg)?,
            sent_in_profit: cfg.import("sent_in_profit", v1)?,
            sent_in_loss: cfg.import("sent_in_loss", v1)?,
            sent_in_profit_sum: cfg.import("sent_in_profit", v1)?,
            sent_in_loss_sum: cfg.import("sent_in_loss", v1)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.core
            .min_stateful_height_len()
            .min(self.sent_in_profit.height.len())
            .min(self.sent_in_loss.height.len())
    }

    pub(crate) fn truncate_push(&mut self, height: Height, state: &impl RealizedOps) -> Result<()> {
        self.core.truncate_push(height, state)?;
        self.sent_in_profit
            .height
            .truncate_push(height, state.sent_in_profit())?;
        self.sent_in_loss
            .height
            .truncate_push(height, state.sent_in_loss())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.core.collect_vecs_mut();
        vecs.push(&mut self.sent_in_profit.height as &mut dyn AnyStoredVec);
        vecs.push(&mut self.sent_in_loss.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let core_refs: Vec<&RealizedCore> = others.iter().map(|o| &o.core).collect();
        self.core
            .compute_from_stateful(starting_indexes, &core_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; sent_in_profit.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_loss.height);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.core.compute_rest_part1(blocks, starting_indexes, exit)?;
        self.sent_in_profit_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.sent_in_profit.height,
            exit,
        )?;
        self.sent_in_loss_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_24h_ago,
            &self.sent_in_loss.height,
            exit,
        )?;
        Ok(())
    }
}
