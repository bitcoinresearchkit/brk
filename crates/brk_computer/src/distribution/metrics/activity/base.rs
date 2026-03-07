use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::internal::{ComputedFromHeight, RollingWindow24h};

use crate::{blocks, distribution::metrics::ImportConfig};

#[derive(Traversable)]
pub struct ActivityBase<M: StorageMode = Rw> {
    pub sent: ComputedFromHeight<Sats, M>,
    pub sent_sum: RollingWindow24h<Sats, M>,
}

impl ActivityBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v1 = Version::ONE;
        Ok(Self {
            sent: cfg.import("sent", v1)?,
            sent_sum: cfg.import("sent", v1)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.sent.height.len()
    }

    pub(crate) fn truncate_push(&mut self, height: Height, sent: Sats) -> Result<()> {
        self.sent.height.truncate_push(height, sent)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.sent.height as &mut dyn AnyStoredVec]
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
        self.sent.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent_sum.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_24h_ago,
            &self.sent.height,
            exit,
        )?;
        Ok(())
    }
}
