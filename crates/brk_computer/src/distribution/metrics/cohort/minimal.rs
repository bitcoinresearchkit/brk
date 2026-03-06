use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{
    BasisPoints16, BasisPoints32, Bitcoin, Cents, Dollars, Height, Indexes, Sats, StoredF32,
    Version,
};
use vecdb::{AnyStoredVec, AnyVec, Exit, ReadableCloneableVec, ReadableVec, Rw, StorageMode, WritableVec};

use crate::{blocks, prices};

use crate::internal::{
    CentsUnsignedToDollars, ComputedFromHeight, ComputedFromHeightCumulative,
    ComputedFromHeightRatio, Identity, LazyFromHeight, PercentFromHeight, Price, RatioSatsBp16,
    ValueFromHeight,
};

use crate::distribution::{
    metrics::{ActivityCore, ImportConfig, OutputsMetrics, SupplyMetrics},
    state::{RealizedOps, UnrealizedState},
};

/// Minimal realized metrics: realized cap, realized price, MVRV, and realized P/L.
#[derive(Traversable)]
pub struct MinimalRealized<M: StorageMode = Rw> {
    pub realized_cap_cents: ComputedFromHeight<Cents, M>,
    pub realized_profit: ComputedFromHeightCumulative<Cents, M>,
    pub realized_loss: ComputedFromHeightCumulative<Cents, M>,
    pub realized_cap: LazyFromHeight<Dollars, Cents>,
    pub realized_price: Price<ComputedFromHeight<Cents, M>>,
    pub realized_price_ratio: ComputedFromHeightRatio<M>,
    pub mvrv: LazyFromHeight<StoredF32>,
}

/// Minimal unrealized metrics: supply in profit/loss only.
#[derive(Traversable)]
pub struct MinimalUnrealized<M: StorageMode = Rw> {
    pub supply_in_profit: ValueFromHeight<M>,
    pub supply_in_loss: ValueFromHeight<M>,
}

/// Minimal relative metrics: supply in profit/loss relative to own supply.
#[derive(Traversable)]
pub struct MinimalRelative<M: StorageMode = Rw> {
    pub supply_in_profit_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
    pub supply_in_loss_rel_to_own_supply: PercentFromHeight<BasisPoints16, M>,
}

/// MinimalCohortMetrics: supply, outputs, sent+ema, realized cap/price/mvrv/profit/loss,
/// supply in profit/loss, relative to own supply.
///
/// Used for type_, amount, and address cohorts.
/// Does NOT implement CohortMetricsBase — standalone, not aggregatable via trait.
#[derive(Traversable)]
pub struct MinimalCohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,
    pub supply: Box<SupplyMetrics<M>>,
    pub outputs: Box<OutputsMetrics<M>>,
    pub activity: Box<ActivityCore<M>>,
    pub realized: Box<MinimalRealized<M>>,
    pub unrealized: Box<MinimalUnrealized<M>>,
    pub relative: Box<MinimalRelative<M>>,
}

impl MinimalRealized {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let realized_cap_cents = cfg.import_computed("realized_cap_cents", Version::ZERO)?;
        let realized_cap = LazyFromHeight::from_computed::<CentsUnsignedToDollars>(
            &cfg.name("realized_cap"),
            cfg.version,
            realized_cap_cents.height.read_only_boxed_clone(),
            &realized_cap_cents,
        );

        let realized_profit = cfg.import_cumulative("realized_profit", Version::ZERO)?;
        let realized_loss = cfg.import_cumulative("realized_loss", Version::ZERO)?;

        let realized_price = cfg.import_price("realized_price", Version::ONE)?;
        let realized_price_ratio = cfg.import_ratio("realized_price", Version::ONE)?;
        let mvrv = LazyFromHeight::from_lazy::<Identity<StoredF32>, BasisPoints32>(
            &cfg.name("mvrv"),
            cfg.version,
            &realized_price_ratio.ratio,
        );

        Ok(Self {
            realized_cap_cents,
            realized_profit,
            realized_loss,
            realized_cap,
            realized_price,
            realized_price_ratio,
            mvrv,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.realized_cap_cents
            .height
            .len()
            .min(self.realized_profit.height.len())
            .min(self.realized_loss.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &impl RealizedOps,
    ) -> Result<()> {
        self.realized_cap_cents
            .height
            .truncate_push(height, state.cap())?;
        self.realized_profit
            .height
            .truncate_push(height, state.profit())?;
        self.realized_loss
            .height
            .truncate_push(height, state.loss())?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.realized_cap_cents.height as &mut dyn AnyStoredVec,
            &mut self.realized_profit.height,
            &mut self.realized_loss.height,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; realized_cap_cents.height);
        sum_others!(self, starting_indexes, others, exit; realized_profit.height);
        sum_others!(self, starting_indexes, others, exit; realized_loss.height);
        Ok(())
    }

    pub(crate) fn compute_rest_part1(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_profit
            .compute_rest(starting_indexes.height, exit)?;
        self.realized_loss
            .compute_rest(starting_indexes.height, exit)?;
        Ok(())
    }

    pub(crate) fn compute_rest_part2(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        height_to_supply: &impl ReadableVec<Height, Bitcoin>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized_price.cents.height.compute_transform2(
            starting_indexes.height,
            &self.realized_cap_cents.height,
            height_to_supply,
            |(i, cap_cents, supply, ..)| {
                let cap = cap_cents.as_u128();
                let supply_sats = Sats::from(supply).as_u128();
                if supply_sats == 0 {
                    (i, Cents::ZERO)
                } else {
                    (i, Cents::from(cap * Sats::ONE_BTC_U128 / supply_sats))
                }
            },
            exit,
        )?;

        self.realized_price_ratio.compute_ratio(
            starting_indexes,
            &prices.price.cents.height,
            &self.realized_price.cents.height,
            exit,
        )?;

        Ok(())
    }
}

impl MinimalUnrealized {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            supply_in_profit: cfg.import_value("supply_in_profit", Version::ZERO)?,
            supply_in_loss: cfg.import_value("supply_in_loss", Version::ZERO)?,
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply_in_profit
            .sats
            .height
            .len()
            .min(self.supply_in_loss.sats.height.len())
    }

    pub(crate) fn truncate_push(
        &mut self,
        height: Height,
        state: &UnrealizedState,
    ) -> Result<()> {
        self.supply_in_profit
            .sats
            .height
            .truncate_push(height, state.supply_in_profit)?;
        self.supply_in_loss
            .sats
            .height
            .truncate_push(height, state.supply_in_loss)?;
        Ok(())
    }

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply_in_profit.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_profit.base.cents.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply_in_loss.base.cents.height as &mut dyn AnyStoredVec,
        ]
    }

    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        sum_others!(self, starting_indexes, others, exit; supply_in_profit.base.sats.height);
        sum_others!(self, starting_indexes, others, exit; supply_in_loss.base.sats.height);
        Ok(())
    }

    pub(crate) fn compute_rest(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit.compute(prices, max_from, exit)?;
        self.supply_in_loss.compute(prices, max_from, exit)?;
        Ok(())
    }
}

impl MinimalRelative {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            supply_in_profit_rel_to_own_supply: cfg
                .import_percent_bp16("supply_in_profit_rel_to_own_supply", Version::ONE)?,
            supply_in_loss_rel_to_own_supply: cfg
                .import_percent_bp16("supply_in_loss_rel_to_own_supply", Version::ONE)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        max_from: Height,
        supply_in_profit_sats: &impl ReadableVec<Height, Sats>,
        supply_in_loss_sats: &impl ReadableVec<Height, Sats>,
        supply_total_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_in_profit_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                supply_in_profit_sats,
                supply_total_sats,
                exit,
            )?;
        self.supply_in_loss_rel_to_own_supply
            .compute_binary::<Sats, Sats, RatioSatsBp16>(
                max_from,
                supply_in_loss_sats,
                supply_total_sats,
                exit,
            )?;
        Ok(())
    }
}

impl MinimalCohortMetrics {
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(SupplyMetrics::forced_import(cfg)?),
            outputs: Box::new(OutputsMetrics::forced_import(cfg)?),
            activity: Box::new(ActivityCore::forced_import(cfg)?),
            realized: Box::new(MinimalRealized::forced_import(cfg)?),
            unrealized: Box::new(MinimalUnrealized::forced_import(cfg)?),
            relative: Box::new(MinimalRelative::forced_import(cfg)?),
        })
    }

    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_height_len())
            .min(self.unrealized.min_stateful_height_len())
    }

    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        Ok(())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();
        vecs.extend(self.supply.collect_vecs_mut());
        vecs.extend(self.outputs.collect_vecs_mut());
        vecs.extend(self.activity.collect_vecs_mut());
        vecs.extend(self.realized.collect_vecs_mut());
        vecs.extend(self.unrealized.collect_vecs_mut());
        vecs
    }

    /// Aggregate Minimal-tier metrics from other MinimalCohortMetrics sources.
    pub(crate) fn compute_from_sources(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&MinimalCohortMetrics],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.supply.as_ref()).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.outputs.as_ref()).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.activity.as_ref()).collect::<Vec<_>>(),
            exit,
        )?;
        self.realized.compute_from_sources(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.realized.as_ref())
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized.compute_from_sources(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.unrealized.as_ref())
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
        self.supply
            .compute(prices, starting_indexes.height, exit)?;
        self.supply
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.outputs
            .compute_rest(blocks, starting_indexes, exit)?;
        self.activity
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;
        self.realized
            .compute_rest_part1(starting_indexes, exit)?;
        self.unrealized
            .compute_rest(prices, starting_indexes.height, exit)?;
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

        self.relative.compute(
            starting_indexes.height,
            &self.unrealized.supply_in_profit.sats.height,
            &self.unrealized.supply_in_loss.sats.height,
            &self.supply.total.sats.height,
            exit,
        )?;

        Ok(())
    }
}
