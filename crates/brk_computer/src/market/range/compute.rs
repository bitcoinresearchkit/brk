use brk_error::Result;
use brk_types::{BasisPoints16, Indexes, StoredF32};
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{blocks, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let price = &prices.price.cents.height;

        for (min_vec, max_vec, starts) in [
            (
                &mut self.min._1w.cents.height,
                &mut self.max._1w.cents.height,
                &blocks.lookback.height_1w_ago,
            ),
            (
                &mut self.min._2w.cents.height,
                &mut self.max._2w.cents.height,
                &blocks.lookback.height_2w_ago,
            ),
            (
                &mut self.min._1m.cents.height,
                &mut self.max._1m.cents.height,
                &blocks.lookback.height_1m_ago,
            ),
            (
                &mut self.min._1y.cents.height,
                &mut self.max._1y.cents.height,
                &blocks.lookback.height_1y_ago,
            ),
        ] {
            min_vec.compute_rolling_min_from_starts(
                starting_indexes.height,
                starts,
                price,
                exit,
            )?;
            max_vec.compute_rolling_max_from_starts(
                starting_indexes.height,
                starts,
                price,
                exit,
            )?;
        }

        // True range at block level: |price[h] - price[h-1]|
        let mut prev_price = None;
        self.true_range.height.compute_transform(
            starting_indexes.height,
            price,
            |(h, current, ..)| {
                let prev = prev_price.unwrap_or_else(|| {
                    if h.to_usize() > 0 {
                        price.collect_one_at(h.to_usize() - 1).unwrap_or(current)
                    } else {
                        current
                    }
                });
                prev_price = Some(current);
                let (c, p) = (f64::from(current), f64::from(prev));
                let tr = (c - p).abs();
                (h, StoredF32::from(tr))
            },
            exit,
        )?;

        // 2w rolling sum of true range
        self.true_range_sum_2w.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.lookback.height_2w_ago,
            &self.true_range.height,
            exit,
        )?;

        self.choppiness_index_2w
            .bps
            .height
            .compute_transform4(
                starting_indexes.height,
                &self.true_range_sum_2w.height,
                &self.max._2w.cents.height,
                &self.min._2w.cents.height,
                &blocks.lookback.height_2w_ago,
                |(h, tr_sum, max, min, window_start, ..)| {
                    let range = f64::from(max) - f64::from(min);
                    let n = (h.to_usize() - window_start.to_usize() + 1) as f32;
                    let ci = if range > 0.0 && n > 1.0 {
                        BasisPoints16::from(
                            (*tr_sum / range as f32).log10() as f64 / n.log10() as f64,
                        )
                    } else {
                        BasisPoints16::ZERO
                    };
                    (h, ci)
                },
                exit,
            )?;

        Ok(())
    }
}
