use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Sats, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{Exit, Rw, StorageMode};

use crate::internal::{ComputedFromHeight, RollingWindows, RollingWindowsFrom1w};

use crate::{blocks, distribution::metrics::ImportConfig};

use super::ActivityBase;

#[derive(Deref, DerefMut, Traversable)]
pub struct ActivityFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ActivityBase<M>,

    pub coinblocks_destroyed_cumulative: ComputedFromHeight<StoredF64, M>,
    pub coindays_destroyed_cumulative: ComputedFromHeight<StoredF64, M>,
    pub coindays_destroyed_sum: RollingWindows<StoredF64, M>,

    pub sent_sum_extended: RollingWindowsFrom1w<Sats, M>,
}

impl ActivityFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            inner: ActivityBase::forced_import(cfg)?,
            coinblocks_destroyed_cumulative: cfg
                .import("coinblocks_destroyed_cumulative", v1)?,
            coindays_destroyed_cumulative: cfg.import("coindays_destroyed_cumulative", v1)?,
            coindays_destroyed_sum: cfg.import("coindays_destroyed", v1)?,
            sent_sum_extended: cfg.import("sent", v1)?,
        })
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&ActivityBase],
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

        self.coinblocks_destroyed_cumulative
            .height
            .compute_cumulative(
                starting_indexes.height,
                &self.inner.coinblocks_destroyed.height,
                exit,
            )?;

        self.coindays_destroyed_cumulative
            .height
            .compute_cumulative(
                starting_indexes.height,
                &self.inner.coindays_destroyed.height,
                exit,
            )?;

        let window_starts = blocks.count.window_starts();
        self.coindays_destroyed_sum.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.inner.coindays_destroyed.height,
            exit,
        )?;

        self.sent_sum_extended.compute_rolling_sum(
            starting_indexes.height,
            &window_starts,
            &self.inner.core.sent.height,
            exit,
        )?;

        Ok(())
    }
}
