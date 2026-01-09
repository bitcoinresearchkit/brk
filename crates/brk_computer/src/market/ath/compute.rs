use brk_error::Result;
use brk_types::StoredU16;
use vecdb::{Exit, GenericStoredVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::{ComputeIndexes, price, traits::ComputeDrawdown};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.price_ath.height.compute_all_time_high(
            starting_indexes.height,
            &price.usd.split.high.height,
            exit,
        )?;

        self.price_drawdown.height.compute_drawdown(
            starting_indexes.height,
            &price.usd.split.close.height,
            &self.price_ath.height,
            exit,
        )?;

        self.price_ath.compute_rest(starting_indexes, exit, |v| {
            v.compute_all_time_high(
                starting_indexes.dateindex,
                &price.usd.split.high.dateindex,
                exit,
            )?;
            Ok(())
        })?;

        self.days_since_price_ath
            .compute_all(starting_indexes, exit, |v| {
                let mut high_iter = price.usd.split.high.dateindex.into_iter();
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.price_ath.dateindex,
                    |(i, ath, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
                            } else {
                                StoredU16::default()
                            });
                        }
                        let days = if *high_iter.get_unwrap(i) == ath {
                            StoredU16::default()
                        } else {
                            prev.unwrap() + StoredU16::new(1)
                        };
                        prev.replace(days);
                        (i, days)
                    },
                    exit,
                )?;
                Ok(())
            })?;

        self.max_days_between_price_aths
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    &self.days_since_price_ath.dateindex,
                    |(i, days, slf)| {
                        if prev.is_none() {
                            let i = i.to_usize();
                            prev.replace(if i > 0 {
                                slf.get_pushed_or_read_at_unwrap_once(i - 1)
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
            })?;

        Ok(())
    }
}
