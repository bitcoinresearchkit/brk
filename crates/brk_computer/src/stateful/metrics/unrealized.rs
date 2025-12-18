//! Unrealized profit/loss metrics.
//!
//! These metrics track paper gains/losses based on current vs acquisition price.

use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height, Sats, Version};
use vecdb::{
    AnyStoredVec, EagerVec, Exit, GenericStoredVec, ImportableVec, IterableCloneableVec, PcoVec,
};

use crate::{
    Indexes,
    grouped::{
        ComputedHeightValueVecs, ComputedValueVecsFromDateIndex, ComputedVecsFromDateIndex, Source,
        VecBuilderOptions,
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
    pub height_to_neg_unrealized_loss: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_neg_unrealized_loss: ComputedVecsFromDateIndex<Dollars>,
    pub height_to_net_unrealized_pnl: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_net_unrealized_pnl: ComputedVecsFromDateIndex<Dollars>,
    pub height_to_total_unrealized_pnl: EagerVec<PcoVec<Height, Dollars>>,
    pub indexes_to_total_unrealized_pnl: ComputedVecsFromDateIndex<Dollars>,
}

impl UnrealizedMetrics {
    /// Import unrealized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let compute_dollars = cfg.compute_dollars();
        let last = VecBuilderOptions::default().add_last();

        // Pre-import the dateindex vecs that are used as sources
        let dateindex_to_supply_in_profit =
            EagerVec::forced_import(cfg.db, &cfg.name("supply_in_profit"), cfg.version + v0)?;
        let dateindex_to_supply_in_loss =
            EagerVec::forced_import(cfg.db, &cfg.name("supply_in_loss"), cfg.version + v0)?;
        let dateindex_to_unrealized_profit =
            EagerVec::forced_import(cfg.db, &cfg.name("unrealized_profit"), cfg.version + v0)?;
        let dateindex_to_unrealized_loss =
            EagerVec::forced_import(cfg.db, &cfg.name("unrealized_loss"), cfg.version + v0)?;

        Ok(Self {
            // === Supply in Profit/Loss ===
            height_to_supply_in_profit: EagerVec::forced_import(
                cfg.db,
                &cfg.name("supply_in_profit"),
                cfg.version + v0,
            )?,
            indexes_to_supply_in_profit: ComputedValueVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("supply_in_profit"),
                Source::Vec(dateindex_to_supply_in_profit.boxed_clone()),
                cfg.version + v0,
                last,
                compute_dollars,
                cfg.indexes,
            )?,
            height_to_supply_in_loss: EagerVec::forced_import(
                cfg.db,
                &cfg.name("supply_in_loss"),
                cfg.version + v0,
            )?,
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
            height_to_supply_in_profit_value: ComputedHeightValueVecs::forced_import(
                cfg.db,
                &cfg.name("supply_in_profit"),
                Source::None,
                cfg.version + v0,
                compute_dollars,
            )?,
            height_to_supply_in_loss_value: ComputedHeightValueVecs::forced_import(
                cfg.db,
                &cfg.name("supply_in_loss"),
                Source::None,
                cfg.version + v0,
                compute_dollars,
            )?,

            // === Unrealized Profit/Loss ===
            height_to_unrealized_profit: EagerVec::forced_import(
                cfg.db,
                &cfg.name("unrealized_profit"),
                cfg.version + v0,
            )?,
            indexes_to_unrealized_profit: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("unrealized_profit"),
                Source::Vec(dateindex_to_unrealized_profit.boxed_clone()),
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            height_to_unrealized_loss: EagerVec::forced_import(
                cfg.db,
                &cfg.name("unrealized_loss"),
                cfg.version + v0,
            )?,
            indexes_to_unrealized_loss: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("unrealized_loss"),
                Source::Vec(dateindex_to_unrealized_loss.boxed_clone()),
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            dateindex_to_unrealized_profit,
            dateindex_to_unrealized_loss,

            // === Negated and Net ===
            height_to_neg_unrealized_loss: EagerVec::forced_import(
                cfg.db,
                &cfg.name("neg_unrealized_loss"),
                cfg.version + v0,
            )?,
            indexes_to_neg_unrealized_loss: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("neg_unrealized_loss"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            height_to_net_unrealized_pnl: EagerVec::forced_import(
                cfg.db,
                &cfg.name("net_unrealized_pnl"),
                cfg.version + v0,
            )?,
            indexes_to_net_unrealized_pnl: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("net_unrealized_pnl"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
            height_to_total_unrealized_pnl: EagerVec::forced_import(
                cfg.db,
                &cfg.name("total_unrealized_pnl"),
                cfg.version + v0,
            )?,
            indexes_to_total_unrealized_pnl: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("total_unrealized_pnl"),
                Source::Compute,
                cfg.version + v0,
                cfg.indexes,
                last,
            )?,
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
    pub fn safe_write(&mut self, exit: &Exit) -> Result<()> {
        self.height_to_supply_in_profit.safe_write(exit)?;
        self.height_to_supply_in_loss.safe_write(exit)?;
        self.height_to_unrealized_profit.safe_write(exit)?;
        self.height_to_unrealized_loss.safe_write(exit)?;
        self.dateindex_to_supply_in_profit.safe_write(exit)?;
        self.dateindex_to_supply_in_loss.safe_write(exit)?;
        self.dateindex_to_unrealized_profit.safe_write(exit)?;
        self.dateindex_to_unrealized_loss.safe_write(exit)?;
        Ok(())
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
}
