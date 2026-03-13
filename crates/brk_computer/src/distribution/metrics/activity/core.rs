use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, Sats, StoredF64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    distribution::{metrics::ImportConfig, state::{CohortState, CostBasisOps, RealizedOps}},
    internal::{AmountPerBlockCumulativeWithSums, PerBlockCumulativeWithSums},
    prices,
};

#[derive(Traversable)]
pub struct ActivityCore<M: StorageMode = Rw> {
    pub sent: PerBlockCumulativeWithSums<Sats, Sats, M>,
    pub coindays_destroyed: PerBlockCumulativeWithSums<StoredF64, StoredF64, M>,
    #[traversable(wrap = "sent", rename = "in_profit")]
    pub sent_in_profit: AmountPerBlockCumulativeWithSums<M>,
    #[traversable(wrap = "sent", rename = "in_loss")]
    pub sent_in_loss: AmountPerBlockCumulativeWithSums<M>,
}

impl ActivityCore {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            sent: cfg.import("sent", v1)?,
            coindays_destroyed: cfg.import("coindays_destroyed", v1)?,
            sent_in_profit: cfg.import("sent_in_profit", v1)?,
            sent_in_loss: cfg.import("sent_in_loss", v1)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.sent
            .base
            .height
            .len()
            .min(self.coindays_destroyed.base.height.len())
            .min(self.sent_in_profit.base.sats.height.len())
            .min(self.sent_in_loss.base.sats.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &CohortState<impl RealizedOps, impl CostBasisOps>,
    ) -> Result<()> {
        self.sent.base.height.truncate_push(height, state.sent)?;
        self.coindays_destroyed.base.height.truncate_push(
            height,
            StoredF64::from(Bitcoin::from(state.satdays_destroyed)),
        )?;
        self.sent_in_profit
            .base
            .sats
            .height
            .truncate_push(height, state.realized.sent_in_profit())?;
        self.sent_in_loss
            .base
            .sats
            .height
            .truncate_push(height, state.realized.sent_in_loss())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.sent.base.height as &mut dyn AnyStoredVec,
            &mut self.coindays_destroyed.base.height,
            &mut self.sent_in_profit.base.sats.height,
            &mut self.sent_in_profit.base.cents.height,
            &mut self.sent_in_loss.base.sats.height,
            &mut self.sent_in_loss.base.cents.height,
        ]
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
        self.sent.base.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent.base.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        sum_others!(self, starting_indexes, others, exit; coindays_destroyed.base.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_profit.base.sats.height);
        sum_others!(self, starting_indexes, others, exit; sent_in_loss.base.sats.height);

        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent
            .compute_rest(starting_indexes.height, exit)?;
        self.coindays_destroyed
            .compute_rest(starting_indexes.height, exit)?;
        Ok(())
    }

    pub(crate) fn compute_sent_profitability(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent_in_profit
            .compute_rest(starting_indexes.height, prices, exit)?;
        self.sent_in_loss
            .compute_rest(starting_indexes.height, prices, exit)?;
        Ok(())
    }
}
