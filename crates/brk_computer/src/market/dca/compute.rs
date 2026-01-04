use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{
    market::lookback,
    price,
    traits::{ComputeDCAAveragePriceViaLen, ComputeDCAStackViaLen, ComputeLumpSumStackViaLen},
    utils::OptionExt,
    ComputeIndexes,
};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        lookback: &lookback::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = price.usd.timeindexes_to_price_close.dateindex.u();

        // DCA by period - stack
        for (stack, days) in self.period_stack.iter_mut_with_days() {
            stack.compute_all(Some(price), starting_indexes, exit, |v| {
                v.compute_dca_stack_via_len(starting_indexes.dateindex, close, days as usize, exit)?;
                Ok(())
            })?;
        }

        // DCA by period - avg_price (needs stack's dateindex)
        for (avg_price, stack, days) in self
            .period_avg_price
            .zip_mut_with_days(&self.period_stack)
        {
            avg_price.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_len(
                    starting_indexes.dateindex,
                    stack.sats.dateindex.u(),
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // DCA by period - CAGR (computed from returns)
        for (cagr, returns, days) in self
            .period_cagr
            .zip_mut_with_period(&self.period_returns)
        {
            cagr.compute_all(starting_indexes, exit, |v| {
                v.compute_cagr(
                    starting_indexes.dateindex,
                    returns.dateindex.u(),
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // Lump sum by period - stack (for comparison with DCA)
        let lookback_dca = lookback.price_ago.as_dca_period();
        for (stack, lookback_price, days) in self
            .period_lump_sum_stack
            .zip_mut_with_days(&lookback_dca)
        {
            stack.compute_all(Some(price), starting_indexes, exit, |v| {
                v.compute_lump_sum_stack_via_len(
                    starting_indexes.dateindex,
                    close,
                    lookback_price.dateindex.u(),
                    days as usize,
                    exit,
                )?;
                Ok(())
            })?;
        }

        // DCA by year class - stack and avg_price
        let dateindexes = super::ByDcaClass::<()>::dateindexes();
        for ((stack, avg_price), dateindex) in self
            .class_stack
            .iter_mut()
            .zip(self.class_avg_price.iter_mut())
            .zip(dateindexes)
        {
            stack.compute_all(Some(price), starting_indexes, exit, |v| {
                v.compute_dca_stack_via_from(starting_indexes.dateindex, close, dateindex, exit)?;
                Ok(())
            })?;

            avg_price.compute_all(starting_indexes, exit, |v| {
                v.compute_dca_avg_price_via_from(
                    starting_indexes.dateindex,
                    stack.sats.dateindex.u(),
                    dateindex,
                    exit,
                )?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
