use super::ValuePerBlock;
use crate::CachePolicy;
use brk_error::Result;
use brk_exit::Exit;
use brk_types::{Height, Sats};
use vecdb::{ReadableVec, Rw, VecIndex, VecValue};

impl<S: CachePolicy> ValuePerBlock<Rw, S> {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_sats_from_indexes<A, B>(
        &mut self,
        max_from: Height,
        first_indexes: &impl ReadableVec<Height, A>,
        indexes_count: &impl ReadableVec<Height, B>,
        source: &impl ReadableVec<A, Sats>,
        exit: &Exit,
    ) -> Result<()>
    where
        A: VecIndex + VecValue,
        B: VecValue,
        usize: From<B>,
    {
        super::cumulative::compute_sats_height_from_indexes(
            &mut self.sats.height,
            max_from,
            first_indexes,
            indexes_count,
            source,
            |_| true,
            exit,
        )
    }
}
