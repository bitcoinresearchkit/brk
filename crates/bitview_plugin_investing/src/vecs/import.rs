use bitview_collections::{ByDcaCagr, ByDcaPeriod};
use bitview_plugin::ImportContext;
use bitview_plugin_blocks::Vecs as BlocksVecs;
use bitview_plugin_mappings::Vecs as MappingsVecs;
use bitview_plugin_price::Vecs as PriceVecs;
use bitview_transforms::RatioDiffCents;
use bitview_vecs::{
    LazyIndexedVec, LazyPercentPerBlock, LazyPreviousDeltaVec, LazySinceDayVec, LazyWindowVec,
    Price, import_cached,
};
use brk_error::{Error, Result};
use brk_types::{Cents, Date, Day1, Height, PartsPerMillionSigned64, Sats};
use vecdb::{BinaryTransform, CheckedSub, ReadableCloneableVec, VecIndex};

use super::Vecs;
use crate::{
    ByDcaClass, STORAGE, class_vecs::ClassVecs, dca_sats::DcaSats, dca_stack::DcaStack,
    lump_sum_stack::LumpSumStack, period_vecs::PeriodVecs,
};

impl Vecs {
    pub fn import(
        context: ImportContext<'_>,
        mappings: &MappingsVecs,
        blocks: &BlocksVecs,
        prices: &PriceVecs,
    ) -> Result<Self> {
        let db = STORAGE.open_database(context, 256)?;
        let version = STORAGE.schema_version();
        let sats_cumulative = import_cached(&db, "dca_sats_cumulative", version)?;

        let height_days = mappings.height_day1.clone();
        let dca_sats = DcaSats::new(sats_cumulative.read_only_boxed_clone(), height_days.clone());
        let height_sats = dca_sats.read_only_boxed_clone();
        let sats_per_day = LazyPreviousDeltaVec::new("dca_sats_per_day", version, &height_sats);

        let window_starts = ByDcaPeriod::try_new(|_, days| {
            Ok::<_, Error>(blocks.lookback.start_vec(days as usize))
        })?;
        let spot_price = prices.spot.cents.resolutions.height_source();

        let dca_stack =
            ByDcaPeriod::try_from_period(&window_starts, |name, _days, window_starts| {
                let metric_name = format!("dca_stack_{name}");
                let source = LazyWindowVec::<Height, Sats, Sats>::new(
                    &format!("{metric_name}_sats_source"),
                    version,
                    &height_sats,
                    &(**window_starts),
                    true,
                    |current, before, _| current.checked_sub(before).unwrap_or_default(),
                );
                DcaStack::from_source(&metric_name, version, mappings, &source, spot_price)
            })?;

        let first_price_day = Day1::try_from(Date::new(2010, 7, 12)).unwrap();
        let dca_cost_basis = ByDcaPeriod::try_from_period(&dca_stack, |name, days, stack| {
            let metric_name = format!("dca_cost_basis_{name}");
            let source = LazyIndexedVec::new(
                &format!("{metric_name}_cents_source"),
                version,
                &stack.sats.height,
                &height_days,
                move |_, stack_sats, day| {
                    if day <= first_price_day {
                        return Cents::ZERO;
                    }
                    let num_days =
                        (days as usize).min(day.to_usize() + 1 - first_price_day.to_usize());
                    DcaStack::cost_basis_cents(num_days, stack_sats)
                },
            );
            Ok::<_, Error>(Price::from_height_source(
                &metric_name,
                version,
                &source,
                mappings,
            ))
        })?;

        let dca_return =
            ByDcaPeriod::try_from_period(&dca_cost_basis, |name, _days, cost_basis| {
                let metric_name = format!("dca_return_{name}");
                let source = LazyIndexedVec::new(
                    &format!("{metric_name}_ppm_source"),
                    version,
                    &cost_basis.cents.height,
                    spot_price,
                    |_, cost_basis, spot| {
                        RatioDiffCents::<PartsPerMillionSigned64>::apply(spot, cost_basis)
                    },
                );
                Ok::<_, Error>(LazyPercentPerBlock::from_height_source(
                    &metric_name,
                    version,
                    &source,
                    mappings,
                ))
            })?;

        let dca_cagr = ByDcaCagr::try_new(&dca_return, |name, days, source| {
            Ok::<_, Error>(LazyPercentPerBlock::from_lazy_cagr(
                &format!("dca_cagr_{name}"),
                version,
                (days / 365) as u8,
                source,
            ))
        })?;

        let lump_sum_stack =
            ByDcaPeriod::try_from_period(&window_starts, |name, days, window_starts| {
                LumpSumStack::from_window(
                    &format!("lump_sum_stack_{name}"),
                    days,
                    version,
                    mappings,
                    *window_starts,
                    prices,
                )
            })?;

        let lump_sum_return =
            ByDcaPeriod::try_from_period(&window_starts, |name, _days, window_starts| {
                let metric_name = format!("lump_sum_return_{name}");
                let source = LazyWindowVec::<Height, Cents, PartsPerMillionSigned64>::new(
                    &format!("{metric_name}_ppm_source"),
                    version,
                    &prices.spot.cents.height,
                    &(**window_starts),
                    false,
                    |current, past, _| {
                        RatioDiffCents::<PartsPerMillionSigned64>::apply(current, past)
                    },
                );
                Ok::<_, Error>(LazyPercentPerBlock::from_height_source(
                    &metric_name,
                    version,
                    &source,
                    mappings,
                ))
            })?;

        let class_stack = ByDcaClass::try_new(|name, _year, day| {
            let metric_name = format!("dca_stack_{name}");
            let source = LazySinceDayVec::new(
                &format!("{metric_name}_sats_source"),
                version,
                &height_sats,
                &mappings.first_height.day1,
                day,
                |current, before| current.checked_sub(before).unwrap_or_default(),
            );
            DcaStack::from_source(&metric_name, version, mappings, &source, spot_price)
        })?;

        let class_cost_basis =
            ByDcaClass::try_from_class(&class_stack, |name, _year, from, stack| {
                let metric_name = format!("dca_cost_basis_{name}");
                let source = LazyIndexedVec::new(
                    &format!("{metric_name}_cents_source"),
                    version,
                    &stack.sats.height,
                    &height_days,
                    move |_, stack_sats, day| {
                        if day < from {
                            return Cents::ZERO;
                        }
                        let num_days = day.to_usize() + 1 - from.to_usize();
                        DcaStack::cost_basis_cents(num_days, stack_sats)
                    },
                );
                Ok::<_, Error>(Price::from_height_source(
                    &metric_name,
                    version,
                    &source,
                    mappings,
                ))
            })?;

        let class_return =
            ByDcaClass::try_from_class(&class_cost_basis, |name, _year, _from, cost_basis| {
                let metric_name = format!("dca_return_{name}");
                let source = LazyIndexedVec::new(
                    &format!("{metric_name}_ppm_source"),
                    version,
                    &cost_basis.cents.height,
                    spot_price,
                    |_, cost_basis, spot| {
                        RatioDiffCents::<PartsPerMillionSigned64>::apply(spot, cost_basis)
                    },
                );
                Ok::<_, Error>(LazyPercentPerBlock::from_height_source(
                    &metric_name,
                    version,
                    &source,
                    mappings,
                ))
            })?;

        let this = Self {
            db,
            sats_cumulative,
            sats_per_day,
            period: PeriodVecs {
                dca_stack,
                dca_cost_basis,
                dca_return,
                dca_cagr,
                lump_sum_stack,
                lump_sum_return,
            },
            class: ClassVecs {
                dca_stack: class_stack,
                dca_cost_basis: class_cost_basis,
                dca_return: class_return,
            },
        };
        STORAGE.finalize_database(&this.db)?;
        Ok(this)
    }
}
