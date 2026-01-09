use brk_error::Result;
use vecdb::Exit;

use super::Vecs;
use crate::ComputeIndexes;

impl Vecs {
    pub fn compute(&mut self, starting_indexes: &ComputeIndexes, exit: &Exit) -> Result<()> {
        self.usd.compute(starting_indexes, &self.cents, exit)?;

        self.sats.compute(starting_indexes, &self.usd, exit)?;

        let _lock = exit.lock();
        self.db().compact()?;
        Ok(())
    }
}
