use brk_cohort::{Loss, Profit, ProfitabilityRange};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned32, Bitcoin, Cents, Dollars, Indexes, Sats, Version};
use vecdb::{AnyStoredVec, AnyVec, Database, Exit, Rw, StorageMode, WritableVec};

use crate::{
    indexes,
    internal::{
        ValuePerBlock, ValuePerBlockWithDeltas, PerBlock, RatioPerBlock, WindowStartVec, Windows,
    },
    prices,
};

#[derive(Traversable)]
pub struct WithSth<All, Sth = All> {
    pub all: All,
    pub sth: Sth,
}

#[derive(Traversable)]
pub struct ProfitabilityBucket<M: StorageMode = Rw> {
    pub supply: WithSth<ValuePerBlockWithDeltas<M>, ValuePerBlock<M>>,
    pub realized_cap: WithSth<PerBlock<Dollars, M>>,
    pub unrealized_pnl: WithSth<PerBlock<Dollars, M>>,
    pub nupl: RatioPerBlock<BasisPointsSigned32, M>,
}

impl<M: StorageMode> ProfitabilityBucket<M> {
    fn min_len(&self) -> usize {
        self.supply
            .all
            .sats
            .height
            .len()
            .min(self.realized_cap.all.height.len())
    }
}

impl ProfitabilityBucket {
    fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        Ok(Self {
            supply: WithSth {
                all: ValuePerBlockWithDeltas::forced_import(
                    db,
                    &format!("{name}_supply"),
                    version,
                    indexes,
                    cached_starts,
                )?,
                sth: ValuePerBlock::forced_import(
                    db,
                    &format!("{name}_sth_supply"),
                    version,
                    indexes,
                )?,
            },
            realized_cap: WithSth {
                all: PerBlock::forced_import(
                    db,
                    &format!("{name}_realized_cap"),
                    version,
                    indexes,
                )?,
                sth: PerBlock::forced_import(
                    db,
                    &format!("{name}_sth_realized_cap"),
                    version,
                    indexes,
                )?,
            },
            unrealized_pnl: WithSth {
                all: PerBlock::forced_import(
                    db,
                    &format!("{name}_unrealized_pnl"),
                    version,
                    indexes,
                )?,
                sth: PerBlock::forced_import(
                    db,
                    &format!("{name}_sth_unrealized_pnl"),
                    version,
                    indexes,
                )?,
            },
            nupl: RatioPerBlock::forced_import_raw(
                db,
                &format!("{name}_nupl"),
                version + Version::ONE,
                indexes,
            )?,
        })
    }

    #[inline(always)]
    pub(crate) fn push(
        &mut self,
        supply: Sats,
        sth_supply: Sats,
        realized_cap: Dollars,
        sth_realized_cap: Dollars,
    ) {
        self.supply.all.sats.height.push(supply);
        self.supply.sth.sats.height.push(sth_supply);
        self.realized_cap.all.height.push(realized_cap);
        self.realized_cap.sth.height.push(sth_realized_cap);
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        is_profit: bool,
        exit: &Exit,
    ) -> Result<()> {
        let max_from = starting_indexes.height;

        self.supply.all.compute(prices, max_from, exit)?;
        self.supply.sth.compute(prices, max_from, exit)?;

        self.unrealized_pnl.all.height.compute_transform3(
            max_from,
            &prices.spot.cents.height,
            &self.realized_cap.all.height,
            &self.supply.all.sats.height,
            |(i, spot, cap, supply, ..)| {
                let mv = f64::from(Dollars::from(spot)) * f64::from(Bitcoin::from(supply));
                let rc = f64::from(cap);
                let pnl = if is_profit { mv - rc } else { rc - mv }.max(0.0);
                (i, Dollars::from(pnl))
            },
            exit,
        )?;
        self.unrealized_pnl.sth.height.compute_transform3(
            max_from,
            &prices.spot.cents.height,
            &self.realized_cap.sth.height,
            &self.supply.sth.sats.height,
            |(i, spot, cap, supply, ..)| {
                let mv = f64::from(Dollars::from(spot)) * f64::from(Bitcoin::from(supply));
                let rc = f64::from(cap);
                let pnl = if is_profit { mv - rc } else { rc - mv }.max(0.0);
                (i, Dollars::from(pnl))
            },
            exit,
        )?;

        self.nupl.bps.height.compute_transform3(
            max_from,
            &prices.spot.cents.height,
            &self.realized_cap.all.height,
            &self.supply.all.sats.height,
            |(i, spot, cap_dollars, supply_sats, ..)| {
                let p = spot.as_u128();
                let supply = supply_sats.as_u128();
                if p == 0 || supply == 0 {
                    (i, BasisPointsSigned32::ZERO)
                } else {
                    let rp = Cents::from(cap_dollars).as_u128() * Sats::ONE_BTC_U128 / supply;
                    let bps = ((p as i128 - rp as i128) * 10000) / p as i128;
                    (i, BasisPointsSigned32::from(bps as i32))
                }
            },
            exit,
        )?;

        Ok(())
    }

    pub(crate) fn compute_from_ranges(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        is_profit: bool,
        sources: &[&ProfitabilityBucket],
        exit: &Exit,
    ) -> Result<()> {
        let max_from = starting_indexes.height;

        self.supply.all.sats.height.compute_sum_of_others(
            max_from,
            &sources
                .iter()
                .map(|s| &s.supply.all.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.supply.sth.sats.height.compute_sum_of_others(
            max_from,
            &sources
                .iter()
                .map(|s| &s.supply.sth.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_cap.all.height.compute_sum_of_others(
            max_from,
            &sources
                .iter()
                .map(|s| &s.realized_cap.all.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_cap.sth.height.compute_sum_of_others(
            max_from,
            &sources
                .iter()
                .map(|s| &s.realized_cap.sth.height)
                .collect::<Vec<_>>(),
            exit,
        )?;

        self.compute(prices, starting_indexes, is_profit, exit)
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.supply.all.inner.sats.height as &mut dyn AnyStoredVec,
            &mut self.supply.all.inner.cents.height,
            &mut self.supply.sth.sats.height,
            &mut self.supply.sth.cents.height,
            &mut self.realized_cap.all.height,
            &mut self.realized_cap.sth.height,
            &mut self.unrealized_pnl.all.height,
            &mut self.unrealized_pnl.sth.height,
            &mut self.nupl.bps.height,
        ]
    }
}

/// All profitability metrics: 25 ranges + 14 profit thresholds + 9 loss thresholds.
#[derive(Traversable)]
pub struct ProfitabilityMetrics<M: StorageMode = Rw> {
    pub range: ProfitabilityRange<ProfitabilityBucket<M>>,
    pub profit: Profit<ProfitabilityBucket<M>>,
    pub loss: Loss<ProfitabilityBucket<M>>,
}

impl<M: StorageMode> ProfitabilityMetrics<M> {
    pub fn iter(&self) -> impl Iterator<Item = &ProfitabilityBucket<M>> {
        self.range
            .iter()
            .chain(self.profit.iter())
            .chain(self.loss.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ProfitabilityBucket<M>> {
        self.range
            .iter_mut()
            .chain(self.profit.iter_mut())
            .chain(self.loss.iter_mut())
    }

    pub(crate) fn min_stateful_len(&self) -> usize {
        self.range.iter().map(|b| b.min_len()).min().unwrap_or(0)
    }
}

impl ProfitabilityMetrics {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let range = ProfitabilityRange::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, version, indexes, cached_starts)
        })?;

        let aggregate_version = version + Version::TWO;

        let profit = Profit::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, aggregate_version, indexes, cached_starts)
        })?;

        let loss = Loss::try_new(|name| {
            ProfitabilityBucket::forced_import(db, name, aggregate_version, indexes, cached_starts)
        })?;

        Ok(Self {
            range,
            profit,
            loss,
        })
    }

    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        for (is_profit, bucket) in self.range.iter_mut_with_is_profit() {
            bucket.compute(prices, starting_indexes, is_profit, exit)?;
        }

        let range_arr = self.range.as_array();

        for (threshold, sources) in self.profit.iter_mut_with_growing_prefix(&range_arr) {
            threshold.compute_from_ranges(prices, starting_indexes, true, sources, exit)?;
        }
        for (threshold, sources) in self.loss.iter_mut_with_growing_suffix(&range_arr) {
            threshold.compute_from_ranges(prices, starting_indexes, false, sources, exit)?;
        }

        Ok(())
    }

    pub(crate) fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        let mut vecs = Vec::new();
        for bucket in self.iter_mut() {
            vecs.extend(bucket.collect_all_vecs_mut());
        }
        vecs
    }
}
