use brk_error::Result;
use brk_types::{Day1, StoredU16};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, indexes, prices, traits::ComputeDrawdown};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.price_ath.cents.height.compute_all_time_high(
            starting_indexes.height,
            &prices.price.cents.height,
            exit,
        )?;

        let mut ath_day: Option<Day1> = None;
        self.days_since_price_ath.height.compute_transform3(
            starting_indexes.height,
            &self.price_ath.cents.height,
            &prices.price.cents.height,
            &indexes.height.day1,
            |(i, ath, price, day, slf)| {
                if ath_day.is_none() {
                    let idx = i.to_usize();
                    ath_day = Some(if idx > 0 {
                        let prev_days_since = slf.collect_one_at(idx - 1).unwrap();
                        Day1::from(day.to_usize().saturating_sub(usize::from(prev_days_since)))
                    } else {
                        day
                    });
                }
                if price == ath {
                    ath_day = Some(day);
                    (i, StoredU16::default())
                } else {
                    let days_since = (day.to_usize() - ath_day.unwrap().to_usize()) as u16;
                    (i, StoredU16::from(days_since))
                }
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

        self.price_drawdown.height.compute_drawdown(
            starting_indexes.height,
            &prices.price.cents.height,
            &self.price_ath.cents.height,
            exit,
        )?;

        Ok(())
    }
}
