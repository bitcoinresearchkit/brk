use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, Sats, SatsSigned, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode};

use crate::{blocks, internal::RollingDelta1m};

use crate::distribution::metrics::ImportConfig;

use super::SupplyCore;

/// Full supply metrics: total + in_profit/in_loss + delta (6 stored vecs).
#[derive(Deref, DerefMut, Traversable)]
pub struct SupplyFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub core: SupplyCore<M>,

    pub delta: RollingDelta1m<Sats, SatsSigned, M>,
}

impl SupplyFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let core = SupplyCore::forced_import(cfg)?;
        let delta = cfg.import("supply_delta", Version::ONE)?;

        Ok(Self { core, delta })
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.core.collect_vecs_mut()
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
        let core_refs: Vec<&SupplyCore> = others.iter().map(|o| &o.core).collect();
        self.core
            .compute_from_stateful(starting_indexes, &core_refs, exit)
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.delta.compute(
            starting_indexes.height,
            &blocks.lookback._1m,
            &self.core.total.sats.height,
            exit,
        )
    }
}
