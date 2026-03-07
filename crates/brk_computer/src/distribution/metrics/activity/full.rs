use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, Sats, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::internal::{ComputedFromHeightCumulative, ComputedFromHeightCumulativeSum};

use crate::{blocks, distribution::metrics::ImportConfig, prices};

use super::ActivityBase;

#[derive(Deref, DerefMut, Traversable)]
pub struct ActivityFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: ActivityBase<M>,

    pub coinblocks_destroyed: ComputedFromHeightCumulative<StoredF64, M>,
    pub coindays_destroyed: ComputedFromHeightCumulativeSum<StoredF64, M>,
}

impl ActivityFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            base: ActivityBase::forced_import(cfg)?,
            coinblocks_destroyed: cfg
                .import("coinblocks_destroyed", Version::ONE)?,
            coindays_destroyed: cfg.import("coindays_destroyed", Version::ONE)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.base
            .min_len()
            .min(self.coinblocks_destroyed.height.len())
            .min(self.coindays_destroyed.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        sent: Sats,
        satblocks_destroyed: Sats,
        satdays_destroyed: Sats,
    ) -> Result<()> {
        self.base.truncate_push(height, sent)?;
        self.coinblocks_destroyed.height.truncate_push(
            height,
            StoredF64::from(Bitcoin::from(satblocks_destroyed)),
        )?;
        self.coindays_destroyed.height.truncate_push(
            height,
            StoredF64::from(Bitcoin::from(satdays_destroyed)),
        )?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.base.sent.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.coinblocks_destroyed.height as &mut dyn AnyStoredVec,
            &mut self.coindays_destroyed.height as &mut dyn AnyStoredVec,
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
        let core_refs: Vec<&ActivityBase> = others.iter().map(|o| &o.base).collect();
        self.base
            .compute_from_stateful(starting_indexes, &core_refs, exit)?;

        sum_others!(self, starting_indexes, others, exit; coinblocks_destroyed.height);
        sum_others!(self, starting_indexes, others, exit; coindays_destroyed.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.base
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;

        self.coinblocks_destroyed
            .compute_rest(starting_indexes.height, exit)?;

        let window_starts = blocks.count.window_starts();
        self.coindays_destroyed
            .compute_rest(starting_indexes.height, &window_starts, exit)?;

        Ok(())
    }
}
