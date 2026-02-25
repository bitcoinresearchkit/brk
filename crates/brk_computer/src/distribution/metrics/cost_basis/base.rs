use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Dollars, Height};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{
    ComputeIndexes,
    distribution::state::CohortState,
    internal::{ComputedFromHeightLast, Price, PriceFromHeight},
};

use crate::distribution::metrics::ImportConfig;

/// Base cost basis metrics (always computed).
#[derive(Traversable)]
pub struct CostBasisBase<M: StorageMode = Rw> {
    /// Minimum cost basis for any UTXO at this height
    pub min: Price<ComputedFromHeightLast<Dollars, M>>,

    /// Maximum cost basis for any UTXO at this height
    pub max: Price<ComputedFromHeightLast<Dollars, M>>,
}

impl CostBasisBase {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            min: PriceFromHeight::forced_import(
                cfg.db,
                &cfg.name("min_cost_basis"),
                cfg.version,
                cfg.indexes,
            )?,
            max: PriceFromHeight::forced_import(
                cfg.db,
                &cfg.name("max_cost_basis"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.min.height.len().min(self.max.height.len())
    }

    pub(crate) fn truncate_push_minmax(
        &mut self,
        height: Height,
        state: &CohortState,
    ) -> Result<()> {
        self.min.height.truncate_push(
            height,
            state
                .cost_basis_data_first_key_value()
                .map(|(cents, _)| cents.into())
                .unwrap_or(Dollars::NAN),
        )?;
        self.max.height.truncate_push(
            height,
            state
                .cost_basis_data_last_key_value()
                .map(|(cents, _)| cents.into())
                .unwrap_or(Dollars::NAN),
        )?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.min.height as &mut dyn AnyStoredVec,
            &mut self.max.height,
        ]
    }

    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.min.height.compute_min_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.min.height).collect::<Vec<_>>(),
            exit,
        )?;
        self.max.height.compute_max_of_others(
            starting_indexes.height,
            &others.iter().map(|v| &v.max.height).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
