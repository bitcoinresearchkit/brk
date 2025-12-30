use brk_error::Result;
use brk_types::StoredU16;
use vecdb::{Exit, GenericStoredVec, TypedVecIterator, VecIndex};

use super::Vecs;
use crate::{Indexes, price, traits::ComputeDrawdown, utils::OptionExt};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.height_to_price_ath.compute_all_time_high(
            starting_indexes.height,
            &price.chainindexes_to_price_high.height,
            exit,
        )?;

        self.height_to_price_drawdown.compute_drawdown(
            starting_indexes.height,
            &price.chainindexes_to_price_close.height,
            &self.height_to_price_ath,
            exit,
        )?;

        self.indexes_to_price_ath
            .compute_all(starting_indexes, exit, |v| {
                v.compute_all_time_high(
                    starting_indexes.dateindex,
                    price.timeindexes_to_price_high.dateindex.u(),
                    exit,
                )?;
                Ok(())
            })?;

        self.indexes_to_days_since_price_ath
            .compute_all(starting_indexes, exit, |v| {
                let mut high_iter = price.timeindexes_to_price_high.dateindex.u().into_iter();
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_price_ath.dateindex.u(),
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

        self.indexes_to_max_days_between_price_aths
            .compute_all(starting_indexes, exit, |v| {
                let mut prev = None;
                v.compute_transform(
                    starting_indexes.dateindex,
                    self.indexes_to_days_since_price_ath.dateindex.u(),
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
