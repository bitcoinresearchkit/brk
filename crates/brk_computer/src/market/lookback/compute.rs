use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::{ComputeIndexes, price};

impl Vecs {
    pub fn compute(
        &mut self,
        price: &price::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        let close = &price.usd.split.close.dateindex;

        for (price_ago, days) in self.price_ago.iter_mut_with_days() {
            price_ago.compute_all(starting_indexes, exit, |v| {
                v.compute_previous_value(starting_indexes.dateindex, close, days as usize, exit)?;
                Ok(())
            })?;
        }

        Ok(())
    }
}
