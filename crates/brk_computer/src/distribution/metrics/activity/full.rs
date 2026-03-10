use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, Sats, StoredF32, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode};

use crate::internal::{ComputedPerBlock, Identity, LazyPerBlock, RollingWindowsFrom1w};

use crate::{blocks, distribution::{metrics::ImportConfig, state::{CohortState, CostBasisOps, RealizedOps}}};

use super::ActivityCore;

#[derive(Deref, DerefMut, Traversable)]
pub struct ActivityFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ActivityCore<M>,

    #[traversable(wrap = "coindays_destroyed", rename = "cumulative")]
    pub coindays_destroyed_cumulative: ComputedPerBlock<StoredF64, M>,
    #[traversable(wrap = "coindays_destroyed", rename = "sum")]
    pub coindays_destroyed_sum: RollingWindowsFrom1w<StoredF64, M>,

    #[traversable(wrap = "sent", rename = "sum")]
    pub sent_sum_extended: RollingWindowsFrom1w<Sats, M>,
    #[traversable(wrap = "sent/in_profit", rename = "sum")]
    pub sent_in_profit_sum_extended: RollingWindowsFrom1w<Sats, M>,
    #[traversable(wrap = "sent/in_loss", rename = "sum")]
    pub sent_in_loss_sum_extended: RollingWindowsFrom1w<Sats, M>,

    pub coinyears_destroyed: LazyPerBlock<StoredF64, StoredF64>,

    pub dormancy: ComputedPerBlock<StoredF32, M>,
    pub velocity: ComputedPerBlock<StoredF32, M>,
}

impl ActivityFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let coindays_destroyed_sum: RollingWindowsFrom1w<StoredF64> =
            cfg.import("coindays_destroyed", v1)?;

        let coinyears_destroyed = LazyPerBlock::from_computed::<Identity<StoredF64>>(
            &cfg.name("coinyears_destroyed"),
            v1,
            coindays_destroyed_sum._1y.height.read_only_boxed_clone(),
            &coindays_destroyed_sum._1y,
        );

        Ok(Self {
            inner: ActivityCore::forced_import(cfg)?,
            coindays_destroyed_cumulative: cfg.import("coindays_destroyed_cumulative", v1)?,
            coindays_destroyed_sum,
            sent_sum_extended: cfg.import("sent", v1)?,
            sent_in_profit_sum_extended: cfg.import("sent_in_profit", v1)?,
            sent_in_loss_sum_extended: cfg.import("sent_in_loss", v1)?,
            coinyears_destroyed,
            dormancy: cfg.import("dormancy", v1)?,
            velocity: cfg.import("velocity", v1)?,
        })
    }

    pub(crate) fn full_min_len(&self) -> usize {
        self.inner.min_len()
    }

    pub(crate) fn full_truncate_push(
        &mut self,
        height: Height,
        state: &CohortState<impl RealizedOps, impl CostBasisOps>,
    ) -> Result<()> {
        self.inner.truncate_push(height, state)
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = self.inner.collect_vecs_mut();
        vecs.push(&mut self.dormancy.height);
        vecs.push(&mut self.velocity.height);
        vecs
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&ActivityCore],
        exit: &Exit,
    ) -> Result<()> {
        self.inner
            .compute_from_stateful(starting_indexes, others, exit)
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.inner
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.coindays_destroyed_cumulative
            .height
            .compute_cumulative(
                starting_indexes.height,
                &self.inner.coindays_destroyed.raw.height,
                exit,
            )?;

        let window_starts = blocks.lookback.window_starts();
        self.coindays_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.inner.coindays_destroyed.raw.height,
            exit,
        )?;

        self.sent_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.inner.sent.raw.height,
            exit,
        )?;

        self.sent_in_profit_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.inner.sent_in_profit.raw.sats.height,
            exit,
        )?;
        self.sent_in_loss_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.inner.sent_in_loss.raw.sats.height,
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        starting_indexes: &Indexes,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.dormancy.height.compute_transform2(
            starting_indexes.height,
            &self.inner.coindays_destroyed.raw.height,
            &self.inner.sent.raw.height,
            |(i, cdd, sent_sats, ..)| {
                let sent_btc = f64::from(Bitcoin::from(sent_sats));
                if sent_btc == 0.0 {
                    (i, StoredF32::from(0.0f32))
                } else {
                    (i, StoredF32::from((f64::from(cdd) / sent_btc) as f32))
                }
            },
            exit,
        )?;

        self.velocity.height.compute_transform2(
            starting_indexes.height,
            &self.inner.sent.raw.height,
            supply_total_sats,
            |(i, sent_sats, supply_sats, ..)| {
                let supply = supply_sats.as_u128() as f64;
                if supply == 0.0 {
                    (i, StoredF32::from(0.0f32))
                } else {
                    (i, StoredF32::from((sent_sats.as_u128() as f64 / supply) as f32))
                }
            },
            exit,
        )?;

        Ok(())
    }
}
