use brk_error::Result;
use brk_types::StoredF32;
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{
    blocks, ComputeIndexes, prices,
    traits::{ComputeRollingMaxFromStarts, ComputeRollingMinFromStarts},
};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let price = &prices.price.usd.height;

        self.price_1w_min.usd.height.compute_rolling_min_from_starts(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            price,
            exit,
        )?;

        self.price_1w_max.usd.height.compute_rolling_max_from_starts(
            starting_indexes.height,
            &blocks.count.height_1w_ago,
            price,
            exit,
        )?;

        self.price_2w_min.usd.height.compute_rolling_min_from_starts(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            price,
            exit,
        )?;

        self.price_2w_max.usd.height.compute_rolling_max_from_starts(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            price,
            exit,
        )?;

        self.price_1m_min.usd.height.compute_rolling_min_from_starts(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            price,
            exit,
        )?;

        self.price_1m_max.usd.height.compute_rolling_max_from_starts(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            price,
            exit,
        )?;

        self.price_1y_min.usd.height.compute_rolling_min_from_starts(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            price,
            exit,
        )?;

        self.price_1y_max.usd.height.compute_rolling_max_from_starts(
            starting_indexes.height,
            &blocks.count.height_1y_ago,
            price,
            exit,
        )?;

        // True range at block level: |price[h] - price[h-1]|
        let mut prev_price = None;
        self.price_true_range.height.compute_transform(
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
                let tr = (*current - *prev).abs();
                (h, StoredF32::from(tr))
            },
            exit,
        )?;

        // 2w rolling sum of true range
        self.price_true_range_2w_sum.height.compute_rolling_sum(
            starting_indexes.height,
            &blocks.count.height_2w_ago,
            &self.price_true_range.height,
            exit,
        )?;

        self.price_2w_choppiness_index.height.compute_transform4(
            starting_indexes.height,
            &self.price_true_range_2w_sum.height,
            &self.price_2w_max.usd.height,
            &self.price_2w_min.usd.height,
            &blocks.count.height_2w_ago,
            |(h, tr_sum, max, min, window_start, ..)| {
                let range = *max - *min;
                let n = (h.to_usize() - window_start.to_usize() + 1) as f32;
                let ci = if range > 0.0 && n > 1.0 {
                    StoredF32::from(
                        100.0 * (*tr_sum / range as f32).log10() / n.log10(),
                    )
                } else {
                    StoredF32::NAN
                };
                (h, ci)
            },
            exit,
        )?;

        Ok(())
    }
}
