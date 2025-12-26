use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, StoredF32, StoredF64, Version};
use vecdb::{EagerVec, Exit, ImportableVec, IterableVec, PcoVec};

use crate::{
    Indexes,
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes,
};

use super::{ImportConfig, RealizedMetrics, SupplyMetrics};

/// Relative metrics comparing cohort values to global values.
#[derive(Clone, Traversable)]
pub struct RelativeMetrics {
    // === Supply Relative to Circulating Supply ===
    pub indexes_to_supply_rel_to_circulating_supply: Option<ComputedVecsFromHeight<StoredF64>>,

    // === Supply in Profit/Loss Relative to Own Supply ===
    pub height_to_supply_in_profit_rel_to_own_supply: EagerVec<PcoVec<Height, StoredF64>>,
    pub height_to_supply_in_loss_rel_to_own_supply: EagerVec<PcoVec<Height, StoredF64>>,
    pub indexes_to_supply_in_profit_rel_to_own_supply: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_supply_in_loss_rel_to_own_supply: ComputedVecsFromDateIndex<StoredF64>,

    // === Supply in Profit/Loss Relative to Circulating Supply ===
    pub height_to_supply_in_profit_rel_to_circulating_supply:
        Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub height_to_supply_in_loss_rel_to_circulating_supply:
        Option<EagerVec<PcoVec<Height, StoredF64>>>,
    pub indexes_to_supply_in_profit_rel_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,
    pub indexes_to_supply_in_loss_rel_to_circulating_supply:
        Option<ComputedVecsFromDateIndex<StoredF64>>,

    // === Unrealized vs Market Cap ===
    pub height_to_unrealized_profit_rel_to_market_cap: EagerVec<PcoVec<Height, StoredF32>>,
    pub height_to_unrealized_loss_rel_to_market_cap: EagerVec<PcoVec<Height, StoredF32>>,
    pub height_to_neg_unrealized_loss_rel_to_market_cap: EagerVec<PcoVec<Height, StoredF32>>,
    pub height_to_net_unrealized_pnl_rel_to_market_cap: EagerVec<PcoVec<Height, StoredF32>>,
    pub indexes_to_unrealized_profit_rel_to_market_cap: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_unrealized_loss_rel_to_market_cap: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_neg_unrealized_loss_rel_to_market_cap: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_net_unrealized_pnl_rel_to_market_cap: ComputedVecsFromDateIndex<StoredF32>,

    // === Unrealized vs Own Market Cap (optional) ===
    pub height_to_unrealized_profit_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_unrealized_loss_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_neg_unrealized_loss_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_net_unrealized_pnl_rel_to_own_market_cap:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub indexes_to_unrealized_profit_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_unrealized_loss_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_neg_unrealized_loss_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_unrealized_pnl_rel_to_own_market_cap:
        Option<ComputedVecsFromDateIndex<StoredF32>>,

    // === Unrealized vs Own Total Unrealized PnL (optional) ===
    pub height_to_unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<EagerVec<PcoVec<Height, StoredF32>>>,
    pub indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
    pub indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl:
        Option<ComputedVecsFromDateIndex<StoredF32>>,
}

impl RelativeMetrics {
    /// Import relative metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let v0 = Version::ZERO;
        let v1 = Version::ONE;
        let v2 = Version::new(2);
        let extended = cfg.extended();
        let compute_rel_to_all = cfg.compute_rel_to_all();
        let last = VecBuilderOptions::default().add_last();

        Ok(Self {
            // === Supply Relative to Circulating Supply ===
            indexes_to_supply_rel_to_circulating_supply: compute_rel_to_all
                .then(|| {
                    ComputedVecsFromHeight::forced_import(
                        cfg.db,
                        &cfg.name("supply_rel_to_circulating_supply"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,

            // === Supply in Profit/Loss Relative to Own Supply ===
            height_to_supply_in_profit_rel_to_own_supply: EagerVec::forced_import(
                cfg.db,
                &cfg.name("supply_in_profit_rel_to_own_supply"),
                cfg.version + v1,
            )?,
            height_to_supply_in_loss_rel_to_own_supply: EagerVec::forced_import(
                cfg.db,
                &cfg.name("supply_in_loss_rel_to_own_supply"),
                cfg.version + v1,
            )?,
            indexes_to_supply_in_profit_rel_to_own_supply:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("supply_in_profit_rel_to_own_supply"),
                    Source::Compute,
                    cfg.version + v1,
                    cfg.indexes,
                    last,
                )?,
            indexes_to_supply_in_loss_rel_to_own_supply: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("supply_in_loss_rel_to_own_supply"),
                Source::Compute,
                cfg.version + v1,
                cfg.indexes,
                last,
            )?,

            // === Supply in Profit/Loss Relative to Circulating Supply ===
            height_to_supply_in_profit_rel_to_circulating_supply: compute_rel_to_all
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            height_to_supply_in_loss_rel_to_circulating_supply: compute_rel_to_all
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            indexes_to_supply_in_profit_rel_to_circulating_supply: compute_rel_to_all
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("supply_in_profit_rel_to_circulating_supply"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_supply_in_loss_rel_to_circulating_supply: compute_rel_to_all
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("supply_in_loss_rel_to_circulating_supply"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,

            // === Unrealized vs Market Cap ===
            height_to_unrealized_profit_rel_to_market_cap: EagerVec::forced_import(
                cfg.db,
                &cfg.name("unrealized_profit_rel_to_market_cap"),
                cfg.version + v0,
            )?,
            height_to_unrealized_loss_rel_to_market_cap: EagerVec::forced_import(
                cfg.db,
                &cfg.name("unrealized_loss_rel_to_market_cap"),
                cfg.version + v0,
            )?,
            height_to_neg_unrealized_loss_rel_to_market_cap: EagerVec::forced_import(
                cfg.db,
                &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                cfg.version + v0,
            )?,
            height_to_net_unrealized_pnl_rel_to_market_cap: EagerVec::forced_import(
                cfg.db,
                &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                cfg.version + v1,
            )?,
            indexes_to_unrealized_profit_rel_to_market_cap:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("unrealized_profit_rel_to_market_cap"),
                    Source::Compute,
                    cfg.version + v1,
                    cfg.indexes,
                    last,
                )?,
            indexes_to_unrealized_loss_rel_to_market_cap: ComputedVecsFromDateIndex::forced_import(
                cfg.db,
                &cfg.name("unrealized_loss_rel_to_market_cap"),
                Source::Compute,
                cfg.version + v1,
                cfg.indexes,
                last,
            )?,
            indexes_to_neg_unrealized_loss_rel_to_market_cap:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("neg_unrealized_loss_rel_to_market_cap"),
                    Source::Compute,
                    cfg.version + v1,
                    cfg.indexes,
                    last,
                )?,
            indexes_to_net_unrealized_pnl_rel_to_market_cap:
                ComputedVecsFromDateIndex::forced_import(
                    cfg.db,
                    &cfg.name("net_unrealized_pnl_rel_to_market_cap"),
                    Source::Compute,
                    cfg.version + v1,
                    cfg.indexes,
                    last,
                )?,

            // === Unrealized vs Own Market Cap (optional) ===
            height_to_unrealized_profit_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            height_to_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            height_to_neg_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            height_to_net_unrealized_pnl_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                        cfg.version + v2,
                    )
                })
                .transpose()?,
            indexes_to_unrealized_profit_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_profit_rel_to_own_market_cap"),
                        Source::Compute,
                        cfg.version + v2,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_loss_rel_to_own_market_cap"),
                        Source::Compute,
                        cfg.version + v2,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_neg_unrealized_loss_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("neg_unrealized_loss_rel_to_own_market_cap"),
                        Source::Compute,
                        cfg.version + v2,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_net_unrealized_pnl_rel_to_own_market_cap: (extended && compute_rel_to_all)
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("net_unrealized_pnl_rel_to_own_market_cap"),
                        Source::Compute,
                        cfg.version + v2,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,

            // === Unrealized vs Own Total Unrealized PnL (optional) ===
            height_to_unrealized_profit_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                        cfg.version + v0,
                    )
                })
                .transpose()?,
            height_to_unrealized_loss_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        cfg.version + v0,
                    )
                })
                .transpose()?,
            height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        cfg.version + v0,
                    )
                })
                .transpose()?,
            height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    EagerVec::forced_import(
                        cfg.db,
                        &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                        cfg.version + v1,
                    )
                })
                .transpose()?,
            indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_profit_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("neg_unrealized_loss_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
            indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl: extended
                .then(|| {
                    ComputedVecsFromDateIndex::forced_import(
                        cfg.db,
                        &cfg.name("net_unrealized_pnl_rel_to_own_total_unrealized_pnl"),
                        Source::Compute,
                        cfg.version + v1,
                        cfg.indexes,
                        last,
                    )
                })
                .transpose()?,
        })
    }

    /// Second phase of computed metrics (ratios, relative values).
    ///
    /// This computes percentage ratios comparing cohort metrics to global metrics:
    /// - Supply relative to circulating supply
    /// - Supply in profit/loss relative to own supply and circulating supply
    /// - Unrealized profit/loss relative to market cap, own market cap, total unrealized
    ///
    /// See `stateful/common/compute.rs` lines 800-1200 for the full original implementation.
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        _height_to_realized_cap: Option<&impl IterableVec<Height, Dollars>>,
        _dateindex_to_realized_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        supply: &SupplyMetrics,
        unrealized: Option<&super::UnrealizedMetrics>,
        _realized: Option<&RealizedMetrics>,
        exit: &Exit,
    ) -> Result<()> {
        // === Supply Relative to Circulating Supply ===
        if let Some(v) = self.indexes_to_supply_rel_to_circulating_supply.as_mut() {
            v.compute_all(indexes, starting_indexes, exit, |v| {
                v.compute_percentage(
                    starting_indexes.height,
                    &supply.height_to_supply_value.bitcoin,
                    height_to_supply,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // === Supply in Profit/Loss Relative to Own Supply ===
        if let Some(unrealized) = unrealized {
            self.height_to_supply_in_profit_rel_to_own_supply
                .compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_supply_in_profit_value.bitcoin,
                    &supply.height_to_supply_value.bitcoin,
                    exit,
                )?;
            self.height_to_supply_in_loss_rel_to_own_supply
                .compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_supply_in_loss_value.bitcoin,
                    &supply.height_to_supply_value.bitcoin,
                    exit,
                )?;

            self.indexes_to_supply_in_profit_rel_to_own_supply
                .compute_all(starting_indexes, exit, |v| {
                    if let Some(dateindex_vec) = unrealized
                        .indexes_to_supply_in_profit
                        .bitcoin
                        .dateindex
                        .as_ref()
                        && let Some(supply_dateindex) =
                            supply.indexes_to_supply.bitcoin.dateindex.as_ref()
                    {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            dateindex_vec,
                            supply_dateindex,
                            exit,
                        )?;
                    }
                    Ok(())
                })?;

            self.indexes_to_supply_in_loss_rel_to_own_supply
                .compute_all(starting_indexes, exit, |v| {
                    if let Some(dateindex_vec) = unrealized
                        .indexes_to_supply_in_loss
                        .bitcoin
                        .dateindex
                        .as_ref()
                        && let Some(supply_dateindex) =
                            supply.indexes_to_supply.bitcoin.dateindex.as_ref()
                    {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            dateindex_vec,
                            supply_dateindex,
                            exit,
                        )?;
                    }
                    Ok(())
                })?;
        }

        // === Supply in Profit/Loss Relative to Circulating Supply ===
        if let (Some(unrealized), Some(v)) = (
            unrealized,
            self.height_to_supply_in_profit_rel_to_circulating_supply
                .as_mut(),
        ) {
            v.compute_percentage(
                starting_indexes.height,
                &unrealized.height_to_supply_in_profit_value.bitcoin,
                height_to_supply,
                exit,
            )?;
        }
        if let (Some(unrealized), Some(v)) = (
            unrealized,
            self.height_to_supply_in_loss_rel_to_circulating_supply
                .as_mut(),
        ) {
            v.compute_percentage(
                starting_indexes.height,
                &unrealized.height_to_supply_in_loss_value.bitcoin,
                height_to_supply,
                exit,
            )?;
        }

        // === Unrealized vs Market Cap ===
        if let (Some(unrealized), Some(height_to_mc)) = (unrealized, height_to_market_cap) {
            self.height_to_unrealized_profit_rel_to_market_cap
                .compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_unrealized_profit,
                    height_to_mc,
                    exit,
                )?;
            self.height_to_unrealized_loss_rel_to_market_cap
                .compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_unrealized_loss,
                    height_to_mc,
                    exit,
                )?;
            self.height_to_neg_unrealized_loss_rel_to_market_cap
                .compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_neg_unrealized_loss,
                    height_to_mc,
                    exit,
                )?;
            self.height_to_net_unrealized_pnl_rel_to_market_cap
                .compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_net_unrealized_pnl,
                    height_to_mc,
                    exit,
                )?;
        }

        if let Some(dateindex_to_mc) = dateindex_to_market_cap
            && let Some(unrealized) = unrealized
        {
            self.indexes_to_unrealized_profit_rel_to_market_cap
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        &unrealized.dateindex_to_unrealized_profit,
                        dateindex_to_mc,
                        exit,
                    )?;
                    Ok(())
                })?;
            self.indexes_to_unrealized_loss_rel_to_market_cap
                .compute_all(starting_indexes, exit, |v| {
                    v.compute_percentage(
                        starting_indexes.dateindex,
                        &unrealized.dateindex_to_unrealized_loss,
                        dateindex_to_mc,
                        exit,
                    )?;
                    Ok(())
                })?;
        }

        // indexes_to_neg_unrealized_loss_rel_to_market_cap
        if let Some(dateindex_to_mc) = dateindex_to_market_cap
            && let Some(unrealized) = unrealized
        {
            if let Some(dateindex_vec) =
                unrealized.indexes_to_neg_unrealized_loss.dateindex.as_ref()
            {
                self.indexes_to_neg_unrealized_loss_rel_to_market_cap
                    .compute_all(starting_indexes, exit, |v| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            dateindex_vec,
                            dateindex_to_mc,
                            exit,
                        )?;
                        Ok(())
                    })?;
            }
            if let Some(dateindex_vec) = unrealized.indexes_to_net_unrealized_pnl.dateindex.as_ref()
            {
                self.indexes_to_net_unrealized_pnl_rel_to_market_cap
                    .compute_all(starting_indexes, exit, |v| {
                        v.compute_percentage(
                            starting_indexes.dateindex,
                            dateindex_vec,
                            dateindex_to_mc,
                            exit,
                        )?;
                        Ok(())
                    })?;
            }
        }

        // === Supply in Profit/Loss Relative to Circulating Supply (indexes) ===
        if let Some(v) = self
            .indexes_to_supply_in_profit_rel_to_circulating_supply
            .as_mut()
            && let Some(unrealized) = unrealized
            && let Some(dateindex_vec) = unrealized
                .indexes_to_supply_in_profit
                .bitcoin
                .dateindex
                .as_ref()
        {
            v.compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    dateindex_vec,
                    dateindex_to_supply,
                    exit,
                )?;
                Ok(())
            })?;
        }

        if let Some(v) = self
            .indexes_to_supply_in_loss_rel_to_circulating_supply
            .as_mut()
            && let Some(unrealized) = unrealized
            && let Some(dateindex_vec) = unrealized
                .indexes_to_supply_in_loss
                .bitcoin
                .dateindex
                .as_ref()
        {
            v.compute_all(starting_indexes, exit, |vec| {
                vec.compute_percentage(
                    starting_indexes.dateindex,
                    dateindex_vec,
                    dateindex_to_supply,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // === Unrealized vs Own Market Cap ===
        // own_market_cap = supply_value.dollars
        if let Some(unrealized) = unrealized {
            if let Some(v) = self
                .height_to_unrealized_profit_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars) = supply.height_to_supply_value.dollars.as_ref()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_unrealized_profit,
                    supply_dollars,
                    exit,
                )?;
            }
            if let Some(v) = self
                .height_to_unrealized_loss_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars) = supply.height_to_supply_value.dollars.as_ref()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_unrealized_loss,
                    supply_dollars,
                    exit,
                )?;
            }
            if let Some(v) = self
                .height_to_neg_unrealized_loss_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars) = supply.height_to_supply_value.dollars.as_ref()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_neg_unrealized_loss,
                    supply_dollars,
                    exit,
                )?;
            }
            if let Some(v) = self
                .height_to_net_unrealized_pnl_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars) = supply.height_to_supply_value.dollars.as_ref()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_net_unrealized_pnl,
                    supply_dollars,
                    exit,
                )?;
            }

            // indexes versions
            if let Some(v) = self
                .indexes_to_unrealized_profit_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars_dateindex) = supply
                    .indexes_to_supply
                    .dollars
                    .as_ref()
                    .and_then(|d| d.dateindex.as_ref())
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &unrealized.dateindex_to_unrealized_profit,
                        supply_dollars_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }
            if let Some(v) = self
                .indexes_to_unrealized_loss_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars_dateindex) = supply
                    .indexes_to_supply
                    .dollars
                    .as_ref()
                    .and_then(|d| d.dateindex.as_ref())
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &unrealized.dateindex_to_unrealized_loss,
                        supply_dollars_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }
            if let Some(v) = self
                .indexes_to_neg_unrealized_loss_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars_dateindex) = supply
                    .indexes_to_supply
                    .dollars
                    .as_ref()
                    .and_then(|d| d.dateindex.as_ref())
                && let Some(neg_loss_dateindex) =
                    unrealized.indexes_to_neg_unrealized_loss.dateindex.as_ref()
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        neg_loss_dateindex,
                        supply_dollars_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }
            if let Some(v) = self
                .indexes_to_net_unrealized_pnl_rel_to_own_market_cap
                .as_mut()
                && let Some(supply_dollars_dateindex) = supply
                    .indexes_to_supply
                    .dollars
                    .as_ref()
                    .and_then(|d| d.dateindex.as_ref())
                && let Some(net_pnl_dateindex) =
                    unrealized.indexes_to_net_unrealized_pnl.dateindex.as_ref()
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        net_pnl_dateindex,
                        supply_dollars_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }

            // === Unrealized vs Own Total Unrealized PnL ===
            if let Some(v) = self
                .height_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                .as_mut()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_unrealized_profit,
                    &unrealized.height_to_total_unrealized_pnl,
                    exit,
                )?;
            }
            if let Some(v) = self
                .height_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                .as_mut()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_unrealized_loss,
                    &unrealized.height_to_total_unrealized_pnl,
                    exit,
                )?;
            }
            if let Some(v) = self
                .height_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                .as_mut()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_neg_unrealized_loss,
                    &unrealized.height_to_total_unrealized_pnl,
                    exit,
                )?;
            }
            if let Some(v) = self
                .height_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                .as_mut()
            {
                v.compute_percentage(
                    starting_indexes.height,
                    &unrealized.height_to_net_unrealized_pnl,
                    &unrealized.height_to_total_unrealized_pnl,
                    exit,
                )?;
            }

            // indexes versions for own total unrealized pnl
            if let Some(v) = self
                .indexes_to_unrealized_profit_rel_to_own_total_unrealized_pnl
                .as_mut()
                && let Some(total_pnl_dateindex) = unrealized
                    .indexes_to_total_unrealized_pnl
                    .dateindex
                    .as_ref()
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &unrealized.dateindex_to_unrealized_profit,
                        total_pnl_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }
            if let Some(v) = self
                .indexes_to_unrealized_loss_rel_to_own_total_unrealized_pnl
                .as_mut()
                && let Some(total_pnl_dateindex) = unrealized
                    .indexes_to_total_unrealized_pnl
                    .dateindex
                    .as_ref()
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        &unrealized.dateindex_to_unrealized_loss,
                        total_pnl_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = self
                .indexes_to_neg_unrealized_loss_rel_to_own_total_unrealized_pnl
                .as_mut()
                && let Some(total_pnl_dateindex) = unrealized
                    .indexes_to_total_unrealized_pnl
                    .dateindex
                    .as_ref()
                && let Some(neg_loss_dateindex) =
                    unrealized.indexes_to_neg_unrealized_loss.dateindex.as_ref()
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        neg_loss_dateindex,
                        total_pnl_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = self
                .indexes_to_net_unrealized_pnl_rel_to_own_total_unrealized_pnl
                .as_mut()
                && let Some(total_pnl_dateindex) = unrealized
                    .indexes_to_total_unrealized_pnl
                    .dateindex
                    .as_ref()
                && let Some(net_pnl_dateindex) =
                    unrealized.indexes_to_net_unrealized_pnl.dateindex.as_ref()
            {
                v.compute_all(starting_indexes, exit, |vec| {
                    vec.compute_percentage(
                        starting_indexes.dateindex,
                        net_pnl_dateindex,
                        total_pnl_dateindex,
                        exit,
                    )?;
                    Ok(())
                })?;
            }
        }

        Ok(())
    }
}
