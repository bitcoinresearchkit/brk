use brk_error::Result;
use brk_types::{Close, Dollars, StoredF32, StoredU32};
use vecdb::Exit;

use super::{ByDcaClass, ByDcaPeriod, Vecs};
use crate::{
    ComputeIndexes,
    internal::{ComputedFromDateLast, LazyBinaryFromDateLast},
    market::lookback,
    price,
    traits::{ComputeDCAAveragePriceViaLen, ComputeDCAStackViaLen, ComputeLumpSumStackViaLen},
};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        lookback: &lookback::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = &price.usd.split.close.dateindex;

        // DCA by period - stack
        for (stack, days) in self.period_stack.iter_mut_with_days() {
            stack.compute_all(Some(price), starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(
                    starting_indexes.dateindex,
                    close,
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // DCA by period - average_price (needs stack's dateindex)
        for (average_price, stack, days) in self
            .period_average_price
            .zip_mut_with_days(&self.period_stack)
        {
            average_price.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_average_price_via_len(
                    starting_indexes.dateindex,
                    &stack.sats_dateindex,
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // DCA by period - CAGR (computed from returns)
        for (cagr, returns, days) in self.period_cagr.zip_mut_with_period(&self.period_returns) {
            cagr.compute_all(starting_indexes, exit, |v| {
                v.compute_cagr(
                    starting_indexes.dateindex,
                    &returns.dateindex,
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // DCA by period - profitability
        compute_period_profitability(
            &mut self.period_days_in_profit,
            &mut self.period_days_in_loss,
            &mut self.period_min_return,
            &mut self.period_max_return,
            &self.period_returns,
            starting_indexes,
            exit,
        )?;

        // Lump sum by period - stack (for comparison with DCA)
        let lookback_dca = lookback.price_ago.as_dca_period();
        for (stack, lookback_price, days) in
            self.period_lump_sum_stack.zip_mut_with_days(&lookback_dca)
        {
            stack.compute_all(Some(price), starting_indexes, exit, |v| {
                v.compute_lump_sum_stack_via_len(
                    starting_indexes.dateindex,
                    close,
                    &lookback_price.dateindex,
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // Lump sum by period - profitability
        compute_period_profitability(
            &mut self.period_lump_sum_days_in_profit,
            &mut self.period_lump_sum_days_in_loss,
            &mut self.period_lump_sum_min_return,
            &mut self.period_lump_sum_max_return,
            &self.period_lump_sum_returns,
            starting_indexes,
            exit,
        )?;

        // DCA by year class - stack and average_price
        let dateindexes = super::ByDcaClass::<()>::dateindexes();
        for ((stack, average_price), dateindex) in self
            .class_stack
            .iter_mut()
            .zip(self.class_average_price.iter_mut())
            .zip(dateindexes)
        {
            stack.compute_all(Some(price), starting_indexes, exit, |v| {
                v.compute_dca_stack_via_from(starting_indexes.dateindex, close, dateindex, exit)?;
                Ok(())
            })?;

            average_price.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_average_price_via_from(
                    starting_indexes.dateindex,
                    &stack.sats_dateindex,
                    dateindex,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // DCA by year class - profitability
        compute_class_profitability(
            &mut self.class_days_in_profit,
            &mut self.class_days_in_loss,
            &mut self.class_min_return,
            &mut self.class_max_return,
            &self.class_returns,
            starting_indexes,
            exit,
        )?;

        Ok(())
    }
}

fn compute_period_profitability(
    days_in_profit: &mut ByDcaPeriod<ComputedFromDateLast<StoredU32>>,
    days_in_loss: &mut ByDcaPeriod<ComputedFromDateLast<StoredU32>>,
    min_return: &mut ByDcaPeriod<ComputedFromDateLast<StoredF32>>,
    max_return: &mut ByDcaPeriod<ComputedFromDateLast<StoredF32>>,
    returns: &ByDcaPeriod<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,
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
        dip.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_rolling_count(
                starting_indexes.dateindex,
                &ret.dateindex,
                days as usize,
                |r| f32::from(*r) > 0.0,
                exit,
            )?)
        })?;

        dil.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_rolling_count(
                starting_indexes.dateindex,
                &ret.dateindex,
                days as usize,
                |r| f32::from(*r) < 0.0,
                exit,
            )?)
        })?;

        minr.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_min(
                starting_indexes.dateindex,
                &ret.dateindex,
                days as usize,
                exit,
            )?)
        })?;

        maxr.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_max(
                starting_indexes.dateindex,
                &ret.dateindex,
                days as usize,
                exit,
            )?)
        })?;
    }
    Ok(())
}

fn compute_class_profitability(
    days_in_profit: &mut ByDcaClass<ComputedFromDateLast<StoredU32>>,
    days_in_loss: &mut ByDcaClass<ComputedFromDateLast<StoredU32>>,
    min_return: &mut ByDcaClass<ComputedFromDateLast<StoredF32>>,
    max_return: &mut ByDcaClass<ComputedFromDateLast<StoredF32>>,
    returns: &ByDcaClass<LazyBinaryFromDateLast<StoredF32, Close<Dollars>, Dollars>>,
    starting_indexes: &ComputeIndexes,
    exit: &Exit,
) -> Result<()> {
    let dateindexes = ByDcaClass::<()>::dateindexes();

    for (((((dip, dil), minr), maxr), ret), from) in days_in_profit
        .iter_mut()
        .zip(days_in_loss.iter_mut())
        .zip(min_return.iter_mut())
        .zip(max_return.iter_mut())
        .zip(returns.iter())
        .zip(dateindexes)
    {
        dip.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_cumulative_count_from(
                starting_indexes.dateindex,
                &ret.dateindex,
                from,
                |r| f32::from(*r) > 0.0,
                exit,
            )?)
        })?;

        dil.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_cumulative_count_from(
                starting_indexes.dateindex,
                &ret.dateindex,
                from,
                |r| f32::from(*r) < 0.0,
                exit,
            )?)
        })?;

        minr.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_all_time_low_from(
                starting_indexes.dateindex,
                &ret.dateindex,
                from,
                exit,
            )?)
        })?;

        maxr.compute_all(starting_indexes, exit, |v| {
            Ok(v.compute_all_time_high_from(
                starting_indexes.dateindex,
                &ret.dateindex,
                from,
                exit,
            )?)
        })?;
    }
    Ok(())
}
