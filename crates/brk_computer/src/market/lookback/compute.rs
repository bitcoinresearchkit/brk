use brk_error::Result;
use brk_indexer::Indexer;
use vecdb::Exit;

use super::Vecs;
use crate::{blocks, price};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        indexer: &Indexer,
        blocks: &blocks::Vecs,
        prices: &price::Vecs,
        exit: &Exit,
    ) -> Result<()> {
        let starting_height = indexer.safe_lengths().height;
        let price = &prices.spot.cents.height;

        for (price_past, days) in self.price_past.iter_mut_with_days() {
            let window_starts = blocks.lookback.start_vec(days as usize);
            price_past.cents.height.compute_lookback(
                starting_height,
                window_starts,
                price,
                exit,
            )?;
        }

        Ok(())
    }
}
