use brk_error::Result;
use brk_types::Indexes;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, prices};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let price = &prices.spot.cents.height;

        for (price_lookback, days) in self.price_lookback.iter_mut_with_days() {
            let window_starts = blocks.lookback.start_vec(days as usize);
            price_lookback.cents.height.compute_lookback(
                starting_indexes.height,
                window_starts,
                price,
                exit,
            )?;
        }

        Ok(())
    }
}
