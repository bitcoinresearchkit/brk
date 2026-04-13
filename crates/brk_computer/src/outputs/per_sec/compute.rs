use brk_error::Result;
use brk_types::{Indexes, StoredF32};
use vecdb::Exit;

use super::Vecs;
use crate::{internal::Windows, outputs::CountVecs};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        count: &CountVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        let h = starting_indexes.height;
        let sums = count.total.rolling.sum.0.as_array();
        let per_sec = self.0.as_mut_array();
        for (i, &secs) in Windows::<()>::SECS.iter().enumerate() {
            per_sec[i].height.compute_transform(
                h,
                &sums[i].height,
                |(h, sum, ..)| (h, StoredF32::from(*sum as f64 / secs)),
                exit,
            )?;
        }
        Ok(())
    }
}
