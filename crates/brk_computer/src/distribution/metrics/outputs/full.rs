use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Indexes, StoredI64, StoredU64, Version};
use derive_more::{Deref, DerefMut};
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode};

use crate::{blocks, internal::RollingDelta1m};

use crate::distribution::metrics::ImportConfig;

use super::OutputsBase;

/// Full output metrics: utxo_count + delta (3 stored vecs).
#[derive(Deref, DerefMut, Traversable)]
pub struct OutputsFull<M: StorageMode = Rw> {
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub base: OutputsBase<M>,

    #[traversable(wrap = "utxo_count", rename = "delta")]
    pub utxo_count_delta: RollingDelta1m<StoredU64, StoredI64, M>,
}

impl OutputsFull {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let base = OutputsBase::forced_import(cfg)?;
        let utxo_count_delta = cfg.import("utxo_count_delta", Version::ONE)?;

        Ok(Self {
            base,
            utxo_count_delta,
        })
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.base.collect_vecs_mut()
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let base_refs: Vec<&OutputsBase> = others.iter().map(|o| &o.base).collect();
        self.base.compute_from_stateful(starting_indexes, &base_refs, exit)
    }

    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_count_delta.compute(
            starting_indexes.height,
            &blocks.lookback._1m,
            &self.base.utxo_count.height,
            exit,
        )?;

        Ok(())
    }
}
