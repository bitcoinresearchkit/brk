use brk_error::Result;
use vecdb::Exit;

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
        let price = &prices.price.cents.height;

        for (price_ago, days) in self.price_ago.iter_mut_with_days() {
            let window_starts = blocks.count.start_vec(days as usize);
            price_ago.cents.height.compute_lookback(
                starting_indexes.height,
                window_starts,
                price,
                exit,
            )?;
        }

        Ok(())
    }
}
