use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{DateIndex, Dollars, Height};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec, Negate};

use crate::{
    ComputeIndexes,
    distribution::state::UnrealizedState,
    internal::{
        ComputedFromHeightAndDateLast, DollarsMinus, DollarsPlus, LazyBinaryFromHeightLast, LazyFromHeightLast,
        ValueFromHeightAndDateLast,
    },
};

use super::ImportConfig;

/// Unrealized profit/loss metrics.
#[derive(Clone, Traversable)]
pub struct UnrealizedMetrics {
    // === Supply in Profit/Loss ===
    pub supply_in_profit: ValueFromHeightAndDateLast,
    pub supply_in_loss: ValueFromHeightAndDateLast,

    // === Unrealized Profit/Loss ===
    pub unrealized_profit: ComputedFromHeightAndDateLast<Dollars>,
    pub unrealized_loss: ComputedFromHeightAndDateLast<Dollars>,

    // === Negated ===
    pub neg_unrealized_loss: LazyFromHeightLast<Dollars>,

    // === Net and Total ===
    pub net_unrealized_pnl: LazyBinaryFromHeightLast<Dollars>,
    pub total_unrealized_pnl: LazyBinaryFromHeightLast<Dollars>,
}

impl UnrealizedMetrics {
    /// Import unrealized metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let compute_dollars = cfg.compute_dollars();

        // === Supply in Profit/Loss ===
        let supply_in_profit = ValueFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_profit"),
            cfg.version,
            compute_dollars,
            cfg.indexes,
            cfg.price,
        )?;
        let supply_in_loss = ValueFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("supply_in_loss"),
            cfg.version,
            compute_dollars,
            cfg.indexes,
            cfg.price,
        )?;

        // === Unrealized Profit/Loss ===
        let unrealized_profit = ComputedFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_profit"),
            cfg.version,
            cfg.indexes,
        )?;
        let unrealized_loss = ComputedFromHeightAndDateLast::forced_import(
            cfg.db,
            &cfg.name("unrealized_loss"),
            cfg.version,
            cfg.indexes,
        )?;

        // === Negated ===
        let neg_unrealized_loss = LazyFromHeightLast::from_computed_height_date::<Negate>(
            &cfg.name("neg_unrealized_loss"),
            cfg.version,
            &unrealized_loss,
        );

        // === Net and Total ===
        let net_unrealized_pnl = LazyBinaryFromHeightLast::from_computed_height_date_last::<DollarsMinus>(
            &cfg.name("net_unrealized_pnl"),
            cfg.version,
            &unrealized_profit,
            &unrealized_loss,
        );
        let total_unrealized_pnl = LazyBinaryFromHeightLast::from_computed_height_date_last::<DollarsPlus>(
            &cfg.name("total_unrealized_pnl"),
            cfg.version,
            &unrealized_profit,
            &unrealized_loss,
        );

        Ok(Self {
            supply_in_profit,
            supply_in_loss,
            unrealized_profit,
            unrealized_loss,
            neg_unrealized_loss,
            net_unrealized_pnl,
            total_unrealized_pnl,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .height
            .len()
            .min(self.supply_in_loss.height.len())
            .min(self.unrealized_profit.height.len())
            .min(self.unrealized_loss.height.len())
    }

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    pub fn min_stateful_dateindex_len(&self) -> usize {
        self.supply_in_profit
            .indexes
            .sats_dateindex
            .len()
            .min(self.supply_in_loss.indexes.sats_dateindex.len())
            .min(self.unrealized_profit.dateindex.len())
            .min(self.unrealized_loss.dateindex.len())
    }

    /// Push unrealized state values to height-indexed vectors.
    pub fn truncate_push(
        &mut self,
        height: Height,
        dateindex: Option<DateIndex>,
        height_state: &UnrealizedState,
        date_state: Option<&UnrealizedState>,
    ) -> Result<()> {
        self.supply_in_profit
            .height
            .truncate_push(height, height_state.supply_in_profit)?;
        self.supply_in_loss
            .height
            .truncate_push(height, height_state.supply_in_loss)?;
        self.unrealized_profit
            .height
            .truncate_push(height, height_state.unrealized_profit)?;
        self.unrealized_loss
            .height
            .truncate_push(height, height_state.unrealized_loss)?;

        if let (Some(dateindex), Some(date_state)) = (dateindex, date_state) {
            self.supply_in_profit
                .indexes
                .sats_dateindex
                .truncate_push(dateindex, date_state.supply_in_profit)?;
            self.supply_in_loss
                .indexes
                .sats_dateindex
                .truncate_push(dateindex, date_state.supply_in_loss)?;
            self.unrealized_profit
                .dateindex
                .truncate_push(dateindex, date_state.unrealized_profit)?;
            self.unrealized_loss
                .dateindex
                .truncate_push(dateindex, date_state.unrealized_loss)?;
        }

        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.supply_in_profit.height.write()?;
        self.supply_in_loss.height.write()?;
        self.unrealized_profit.height.write()?;
        self.unrealized_loss.height.write()?;
        self.supply_in_profit.indexes.sats_dateindex.write()?;
        self.supply_in_loss.indexes.sats_dateindex.write()?;
        self.unrealized_profit.dateindex.write()?;
        self.unrealized_loss.dateindex.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.height as &mut dyn AnyStoredVec,
            &mut self.unrealized_loss.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.indexes.sats_dateindex as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.indexes.sats_dateindex as &mut dyn AnyStoredVec,
            &mut self.unrealized_profit.rest.dateindex as &mut dyn AnyStoredVec,
            &mut self.unrealized_loss.rest.dateindex as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply_in_profit.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.supply_in_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply_in_loss.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_profit.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.unrealized_profit.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_loss.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.unrealized_loss.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.supply_in_profit
            .indexes
            .sats_dateindex
            .compute_sum_of_others(
                starting_indexes.dateindex,
                &others
                    .iter()
                    .map(|v| &v.supply_in_profit.indexes.sats_dateindex)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.supply_in_loss
            .indexes
            .sats_dateindex
            .compute_sum_of_others(
                starting_indexes.dateindex,
                &others
                    .iter()
                    .map(|v| &v.supply_in_loss.indexes.sats_dateindex)
                    .collect::<Vec<_>>(),
                exit,
            )?;
        self.unrealized_profit.dateindex.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.unrealized_profit.dateindex)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_loss.dateindex.compute_sum_of_others(
            starting_indexes.dateindex,
            &others
                .iter()
                .map(|v| &v.unrealized_loss.dateindex)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// First phase of computed metrics.
    pub fn compute_rest_part1(
        &mut self,
        price: Option<&crate::price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit
            .compute_dollars_from_price(price, starting_indexes, exit)?;

        self.supply_in_loss
            .compute_dollars_from_price(price, starting_indexes, exit)?;

        Ok(())
    }
}
