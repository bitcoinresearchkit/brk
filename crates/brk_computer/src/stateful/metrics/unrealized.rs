use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Sats, Version};
use rayon::prelude::*;
use vecdb::{
    AnyStoredVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec,
    LazyVecFrom1, LazyVecFrom2, Negate, PcoVec,
};

use crate::{
    Indexes,
    grouped::{
        ComputedHeightValueVecs, ComputedValueVecsFromDateIndex, ComputedVecsFromDateIndex,
        DollarsMinus, DollarsPlus, LazyVecsFromDateIndex, Source, VecBuilderOptions,
    },
    stateful::states::UnrealizedState,
};

use super::ImportConfig;

/// Unrealized profit/loss metrics.
#[derive(Clone, Traversable)]
pub struct UnrealizedMetrics {
    // === Supply in Profit/Loss ===
    pub height_to_supply_in_profit: EagerVec<PcoVec<Height, Sats>>,
    pub indexes_to_supply_in_profit: ComputedValueVecsFromDateIndex,
    pub height_to_supply_in_loss: EagerVec<PcoVec<Height, Sats>>,
    pub indexes_to_supply_in_loss: ComputedValueVecsFromDateIndex,
    pub dateindex_to_supply_in_profit: EagerVec<PcoVec<DateIndex, Sats>>,
    pub dateindex_to_supply_in_loss: EagerVec<PcoVec<DateIndex, Sats>>,
    pub height_to_supply_in_profit_value: ComputedHeightValueVecs,
    pub height_to_supply_in_loss_value: ComputedHeightValueVecs,

    // === Unrealized Profit/Loss ===
    pub height_to_unrealized_profit: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_unrealized_profit: ComputedVecsFromDateIndex<Dollars>,
    pub height_to_unrealized_loss: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_unrealized_loss: ComputedVecsFromDateIndex<Dollars>,
    pub dateindex_to_unrealized_profit: EagerVec<PcoVec<DateIndex, Dollars>>,
    pub dateindex_to_unrealized_loss: EagerVec<PcoVec<DateIndex, Dollars>>,

    // === Negated and Net ===
    pub height_to_neg_unrealized_loss: LazyVecFrom1<Height, Dollars, Height, Dollars>,
    pub indexes_to_neg_unrealized_loss: LazyVecsFromDateIndex<Dollars>,

    // net = profit - loss (height is lazy, indexes computed)
    pub height_to_net_unrealized_pnl:
        LazyVecFrom2<Height, Dollars, Height, Dollars, Height, Dollars>,
    pub indexes_to_net_unrealized_pnl: ComputedVecsFromDateIndex<Dollars>,

    // total = profit + loss (height is lazy, indexes computed)
    pub height_to_total_unrealized_pnl:
        LazyVecFrom2<Height, Dollars, Height, Dollars, Height, Dollars>,
    pub indexes_to_total_unrealized_pnl: ComputedVecsFromDateIndex<Dollars>,
}

impl UnrealizedMetrics {
    /// Import unrealized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let compute_dollars = cfg.compute_dollars();
        let last = VecBuilderOptions::default().add_last();

        let dateindex_to_supply_in_profit =
            EagerVec::forced_import(cfg.db, &cfg.name("supply_in_profit"), cfg.version + v0)?;
        let dateindex_to_supply_in_loss =
            EagerVec::forced_import(cfg.db, &cfg.name("supply_in_loss"), cfg.version + v0)?;
        let dateindex_to_unrealized_profit =
            EagerVec::forced_import(cfg.db, &cfg.name("unrealized_profit"), cfg.version + v0)?;
        let dateindex_to_unrealized_loss =
            EagerVec::forced_import(cfg.db, &cfg.name("unrealized_loss"), cfg.version + v0)?;
        let height_to_unrealized_loss: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("unrealized_loss"), cfg.version + v0)?;
        let height_to_neg_unrealized_loss = LazyVecFrom1::transformed::<Negate>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version + v0,
            height_to_unrealized_loss.boxed_clone(),
        );

        let indexes_to_unrealized_loss = ComputedVecsFromDateIndex::forced_import(
            cfg.db,
            &cfg.name("unrealized_loss"),
            Source::Vec(dateindex_to_unrealized_loss.boxed_clone()),
            cfg.version + v0,
            cfg.indexes,
            last,
        )?;

        let indexes_to_neg_unrealized_loss = LazyVecsFromDateIndex::from_computed::<Negate>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version + v0,
            Some(dateindex_to_unrealized_loss.boxed_clone()),
            &indexes_to_unrealized_loss,
        );

        // Extract profit sources for lazy net/total vecs
        let height_to_unrealized_profit: EagerVec<PcoVec<Height, Dollars>> =
            EagerVec::forced_import(cfg.db, &cfg.name("unrealized_profit"), cfg.version + v0)?;
        let indexes_to_unrealized_profit = ComputedVecsFromDateIndex::forced_import(
            cfg.db,
            &cfg.name("unrealized_profit"),
            Source::Vec(dateindex_to_unrealized_profit.boxed_clone()),
            cfg.version + v0,
            cfg.indexes,
            last,
        )?;

        // Create lazy height vecs from profit/loss sources
        let height_to_net_unrealized_pnl = LazyVecFrom2::transformed::<DollarsMinus>(
            &cfg.name("net_unrealized_pnl"),
            cfg.version + v0,
            height_to_unrealized_profit.boxed_clone(),
            height_to_unrealized_loss.boxed_clone(),
        );
        let height_to_total_unrealized_pnl = LazyVecFrom2::transformed::<DollarsPlus>(
            &cfg.name("total_unrealized_pnl"),
            cfg.version + v0,
            height_to_unrealized_profit.boxed_clone(),
            height_to_unrealized_loss.boxed_clone(),
        );

        // indexes_to_net/total remain computed (needed by relative.rs)
        let indexes_to_net_unrealized_pnl = ComputedVecsFromDateIndex::forced_import(
            cfg.db,
            &cfg.name("net_unrealized_pnl"),
            Source::Compute,
            cfg.version + v0,
            cfg.indexes,
            last,
        )?;
        let indexes_to_total_unrealized_pnl = ComputedVecsFromDateIndex::forced_import(
            cfg.db,
            &cfg.name("total_unrealized_pnl"),
            Source::Compute,
            cfg.version + v0,
            cfg.indexes,
            last,
        )?;

        let height_to_supply_in_profit: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(cfg.db, &cfg.name("supply_in_profit"), cfg.version + v0)?;
        let height_to_supply_in_loss: EagerVec<PcoVec<Height, Sats>> =
            EagerVec::forced_import(cfg.db, &cfg.name("supply_in_loss"), cfg.version + v0)?;

        let price_source = cfg
            .price
            .map(|p| p.chainindexes_to_price_close.height.boxed_clone());

        let height_to_supply_in_profit_value = ComputedHeightValueVecs::forced_import(
            cfg.db,
            &cfg.name("supply_in_profit"),
            Source::Vec(height_to_supply_in_profit.boxed_clone()),
            cfg.version + v0,
            price_source.clone(),
        )?;
        let height_to_supply_in_loss_value = ComputedHeightValueVecs::forced_import(
            cfg.db,
            &cfg.name("supply_in_loss"),
            Source::Vec(height_to_supply_in_loss.boxed_clone()),
            cfg.version + v0,
            price_source,
        )?;

        Ok(Self {
            // === Supply in Profit/Loss ===
            height_to_supply_in_profit,
            indexes_to_supply_in_profit: ComputedValueVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("supply_in_profit"),
                Source::Vec(dateindex_to_supply_in_profit.boxed_clone()),
                cfg.version + v0,
                last,
                compute_dollars,
                cfg.indexes,
            )?,
            height_to_supply_in_loss,
            indexes_to_supply_in_loss: ComputedValueVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("supply_in_loss"),
                Source::Vec(dateindex_to_supply_in_loss.boxed_clone()),
                cfg.version + v0,
                last,
                compute_dollars,
                cfg.indexes,
            )?,
            dateindex_to_supply_in_profit,
            dateindex_to_supply_in_loss,
            height_to_supply_in_profit_value,
            height_to_supply_in_loss_value,

            // === Unrealized Profit/Loss ===
            height_to_unrealized_profit,
            indexes_to_unrealized_profit,
            height_to_unrealized_loss,
            indexes_to_unrealized_loss,
            dateindex_to_unrealized_profit,
            dateindex_to_unrealized_loss,

            height_to_neg_unrealized_loss,
            indexes_to_neg_unrealized_loss,
            height_to_net_unrealized_pnl,
            indexes_to_net_unrealized_pnl,
            height_to_total_unrealized_pnl,
            indexes_to_total_unrealized_pnl,
        })
    }

    /// Push unrealized state values to height-indexed vectors.
    pub fn truncate_push(
        &mut self,
        height: Height,
        dateindex: Option<DateIndex>,
        height_state: &UnrealizedState,
        date_state: Option<&UnrealizedState>,
    ) -> Result<()> {
        self.height_to_supply_in_profit
            .truncate_push(height, height_state.supply_in_profit)?;
        self.height_to_supply_in_loss
            .truncate_push(height, height_state.supply_in_loss)?;
        self.height_to_unrealized_profit
            .truncate_push(height, height_state.unrealized_profit)?;
        self.height_to_unrealized_loss
            .truncate_push(height, height_state.unrealized_loss)?;

        if let (Some(dateindex), Some(date_state)) = (dateindex, date_state) {
            self.dateindex_to_supply_in_profit
                .truncate_push(dateindex, date_state.supply_in_profit)?;
            self.dateindex_to_supply_in_loss
                .truncate_push(dateindex, date_state.supply_in_loss)?;
            self.dateindex_to_unrealized_profit
                .truncate_push(dateindex, date_state.unrealized_profit)?;
            self.dateindex_to_unrealized_loss
                .truncate_push(dateindex, date_state.unrealized_loss)?;
        }

        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.height_to_supply_in_profit.write()?;
        self.height_to_supply_in_loss.write()?;
        self.height_to_unrealized_profit.write()?;
        self.height_to_unrealized_loss.write()?;
        self.dateindex_to_supply_in_profit.write()?;
        self.dateindex_to_supply_in_loss.write()?;
        self.dateindex_to_unrealized_profit.write()?;
        self.dateindex_to_unrealized_loss.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.height_to_supply_in_profit as &mut dyn AnyStoredVec,
            &mut self.height_to_supply_in_loss as &mut dyn AnyStoredVec,
            &mut self.height_to_unrealized_profit as &mut dyn AnyStoredVec,
            &mut self.height_to_unrealized_loss as &mut dyn AnyStoredVec,
            &mut self.dateindex_to_supply_in_profit as &mut dyn AnyStoredVec,
            &mut self.dateindex_to_supply_in_loss as &mut dyn AnyStoredVec,
            &mut self.dateindex_to_unrealized_profit as &mut dyn AnyStoredVec,
            &mut self.dateindex_to_unrealized_loss as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_supply_in_profit.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_supply_in_profit)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_supply_in_loss.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_supply_in_loss)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_unrealized_profit.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_unrealized_profit)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.height_to_unrealized_loss.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.height_to_unrealized_loss)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.dateindex_to_supply_in_profit.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.dateindex_to_supply_in_profit)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.dateindex_to_supply_in_loss.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.dateindex_to_supply_in_loss)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.dateindex_to_unrealized_profit.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.dateindex_to_unrealized_profit)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.dateindex_to_unrealized_loss.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.dateindex_to_unrealized_loss)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics.
    pub fn compute_rest_part1(
        &mut self,
        price: Option<&crate::price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.indexes_to_supply_in_profit.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.dateindex_to_supply_in_profit),
        )?;

        self.indexes_to_supply_in_loss.compute_rest(
            price,
            starting_indexes,
            exit,
            Some(&self.dateindex_to_supply_in_loss),
        )?;

        self.indexes_to_unrealized_profit.compute_rest(
            starting_indexes,
            exit,
            Some(&self.dateindex_to_unrealized_profit),
        )?;

        self.indexes_to_unrealized_loss.compute_rest(
            starting_indexes,
            exit,
            Some(&self.dateindex_to_unrealized_loss),
        )?;

        // height_to_net/total are lazy, but indexes still need compute
        // total_unrealized_pnl = profit + loss
        self.indexes_to_total_unrealized_pnl
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_add(
                    starting_indexes.dateindex,
                    &self.dateindex_to_unrealized_profit,
                    &self.dateindex_to_unrealized_loss,
                    exit,
                )?;
                Ok(())
            })?;

        // net_unrealized_pnl = profit - loss
        self.indexes_to_net_unrealized_pnl
            .compute_all(starting_indexes, exit, |vec| {
                vec.compute_subtract(
                    starting_indexes.dateindex,
                    &self.dateindex_to_unrealized_profit,
                    &self.dateindex_to_unrealized_loss,
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
