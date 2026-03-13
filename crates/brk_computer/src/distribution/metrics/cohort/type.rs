use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::Indexes;
use vecdb::{AnyStoredVec, Exit, Rw, StorageMode};

use crate::{
    distribution::metrics::{
        ImportConfig, OutputsBase, RealizedMinimal, SupplyCore, UnrealizedBasic,
    },
    prices,
};

/// TypeCohortMetrics: supply(core), outputs(base), realized(minimal), unrealized(basic).
///
/// Used for type_ cohorts (p2pkh, p2sh, etc.).
#[derive(Traversable)]
pub struct TypeCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyCore<M>>,
    pub outputs: Box<OutputsBase<M>>,
    pub realized: Box<RealizedMinimal<M>>,
    pub unrealized: Box<UnrealizedBasic<M>>,
}

impl TypeCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyCore::forced_import(cfg)?),
            outputs: Box::new(OutputsBase::forced_import(cfg)?),
            realized: Box::new(RealizedMinimal::forced_import(cfg)?),
            unrealized: Box::new(UnrealizedBasic::forced_import(cfg)?),
        })
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.realized.min_stateful_len())
            .min(self.unrealized.min_stateful_len())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute(prices, starting_indexes.height, exit)?;
        self.realized
            .compute_rest_part1(starting_indexes, exit)?;
        self.unrealized
            .compute_rest(starting_indexes.height, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            exit,
        )?;

        self.unrealized.compute(
            starting_indexes.height,
            &prices.spot.cents.height,
            &self.realized.price.cents.height,
            exit,
        )?;

        Ok(())
    }
}
