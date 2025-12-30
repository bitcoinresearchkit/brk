use brk_error::Result;
use brk_types::{DateIndex, StoredF32};
use vecdb::{CollectableVec, Exit};

use super::Vecs;
use crate::Indexes;

impl Vecs {
    pub fn compute<V>(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
        _1d_price_returns_dateindex: &V,
    ) -> Result<()>
    where
        V: CollectableVec<DateIndex, StoredF32>,
    {
        self.indexes_to_1d_returns_1w_sd.compute_all(
            starting_indexes,
            exit,
            _1d_price_returns_dateindex,
        )?;

        self.indexes_to_1d_returns_1m_sd.compute_all(
            starting_indexes,
            exit,
            _1d_price_returns_dateindex,
        )?;

        self.indexes_to_1d_returns_1y_sd.compute_all(
            starting_indexes,
            exit,
            _1d_price_returns_dateindex,
        )?;

        Ok(())
    }
}
