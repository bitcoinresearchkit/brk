use std::ops::{Add, AddAssign};

use bitview_cohort::{
    ByTerm, Profitability, ProfitabilityId, ProfitabilityRange, UTXOAggregate, UTXOAggregateId,
};
use bitview_collections::Windows;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_traversable::Traversable;
use bitview_vecs::{
    LazyFiatPerBlock, LazyRatioPerBlock, LazySpotValuePerBlockWithDeltas, LazyWindowStartVec,
    StoredSeries, import_stored,
};
use brk_error::Result;
use brk_types::{Cents, CentsSats, Height, PartsPerMillionSigned32, Sats, Version};
use vecdb::{
    AnyStoredVec, AnyVec, CacheBudget, Database, PcoVecValue, ReadableBoxedVec, Rw, StorageMode,
    WritableVec,
};

const VERSION: Version = Version::new(9);

#[derive(Traversable)]
pub struct ProfitabilityVecs<M: StorageMode = Rw> {
    /// Unspent supply grouped by short- or long-term ownership and by the
    /// output's percentage profit or loss relative to spot price.
    pub supply: Profitability<UTXOAggregate<LazySpotValuePerBlockWithDeltas>>,
    /// Creation-date value of unspent supply grouped by short- or long-term
    /// ownership and by percentage profit or loss relative to spot price.
    pub realized_cap: Profitability<UTXOAggregate<LazyFiatPerBlock<Cents>>>,
    /// Absolute unrealized profit or loss of unspent supply grouped by short-
    /// or long-term ownership and by percentage profit or loss relative to
    /// spot price.
    pub unrealized_pnl: Profitability<UTXOAggregate<LazyFiatPerBlock<Cents>>>,
    /// Net unrealized profit and loss as a share of a profitability cohort's
    /// own market cap: spot price minus aggregate realized price, divided by
    /// spot price. Positive values place spot above that cohort's aggregate
    /// cost basis; negative values place it below. Returns zero when spot price
    /// or the cohort's unspent supply is zero.
    pub nupl: Profitability<LazyRatioPerBlock<PartsPerMillionSigned32>>,
    #[traversable(hidden)]
    supply_stored: Profitability<UTXOAggregate<StoredSeries<Height, Sats, M>>>,
    #[traversable(hidden)]
    realized_cap_stored: Profitability<UTXOAggregate<StoredSeries<Height, Cents, M>>>,
    #[traversable(hidden)]
    unrealized_pnl_stored: Profitability<UTXOAggregate<StoredSeries<Height, Cents, M>>>,
    #[traversable(hidden)]
    nupl_stored: Profitability<StoredSeries<Height, PartsPerMillionSigned32, M>>,
}

impl<M: StorageMode> ProfitabilityVecs<M> {
    pub fn min_resume_len(&self) -> usize {
        self.supply_stored
            .iter()
            .flat_map(|v| v.iter())
            .map(AnyVec::len)
            .chain(
                self.realized_cap_stored
                    .iter()
                    .flat_map(|v| v.iter())
                    .map(AnyVec::len),
            )
            .chain(
                self.unrealized_pnl_stored
                    .iter()
                    .flat_map(|v| v.iter())
                    .map(AnyVec::len),
            )
            .chain(self.nupl_stored.iter().map(AnyVec::len))
            .min()
            .unwrap_or_default()
    }
}

impl ProfitabilityVecs {
    pub fn forced_import(
        cache: &'static CacheBudget,
        db: &Database,
        version: Version,
        mappings: &MappingsVecs,
        window_starts: &Windows<&LazyWindowStartVec>,
        spot_price: &ReadableBoxedVec<Height, Cents>,
    ) -> Result<Box<Self>> {
        let version = version + VERSION;
        let supply_stored = Self::import_sources(cache, db, "supply_sats", version)?;
        let realized_cap_stored = Self::import_sources(cache, db, "realized_cap_cents", version)?;
        let unrealized_pnl_stored =
            Self::import_sources(cache, db, "unrealized_pnl_cents", version)?;
        let nupl_stored = ProfitabilityId::try_series(|_, name| {
            import_stored(cache, db, &format!("{name}_nupl_ppm"), version)
        })?;
        let supply = Self::series(&supply_stored, "supply", |name, source| {
            LazySpotValuePerBlockWithDeltas::from_sats_source(
                name,
                version,
                source,
                mappings,
                window_starts,
                spot_price,
            )
        });
        let realized_cap = Self::series(&realized_cap_stored, "realized_cap", |name, source| {
            LazyFiatPerBlock::from_cents_source(name, version, source, mappings)
        });
        let unrealized_pnl =
            Self::series(&unrealized_pnl_stored, "unrealized_pnl", |name, source| {
                LazyFiatPerBlock::from_cents_source(name, version, source, mappings)
            });
        let nupl = ProfitabilityId::series(|id, name| {
            LazyRatioPerBlock::from_height_source(
                &format!("{name}_nupl"),
                version,
                id.select(&nupl_stored),
                mappings,
            )
        });
        Ok(Box::new(Self {
            supply,
            realized_cap,
            unrealized_pnl,
            nupl,
            supply_stored,
            realized_cap_stored,
            unrealized_pnl_stored,
            nupl_stored,
        }))
    }

    fn import_sources<T: PcoVecValue>(
        cache: &'static CacheBudget,
        db: &Database,
        metric: &str,
        version: Version,
    ) -> Result<Profitability<UTXOAggregate<StoredSeries<Height, T>>>> {
        ProfitabilityId::try_series(|_, cohort| {
            UTXOAggregate::try_from_fn(|id| {
                import_stored(cache, db, &Self::metric_name(cohort, id, metric), version)
            })
        })
    }

    fn series<T: PcoVecValue, S>(
        sources: &Profitability<UTXOAggregate<StoredSeries<Height, T>>>,
        metric: &str,
        mut build: impl FnMut(&str, &StoredSeries<Height, T>) -> S,
    ) -> Profitability<UTXOAggregate<S>> {
        ProfitabilityId::series(|id, cohort| {
            UTXOAggregate::from_fn(|aggregate| {
                build(
                    &Self::metric_name(cohort, aggregate, metric),
                    aggregate.select(id.select(sources)),
                )
            })
        })
    }

    fn metric_name(cohort: &str, aggregate: UTXOAggregateId, metric: &str) -> String {
        match aggregate {
            UTXOAggregateId::All => format!("{cohort}_{metric}"),
            UTXOAggregateId::Sth | UTXOAggregateId::Lth => {
                format!("{cohort}_{}_{metric}", aggregate.cohort_name().id)
            }
        }
    }

    #[inline(always)]
    pub fn push(
        &mut self,
        spot: Cents,
        supply: ByTerm<ProfitabilityRange<Sats>>,
        realized_cap: ByTerm<ProfitabilityRange<Cents>>,
    ) {
        let all_supply = Self::sum_terms(&supply);
        let all_realized_cap = Self::sum_terms(&realized_cap);
        let unrealized_pnl = Self::unrealized_pnl_by_term(spot, &realized_cap, &supply);
        let nupl = Self::nupl(spot, &all_realized_cap, &all_supply);

        Self::push_sources(&mut self.supply_stored, supply);
        Self::push_sources(&mut self.realized_cap_stored, realized_cap);
        Self::push_sources(&mut self.unrealized_pnl_stored, unrealized_pnl);
        for (target, &value) in self.nupl_stored.iter_mut().zip(nupl.iter()) {
            target.push(value);
        }
    }

    fn push_sources<T: PcoVecValue + Copy + Default + AddAssign>(
        targets: &mut Profitability<UTXOAggregate<StoredSeries<Height, T>>>,
        values: ByTerm<ProfitabilityRange<T>>,
    ) {
        let short = Profitability::from_ranges(values.short);
        let long = Profitability::from_ranges(values.long);
        for ((target, &short), &long) in targets.iter_mut().zip(short.iter()).zip(long.iter()) {
            let mut all = short;
            all += long;
            target.all.push(all);
            target.sth.push(short);
            target.lth.push(long);
        }
    }

    pub fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        self.supply_stored
            .iter_mut()
            .flat_map(|v| v.iter_mut())
            .map(|v| v as &mut dyn AnyStoredVec)
            .chain(
                self.realized_cap_stored
                    .iter_mut()
                    .flat_map(|v| v.iter_mut())
                    .map(|v| v as &mut dyn AnyStoredVec),
            )
            .chain(
                self.unrealized_pnl_stored
                    .iter_mut()
                    .flat_map(|v| v.iter_mut())
                    .map(|v| v as &mut dyn AnyStoredVec),
            )
            .chain(
                self.nupl_stored
                    .iter_mut()
                    .map(|v| v as &mut dyn AnyStoredVec),
            )
            .collect()
    }

    fn sum_terms<T>(cohort_values: &ByTerm<ProfitabilityRange<T>>) -> ProfitabilityRange<T>
    where
        T: Add<Output = T> + Copy,
    {
        ProfitabilityRange::from_fn(|range| {
            *range.select(&cohort_values.short) + *range.select(&cohort_values.long)
        })
    }

    fn unrealized_pnl_by_term(
        spot: Cents,
        cap: &ByTerm<ProfitabilityRange<Cents>>,
        supply: &ByTerm<ProfitabilityRange<Sats>>,
    ) -> ByTerm<ProfitabilityRange<Cents>> {
        ByTerm {
            short: Self::unrealized_pnl(spot, &cap.short, &supply.short),
            long: Self::unrealized_pnl(spot, &cap.long, &supply.long),
        }
    }

    fn unrealized_pnl(
        spot: Cents,
        cap: &ProfitabilityRange<Cents>,
        supply: &ProfitabilityRange<Sats>,
    ) -> ProfitabilityRange<Cents> {
        ProfitabilityRange::from_fn(|id| {
            let market_value =
                CentsSats::from_price_sats(spot, *id.select(supply)).to_cents_rounded();
            let realized_cap = *id.select(cap);
            if id.is_profit() {
                market_value.saturating_sub(realized_cap)
            } else {
                realized_cap.saturating_sub(market_value)
            }
        })
    }

    fn nupl(
        spot: Cents,
        cap: &ProfitabilityRange<Cents>,
        supply: &ProfitabilityRange<Sats>,
    ) -> Profitability<PartsPerMillionSigned32> {
        let cap = Profitability::from_ranges(cap.clone());
        let supply = Profitability::from_ranges(supply.clone());
        ProfitabilityId::series(|id, _| {
            let spot = spot.as_u128();
            let supply = id.select(&supply).as_u128();
            if spot == 0 || supply == 0 {
                PartsPerMillionSigned32::ZERO
            } else {
                let realized_price = id.select(&cap).as_u128() * Sats::ONE_BTC_U128 / supply;
                PartsPerMillionSigned32::from((spot as f64 - realized_price as f64) / spot as f64)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use bitview_cohort::{
        ByTerm, PROFIT_COUNT, Profitability, ProfitabilityId, ProfitabilityRange,
        ProfitabilityRangeId,
    };
    use brk_types::{Cents, PartsPerMillionSigned32, Sats};

    use super::ProfitabilityVecs;

    #[test]
    fn expanded_thresholds_match_prefix_and_suffix_sums() {
        let ranges = ProfitabilityRange::from_fn(|id| Sats::from(id.index() as u64 + 1));
        let values = Profitability::from_ranges(ranges.clone());
        let sum = |values: &[Sats]| {
            values
                .iter()
                .copied()
                .fold(Sats::default(), |total, value| total + value)
        };

        let ranges: Vec<_> = ranges.iter().copied().collect();
        for (threshold, &id) in ProfitabilityId::profit_ids().iter().enumerate() {
            assert_eq!(
                *id.select(&values),
                sum(&ranges[..PROFIT_COUNT + 1 - threshold])
            );
        }
        for (threshold, &id) in ProfitabilityId::loss_ids().iter().enumerate() {
            assert_eq!(
                *id.select(&values),
                sum(&ranges[PROFIT_COUNT + 1 + threshold..])
            );
        }
    }

    #[test]
    fn derived_values_preserve_profit_and_loss_polarity() {
        let supply = ProfitabilityRange::from_fn(|_| Sats::ONE_BTC);
        let cap = ProfitabilityRange::from_fn(|id| {
            Cents::from(if id.is_profit() { 100_u64 } else { 300_u64 })
        });
        let spot = Cents::from(200_u64);

        let cap = ByTerm {
            short: cap.clone(),
            long: cap,
        };
        let supply = ByTerm {
            short: supply.clone(),
            long: supply,
        };
        let pnl = ProfitabilityVecs::unrealized_pnl_by_term(spot, &cap, &supply);
        let all_cap = ProfitabilityVecs::sum_terms(&cap);
        let all_supply = ProfitabilityVecs::sum_terms(&supply);
        let nupl = ProfitabilityVecs::nupl(spot, &all_cap, &all_supply);

        for id in ProfitabilityRangeId::ALL {
            assert_eq!(*id.select(&pnl.short), Cents::from(100_u64));
            assert_eq!(*id.select(&pnl.long), Cents::from(100_u64));
        }
        for id in ProfitabilityRangeId::ALL {
            assert_eq!(
                *id.select(&nupl.range),
                PartsPerMillionSigned32::from(if id.is_profit() { 0.5 } else { -0.5 })
            );
        }
        for value in nupl.profit.iter() {
            assert_eq!(*value, PartsPerMillionSigned32::from(0.5));
        }
        for value in nupl.loss.iter() {
            assert_eq!(*value, PartsPerMillionSigned32::from(-0.5));
        }
    }
}
