use brk_error::Result;
use brk_types::Dollars;
use vecdb::{Exit, ReadableVec, VecIndex};

use super::Vecs;
use crate::{blocks, ComputeIndexes, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close_data: Vec<Dollars> = prices.usd.price.collect();

        for (price_ago, days) in self.price_ago.iter_mut_with_days() {
            let window_starts = blocks.count.start_vec(days as usize);
            price_ago.usd.height.compute_transform(
                starting_indexes.height,
                window_starts,
                |(h, start_h, _)| {
                    let val = close_data[start_h.to_usize()];
                    (h, val)
                },
                exit,
            )?;
        }

        Ok(())
    }
}
