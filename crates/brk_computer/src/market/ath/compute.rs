use brk_error::Result;
use brk_types::StoredU16;
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.price_ath.usd.height.compute_all_time_high(
            starting_indexes.height,
            &prices.usd.price,
            exit,
        )?;

        let mut prev = None;
        self.days_since_price_ath.height.compute_transform2(
            starting_indexes.height,
            &self.price_ath.usd.height,
            &prices.usd.price,
            |(i, ath, price, slf)| {
                if prev.is_none() {
                    let i = i.to_usize();
                    prev.replace(if i > 0 {
                        slf.collect_one_at(i - 1).unwrap()
                    } else {
                        StoredU16::default()
                    });
                }
                let days = if *price == *ath {
                    StoredU16::default()
                } else {
                    prev.unwrap() + StoredU16::new(1)
                };
                prev.replace(days);
                (i, days)
            },
            exit,
        )?;

        let mut prev = None;
        self.max_days_between_price_aths.height.compute_transform(
            starting_indexes.height,
            &self.days_since_price_ath.height,
            |(i, days, slf)| {
                if prev.is_none() {
                    let i = i.to_usize();
                    prev.replace(if i > 0 {
                        slf.collect_one_at(i - 1).unwrap()
                    } else {
                        StoredU16::ZERO
                    });
                }
                let max = prev.unwrap().max(days);
                prev.replace(max);
                (i, max)
            },
            exit,
        )?;

        Ok(())
    }
}
