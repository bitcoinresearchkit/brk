use brk_error::Result;
use brk_types::{BasisPointsSigned32, Bitcoin, Cents, Date, Day1, Dollars, Indexes, Sats};
use vecdb::{AnyVec, Exit, ReadableOptionVec, ReadableVec, VecIndex};

use super::{ByDcaPeriod, Vecs};
use crate::{blocks, indexes, internal::RatioDiffCentsBps32, market, prices};

const DCA_AMOUNT: Dollars = Dollars::mint(100.0);

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        lookback: &market::lookback::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.db.sync_bg_tasks()?;

        let h2d = &indexes.height.day1;
        let close = &prices.split.close.usd.day1;

        let first_price_di = Day1::try_from(Date::new(2010, 7, 12)).unwrap().to_usize();

        // Compute per-height DCA sats contribution once (reused by all periods).
        // Value = sats_from_dca(close_price) on day-boundary blocks, Sats::ZERO otherwise.
        {
            let mut last_di: Option<Day1> = None;
            self.sats_per_day.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    if last_di.is_none() && h.to_usize() > 0 {
                        last_di = Some(h2d.collect_one_at(h.to_usize() - 1).unwrap());
                    }
                    let same_day = last_di.is_some_and(|prev| prev == di);
                    last_di = Some(di);
                    if same_day {
                        (h, Sats::ZERO)
                    } else {
                        let s = close
                            .collect_one_flat(di)
                            .map(sats_from_dca)
                            .unwrap_or(Sats::ZERO);
                        (h, s)
                    }
                },
                exit,
            )?;
        }

        // DCA by period - stack (rolling sum via _start vecs)
        for (stack, days) in self.period.dca_stack.iter_mut_with_days() {
            let window_starts = blocks.lookback.start_vec(days as usize);
            stack.sats.height.compute_rolling_sum(
                starting_indexes.height,
                window_starts,
                &self.sats_per_day,
                exit,
            )?;
        }

        // DCA by period - stack cents (sats × price)
        for stack in self.period.dca_stack.iter_mut() {
            stack.compute(prices, starting_indexes.height, exit)?;
        }

        // DCA by period - average price (derived from stack)
        let starting_height = starting_indexes.height.to_usize();
        for (average_price, stack, days) in self
            .period
            .dca_cost_basis
            .zip_mut_with_days(&self.period.dca_stack)
        {
            let days = days as usize;
            average_price.cents.height.compute_transform2(
                starting_indexes.height,
                h2d,
                &stack.sats.height,
                |(h, di, stack_sats, ..)| {
                    let di_usize = di.to_usize();
                    let avg = if di_usize > first_price_di {
                        let num_days = days.min(di_usize + 1 - first_price_di);
                        Cents::from(DCA_AMOUNT * num_days / Bitcoin::from(stack_sats))
                    } else {
                        Cents::ZERO
                    };
                    (h, avg)
                },
                exit,
            )?;
        }

        // DCA by period - returns (compute from average price)
        for (returns, (average_price, _)) in self
            .period
            .dca_return
            .iter_mut()
            .zip(self.period.dca_cost_basis.iter_with_days())
        {
            returns.compute_binary::<Cents, Cents, RatioDiffCentsBps32>(
                starting_indexes.height,
                &prices.spot.cents.height,
                &average_price.cents.height,
                exit,
            )?;
        }

        // DCA by period - CAGR (computed from returns at height level)
        for (cagr, returns, days) in self
            .period
            .dca_cagr
            .zip_mut_with_period(&self.period.dca_return)
        {
            let years = days as f64 / 365.0;
            cagr.bps.height.compute_transform(
                starting_indexes.height,
                &returns.bps.height,
                |(h, r, ..)| {
                    let ratio = f64::from(r);
                    let v = (ratio + 1.0).powf(1.0 / years) - 1.0;
                    (h, BasisPointsSigned32::from(v))
                },
                exit,
            )?;
        }

        // Lump sum by period - stack
        let lookback_dca = ByDcaPeriod::from_lookback(&lookback.price_past);
        for (stack, lookback_price, days) in
            self.period.lump_sum_stack.zip_mut_with_days(&lookback_dca)
        {
            let total_invested = DCA_AMOUNT * days as usize;
            stack.sats.height.compute_transform2(
                starting_indexes.height,
                h2d,
                &lookback_price.cents.height,
                |(h, _di, lp, ..)| {
                    let sats = if lp == Cents::ZERO {
                        Sats::ZERO
                    } else {
                        Sats::from(Bitcoin::from(total_invested / Dollars::from(lp)))
                    };
                    (h, sats)
                },
                exit,
            )?;
        }

        // Lump sum by period - stack cents (sats × price)
        for stack in self.period.lump_sum_stack.iter_mut() {
            stack.compute(prices, starting_indexes.height, exit)?;
        }

        // Lump sum by period - returns (compute from lookback price)
        for (returns, (lookback_price, _)) in self
            .period
            .lump_sum_return
            .iter_mut()
            .zip(lookback_dca.iter_with_days())
        {
            returns.compute_binary::<Cents, Cents, RatioDiffCentsBps32>(
                starting_indexes.height,
                &prices.spot.cents.height,
                &lookback_price.cents.height,
                exit,
            )?;
        }

        // DCA by year class - stack (cumulative sum from class start date)
        let start_days = super::ByDcaClass::<()>::start_days();
        for (stack, day1) in self.class.dca_stack.iter_mut().zip(start_days) {
            let mut last_di: Option<Day1> = None;
            let cls_start = stack.sats.height.len().min(starting_height);
            let mut prev_value = if cls_start > 0 {
                stack
                    .sats
                    .height
                    .collect_one_at(cls_start - 1)
                    .unwrap_or_default()
            } else {
                Sats::ZERO
            };

            stack.sats.height.compute_transform(
                starting_indexes.height,
                h2d,
                |(h, di, _)| {
                    let hi = h.to_usize();

                    if last_di.is_none() && hi > 0 {
                        last_di = Some(h2d.collect_one_at(hi - 1).unwrap());
                    }

                    if di < day1 {
                        last_di = Some(di);
                        prev_value = Sats::ZERO;
                        return (h, Sats::ZERO);
                    }

                    let prev_di = last_di;
                    last_di = Some(di);

                    let same_day = prev_di.is_some_and(|prev| prev == di);
                    let result = if same_day {
                        prev_value
                    } else {
                        let prev = if hi > 0 && prev_di.is_some_and(|pd| pd >= day1) {
                            prev_value
                        } else {
                            Sats::ZERO
                        };
                        let s = close
                            .collect_one_flat(di)
                            .map(sats_from_dca)
                            .unwrap_or(Sats::ZERO);
                        prev + s
                    };
                    prev_value = result;
                    (h, result)
                },
                exit,
            )?;
        }

        // DCA by year class - stack cents (sats × price)
        for stack in self.class.dca_stack.iter_mut() {
            stack.compute(prices, starting_indexes.height, exit)?;
        }

        // DCA by year class - average price (derived from stack)
        let start_days = super::ByDcaClass::<()>::start_days();
        for ((average_price, stack), from) in self
            .class
            .dca_cost_basis
            .iter_mut()
            .zip(self.class.dca_stack.iter())
            .zip(start_days)
        {
            let from_usize = from.to_usize();
            average_price.cents.height.compute_transform2(
                starting_indexes.height,
                h2d,
                &stack.sats.height,
                |(h, di, stack_sats, ..)| {
                    let di_usize = di.to_usize();
                    if di_usize < from_usize {
                        return (h, Cents::ZERO);
                    }
                    let num_days = di_usize + 1 - from_usize;
                    let avg = Cents::from(DCA_AMOUNT * num_days / Bitcoin::from(stack_sats));
                    (h, avg)
                },
                exit,
            )?;
        }

        // DCA by year class - returns (compute from average price)
        for (returns, average_price) in self
            .class
            .dca_return
            .iter_mut()
            .zip(self.class.dca_cost_basis.iter())
        {
            returns.compute_binary::<Cents, Cents, RatioDiffCentsBps32>(
                starting_indexes.height,
                &prices.spot.cents.height,
                &average_price.cents.height,
                exit,
            )?;
        }

        let exit = exit.clone();
        self.db.run_bg(move |db| {
            let _lock = exit.lock();
            db.compact_deferred_default()
        });
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
