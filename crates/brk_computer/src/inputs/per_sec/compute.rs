use brk_error::Result;
use brk_indexer::Lengths;
use brk_types::StoredF32;
use vecdb::Exit;

use super::Vecs;
use crate::{inputs::CountVecs, internal::Windows};

impl Vecs {
    pub(crate) fn compute(
        &mut self,
        count: &CountVecs,
        starting_lengths: &Lengths,
        exit: &Exit,
    ) -> Result<()> {
        let h = starting_lengths.height;
        let sums = count.rolling.sum.0.as_array();
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
