use brk_error::Result;
use brk_types::StoredF32;
use vecdb::Exit;

use super::Vecs;
use crate::{price, utils::OptionExt, ComputeIndexes};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let open = price.timeindexes_to_price_open.dateindex.u();
        let low = price.timeindexes_to_price_low.dateindex.u();
        let high = price.timeindexes_to_price_high.dateindex.u();

        self.indexes_to_price_1w_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(starting_indexes.dateindex, low, 7, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1w_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(starting_indexes.dateindex, high, 7, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_2w_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(starting_indexes.dateindex, low, 14, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_2w_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(starting_indexes.dateindex, high, 14, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1m_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(starting_indexes.dateindex, low, 30, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1m_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(starting_indexes.dateindex, high, 30, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1y_min
            .compute_all(starting_indexes, exit, |v| {
                v.compute_min(starting_indexes.dateindex, low, 365, exit)?;
                Ok(())
            })?;

        self.indexes_to_price_1y_max
            .compute_all(starting_indexes, exit, |v| {
                v.compute_max(starting_indexes.dateindex, high, 365, exit)?;
                Ok(())
            })?;

        self.dateindex_to_price_true_range.compute_transform3(
            starting_indexes.dateindex,
            open,
            high,
            low,
            |(i, open, high, low, ..)| {
                let high_min_low = **high - **low;
                let high_min_open = (**high - **open).abs();
                let low_min_open = (**low - **open).abs();
                (i, high_min_low.max(high_min_open).max(low_min_open).into())
            },
            exit,
        )?;

        self.dateindex_to_price_true_range_2w_sum.compute_sum(
            starting_indexes.dateindex,
            &self.dateindex_to_price_true_range,
            14,
            exit,
        )?;

        self.indexes_to_price_2w_choppiness_index
            .compute_all(starting_indexes, exit, |v| {
                let n = 14;
                let log10n = (n as f32).log10();
                v.compute_transform3(
                    starting_indexes.dateindex,
                    &self.dateindex_to_price_true_range_2w_sum,
                    self.indexes_to_price_2w_max.dateindex.u(),
                    self.indexes_to_price_2w_min.dateindex.u(),
                    |(i, tr_sum, max, min, ..)| {
                        (
                            i,
                            StoredF32::from(
                                100.0 * (*tr_sum / (*max - *min) as f32).log10() / log10n,
                            ),
                        )
                    },
                    exit,
                )?;
                Ok(())
            })?;

        Ok(())
    }
}
