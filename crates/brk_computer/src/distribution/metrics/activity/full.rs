use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, Height, Indexes, StoredF32, StoredF64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, ReadableCloneableVec, Rw, StorageMode};

use crate::internal::{Identity, LazyPerBlock, PerBlock};

use crate::distribution::{metrics::ImportConfig, state::{CohortState, CostBasisOps, RealizedOps}};

use super::ActivityCore;

#[derive(Deref, DerefMut, Traversable)]
pub struct ActivityFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub inner: ActivityCore<M>,

    pub coinyears_destroyed: LazyPerBlock<StoredF64, StoredF64>,

    pub dormancy: PerBlock<StoredF32, M>,
}

impl ActivityFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        let inner = ActivityCore::forced_import(cfg)?;

        let coinyears_destroyed = LazyPerBlock::from_height_source::<Identity<StoredF64>>(
            &cfg.name("coinyears_destroyed"),
            cfg.version + v1,
            inner.coindays_destroyed.sum._1y.height.read_only_boxed_clone(),
            cfg.indexes,
        );

        Ok(Self {
            inner,
            coinyears_destroyed,
            dormancy: cfg.import("dormancy", v1)?,
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.inner.compute_rest_part1(starting_indexes, exit)
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.dormancy.height.compute_transform2(
            starting_indexes.height,
            &self.inner.coindays_destroyed.base.height,
            &self.inner.sent.base.height,
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

        Ok(())
    }
}
