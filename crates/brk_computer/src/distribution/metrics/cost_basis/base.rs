use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    ComputeIndexes,
    distribution::state::CohortState,
    internal::{ComputedFromHeight, Price},
};

use crate::distribution::metrics::ImportConfig;

/// Base cost basis metrics (always computed).
#[derive(Traversable)]
pub struct CostBasisBase<M: StorageMode = Rw> {
    /// Minimum cost basis for any UTXO at this height
    pub min: Price<ComputedFromHeight<Cents, M>>,

    /// Maximum cost basis for any UTXO at this height
    pub max: Price<ComputedFromHeight<Cents, M>>,
}

impl CostBasisBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            min: Price::forced_import(
                cfg.db,
                &cfg.name("cost_basis_min"),
                cfg.version,
                cfg.indexes,
            )?,
            max: Price::forced_import(
                cfg.db,
                &cfg.name("cost_basis_max"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.min.cents.height.len().min(self.max.cents.height.len())
    }

    pub(crate) fn truncate_push_minmax(
        &mut self,
        height: Height,
        state: &CohortState,
    ) -> Result<()> {
        self.min.cents.height.truncate_push(
            height,
            state
                .cost_basis_data_first_key_value()
                .map(|(cents, _)| cents)
                .unwrap_or(Cents::ZERO),
        )?;
        self.max.cents.height.truncate_push(
            height,
            state
                .cost_basis_data_last_key_value()
                .map(|(cents, _)| cents)
                .unwrap_or(Cents::ZERO),
        )?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.min.cents.height as &mut dyn AnyStoredVec,
            &mut self.max.cents.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.min.cents.height.compute_min_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.min.cents.height).collect::<Vec<_>>(),
            exit,
        )?;
        self.max.cents.height.compute_max_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.max.cents.height).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
