use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::internal::{RollingEmas2w, ValueFromHeightCumulative};

use crate::{blocks, distribution::metrics::ImportConfig, prices};

#[derive(Traversable)]
pub struct ActivityBase<M: StorageMode = Rw> {
    pub sent: ValueFromHeightCumulative<M>,
    pub sent_ema: RollingEmas2w<M>,
}

impl ActivityBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            sent: cfg.import_value_cumulative("sent", Version::ZERO)?,
            sent_ema: cfg.import_emas_2w("sent", Version::ZERO)?,
        })
    }

    pub(crate) fn min_len(&self) -> usize {
        self.sent.base.sats.height.len()
    }

    pub(crate) fn truncate_push(&mut self, height: Height, sent: Sats) -> Result<()> {
        self.sent.base.sats.height.truncate_push(height, sent)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.sent.base.sats.height as &mut dyn AnyStoredVec]
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
        self.sent.base.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.sent.base.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.sent
            .compute(prices, starting_indexes.height, exit)?;

        self.sent_ema.compute(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.sent.base.sats.height,
            &self.sent.base.cents.height,
            exit,
        )?;

        Ok(())
    }
}
