use brk_error::Result;
use brk_types::{Bitcoin, Day1, Date, Dollars, Height, Sats, StoredF32, StoredU32};
use vecdb::{AnyVec, EagerVec, Exit, ReadableOptionVec, ReadableVec, PcoVec, PcoVecValue, VecIndex};

use super::{ByDcaClass, ByDcaPeriod, Vecs};
use crate::{
    ComputeIndexes, blocks, indexes,
    internal::{ComputedFromHeightLast, PercentageDiffDollars},
    market::lookback,
    prices,
};

const DCA_AMOUNT: Dollars = Dollars::mint(100.0);

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        lookback: &lookback::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let h2d = &indexes.height.day1;
        let close = &prices.usd.close.day1;

        let first_price_di = Day1::try_from(Date::new(2010, 7, 12))
            .unwrap()
            .to_usize();

        // Compute per-height DCA sats contribution once (reused by all periods).
        // Value = sats_from_dca(close_price) on day-boundary blocks, Sats::ZERO otherwise.
        {
            let mut last_di: Option<Day1> = None;
            self.dca_sats_per_day.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    let same_day = last_di.is_some_and(|prev| prev == di);
                    last_di = Some(di);
                    if same_day {
                        (h, Sats::ZERO)
                    } else {
                        let s = close.collect_one_flat(di).map(sats_from_dca).unwrap_or(Sats::ZERO);
                        (h, s)
                    }
                },
                exit,
            )?;
        }

        // DCA by period - stack (rolling sum via _start vecs)
        for (stack, days) in self.period_stack.iter_mut_with_days() {
            let window_starts = blocks.count.start_vec(days as usize);
            stack.sats.height.compute_rolling_sum(
                starting_indexes.height,
                window_starts,
                &self.dca_sats_per_day,
                exit,
            )?;
        }

        // DCA by period - average price (derived from stack)
        let sh = starting_indexes.height.to_usize();
        for (average_price, stack, days) in self
            .period_average_price
            .zip_mut_with_days(&self.period_stack)
        {
            let days = days as usize;
            let stack_data = stack.sats.height.collect_range_at(sh, stack.sats.height.len());
            average_price.usd.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    let di_usize = di.to_usize();
                    let stack_sats = stack_data[h.to_usize() - sh];
                    let avg = if di_usize > first_price_di {
                        let num_days = days
                            .min(di_usize + 1)
                            .min(di_usize + 1 - first_price_di);
                        DCA_AMOUNT * num_days / Bitcoin::from(stack_sats)
                    } else {
                        Dollars::NAN
                    };
                    (h, avg)
                },
                exit,
            )?;
        }

        // DCA by period - returns (compute from average price)
        for (returns, (average_price, _)) in self
            .period_returns
            .iter_mut()
            .zip(self.period_average_price.iter_with_days())
        {
            returns.compute_binary::<Dollars, Dollars, PercentageDiffDollars>(
                starting_indexes.height,
                &prices.usd.price,
                &average_price.usd.height,
                exit,
            )?;
        }

        // DCA by period - CAGR (computed from returns)
        for (cagr, returns, days) in self.period_cagr.zip_mut_with_period(&self.period_returns) {
            let years = days as f32 / 365.0;
            let returns_data: Vec<StoredF32> = returns.day1.collect_or_default();
            cagr.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    let v = returns_data.get(di.to_usize())
                        .map(|r| ((**r / 100.0 + 1.0).powf(1.0 / years) - 1.0) * 100.0)
                        .unwrap_or(0.0);
                    (h, StoredF32::from(v))
                },
                exit,
            )?;
        }

        // DCA by period - profitability
        compute_period_rolling(
            &mut self.period_days_in_profit,
            &mut self.period_days_in_loss,
            &mut self.period_min_return,
            &mut self.period_max_return,
            &self.period_returns,
            blocks,
            h2d,
            starting_indexes,
            exit,
        )?;

        // Lump sum by period - stack
        let lookback_dca = lookback.price_ago.as_dca_period();
        for (stack, lookback_price, days) in
            self.period_lump_sum_stack.zip_mut_with_days(&lookback_dca)
        {
            let total_invested = DCA_AMOUNT * days as usize;
            let lookback_data = lookback_price.usd.height.collect_range_at(sh, lookback_price.usd.height.len());
            stack.sats.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, _di, _)| {
                    let lp = lookback_data[h.to_usize() - sh];
                    let sats = if lp == Dollars::ZERO {
                        Sats::ZERO
                    } else {
                        Sats::from(Bitcoin::from(total_invested / lp))
                    };
                    (h, sats)
                },
                exit,
            )?;
        }

        // Lump sum by period - returns (compute from lookback price)
        let lookback_dca2 = lookback.price_ago.as_dca_period();
        for (returns, (lookback_price, _)) in self
            .period_lump_sum_returns
            .iter_mut()
            .zip(lookback_dca2.iter_with_days())
        {
            returns.compute_binary::<Dollars, Dollars, PercentageDiffDollars>(
                starting_indexes.height,
                &prices.usd.price,
                &lookback_price.usd.height,
                exit,
            )?;
        }

        // Lump sum by period - profitability
        compute_period_rolling(
            &mut self.period_lump_sum_days_in_profit,
            &mut self.period_lump_sum_days_in_loss,
            &mut self.period_lump_sum_min_return,
            &mut self.period_lump_sum_max_return,
            &self.period_lump_sum_returns,
            blocks,
            h2d,
            starting_indexes,
            exit,
        )?;

        // DCA by year class - stack (cumulative sum from class start date)
        let start_days = super::ByDcaClass::<()>::start_days();
        for (stack, day1) in self.class_stack.iter_mut().zip(start_days) {
            let mut last_di: Option<Day1> = None;

            stack.sats.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, this)| {
                    let hi = h.to_usize();

                    if last_di.is_none() && hi > 0 {
                        last_di = Some(h2d.collect_one_at(hi - 1).unwrap());
                    }

                    if di < day1 {
                        last_di = Some(di);
                        return (h, Sats::ZERO);
                    }

                    let prev_di = last_di;
                    last_di = Some(di);

                    let same_day = prev_di.is_some_and(|prev| prev == di);
                    if same_day {
                        (h, this.collect_one_at(hi - 1).unwrap_or_default())
                    } else {
                        let prev = if hi > 0 && prev_di.is_some_and(|pd| pd >= day1) {
                            this.collect_one_at(hi - 1).unwrap_or_default()
                        } else {
                            Sats::ZERO
                        };
                        let s = close.collect_one_flat(di).map(sats_from_dca).unwrap_or(Sats::ZERO);
                        (h, prev + s)
                    }
                },
                exit,
            )?;
        }

        // DCA by year class - average price (derived from stack)
        let start_days = super::ByDcaClass::<()>::start_days();
        for ((average_price, stack), from) in self
            .class_average_price
            .iter_mut()
            .zip(self.class_stack.iter())
            .zip(start_days)
        {
            let from_usize = from.to_usize();
            let stack_data = stack.sats.height.collect_range_at(sh, stack.sats.height.len());
            average_price.usd.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    let di_usize = di.to_usize();
                    if di_usize < from_usize {
                        return (h, Dollars::NAN);
                    }
                    let stack_sats = stack_data[h.to_usize() - sh];
                    let num_days = di_usize + 1 - from_usize;
                    let avg = DCA_AMOUNT * num_days / Bitcoin::from(stack_sats);
                    (h, avg)
                },
                exit,
            )?;
        }

        // DCA by year class - returns (compute from average price)
        for (returns, average_price) in self
            .class_returns
            .iter_mut()
            .zip(self.class_average_price.iter())
        {

            returns.compute_binary::<Dollars, Dollars, PercentageDiffDollars>(
                starting_indexes.height,
                &prices.usd.price,
                &average_price.usd.height,
                exit,
            )?;
        }

        // DCA by year class - profitability
        compute_class_cumulative(
            &mut self.class_days_in_profit,
            &mut self.class_days_in_loss,
            &mut self.class_min_return,
            &mut self.class_max_return,
            &self.class_returns,
            h2d,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }
}

fn sats_from_dca(price: Dollars) -> Sats {
    if price == Dollars::ZERO {
        Sats::ZERO
    } else {
        Sats::from(Bitcoin::from(DCA_AMOUNT / price))
    }
}

#[allow(clippy::too_many_arguments)]
fn compute_period_rolling(
    days_in_profit: &mut ByDcaPeriod<ComputedFromHeightLast<StoredU32>>,
    days_in_loss: &mut ByDcaPeriod<ComputedFromHeightLast<StoredU32>>,
    min_return: &mut ByDcaPeriod<ComputedFromHeightLast<StoredF32>>,
    max_return: &mut ByDcaPeriod<ComputedFromHeightLast<StoredF32>>,
    returns: &ByDcaPeriod<ComputedFromHeightLast<StoredF32>>,
    blocks: &blocks::Vecs,
    h2d: &EagerVec<PcoVec<Height, Day1>>,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    for ((((dip, dil), minr), maxr), (ret, days)) in days_in_profit
        .iter_mut()
        .zip(days_in_loss.iter_mut())
        .zip(min_return.iter_mut())
        .zip(max_return.iter_mut())
        .zip(returns.iter_with_days())
    {
        let window_starts = blocks.count.start_vec(days as usize);
        let returns_data: Vec<StoredF32> = ret.day1.collect_or_default();

        compute_rolling(
            &mut dip.height, h2d, &returns_data, window_starts, starting_indexes.height, exit,
            |buf| StoredU32::from(buf.iter().copied().filter(|r| **r > 0.0).count()),
        )?;

        compute_rolling(
            &mut dil.height, h2d, &returns_data, window_starts, starting_indexes.height, exit,
            |buf| StoredU32::from(buf.iter().copied().filter(|r| **r < 0.0).count()),
        )?;

        compute_rolling(
            &mut minr.height, h2d, &returns_data, window_starts, starting_indexes.height, exit,
            |buf| {
                buf.iter()
                    .copied()
                    .reduce(|a, b| if *b < *a { b } else { a })
                    .unwrap_or_default()
            },
        )?;

        compute_rolling(
            &mut maxr.height, h2d, &returns_data, window_starts, starting_indexes.height, exit,
            |buf| {
                buf.iter()
                    .copied()
                    .reduce(|a, b| if *b > *a { b } else { a })
                    .unwrap_or_default()
            },
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn compute_class_cumulative(
    days_in_profit: &mut ByDcaClass<ComputedFromHeightLast<StoredU32>>,
    days_in_loss: &mut ByDcaClass<ComputedFromHeightLast<StoredU32>>,
    min_return: &mut ByDcaClass<ComputedFromHeightLast<StoredF32>>,
    max_return: &mut ByDcaClass<ComputedFromHeightLast<StoredF32>>,
    returns: &ByDcaClass<ComputedFromHeightLast<StoredF32>>,
    h2d: &EagerVec<PcoVec<Height, Day1>>,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let start_days = ByDcaClass::<()>::start_days();

    for (((((dip, dil), minr), maxr), ret), from) in days_in_profit
        .iter_mut()
        .zip(days_in_loss.iter_mut())
        .zip(min_return.iter_mut())
        .zip(max_return.iter_mut())
        .zip(returns.iter())
        .zip(start_days)
    {
        compute_cumulative(
            &mut dip.height, h2d, &ret.day1, from, starting_indexes.height, exit,
            StoredU32::ZERO,
            |prev, ret| if *ret > 0.0 { prev + StoredU32::ONE } else { prev },
        )?;

        compute_cumulative(
            &mut dil.height, h2d, &ret.day1, from, starting_indexes.height, exit,
            StoredU32::ZERO,
            |prev, ret| if *ret < 0.0 { prev + StoredU32::ONE } else { prev },
        )?;

        compute_cumulative(
            &mut minr.height, h2d, &ret.day1, from, starting_indexes.height, exit,
            StoredF32::from(f32::MAX),
            |prev, ret| if *ret < *prev { ret } else { prev },
        )?;

        compute_cumulative(
            &mut maxr.height, h2d, &ret.day1, from, starting_indexes.height, exit,
            StoredF32::from(f32::MIN),
            |prev, ret| if *ret > *prev { ret } else { prev },
        )?;
    }
    Ok(())
}

/// Compute a rolling day-window metric at height level using _start vecs.
#[allow(clippy::too_many_arguments)]
fn compute_rolling<T: PcoVecValue + Default>(
    output: &mut EagerVec<PcoVec<Height, T>>,
    h2d: &EagerVec<PcoVec<Height, Day1>>,
    returns_data: &[StoredF32],
    window_starts: &EagerVec<PcoVec<Height, Height>>,
    starting_height: Height,
    exit: &Exit,
    mut aggregate: impl FnMut(&[StoredF32]) -> T,
) -> Result<()> {
    // Cursor + cache avoids per-height PcoVec page decompression for the
    // h2d lookback read.  Window-start heights are non-decreasing so the
    // cursor only moves forward; the cache handles repeated values.
    let mut h2d_cursor = h2d.cursor();
    let mut last_ws = Height::ZERO;
    let mut last_ws_di = Day1::default();

    output.compute_transform2(
        starting_height,
        h2d,
        window_starts,
        |(h, di, window_start, ..)| {
            let window_start_di = if window_start == last_ws {
                last_ws_di
            } else {
                let target = window_start.to_usize();
                let ws_di = if target >= h2d_cursor.position() {
                    h2d_cursor.advance(target - h2d_cursor.position());
                    h2d_cursor.next().unwrap_or_default()
                } else {
                    // Cursor past target (batch boundary); rare fallback
                    h2d.collect_one(window_start).unwrap_or_default()
                };
                last_ws = window_start;
                last_ws_di = ws_di;
                ws_di
            };
            let start = window_start_di.to_usize();
            let end = di.to_usize() + 1;
            if start >= end {
                return (h, T::default());
            }
            (h, aggregate(&returns_data[start..end]))
        },
        exit,
    )?;

    Ok(())
}

/// Compute a cumulative metric at height level starting from a fixed date.
#[allow(clippy::too_many_arguments)]
fn compute_cumulative<T: PcoVecValue + Default>(
    output: &mut EagerVec<PcoVec<Height, T>>,
    h2d: &EagerVec<PcoVec<Height, Day1>>,
    returns: &impl ReadableOptionVec<Day1, StoredF32>,
    from_day1: Day1,
    starting_height: Height,
    exit: &Exit,
    initial: T,
    mut accumulate: impl FnMut(T, StoredF32) -> T,
) -> Result<()> {
    let mut last_di: Option<Day1> = None;

    output.compute_transform(
        starting_height,
        h2d,
        |(h, di, this)| {
            let hi = h.to_usize();

            if last_di.is_none() && hi > 0 {
                last_di = Some(h2d.collect_one_at(hi - 1).unwrap());
            }

            if di < from_day1 {
                last_di = Some(di);
                return (h, T::default());
            }

            let prev_di = last_di;
            last_di = Some(di);

            let same_day = prev_di.is_some_and(|prev| prev == di);
            if same_day {
                (h, this.collect_one_at(hi - 1).unwrap_or_default())
            } else {
                let prev = if hi > 0 && prev_di.is_some_and(|pd| pd >= from_day1) {
                    this.collect_one_at(hi - 1).unwrap_or_default()
                } else {
                    initial
                };
                let ret = returns.collect_one_flat(di).unwrap_or_default();
                (h, accumulate(prev, ret))
            }
        },
        exit,
    )?;

    Ok(())
}
