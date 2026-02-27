use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{StoredF32, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{ComputeIndexes, indexes, internal::{ComputedFromHeightLast, RatioU64F32}, outputs};

use super::count::Vecs as CountVecs;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub taproot: ComputedFromHeightLast<StoredF32, M>,
    pub segwit: ComputedFromHeightLast<StoredF32, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            taproot: ComputedFromHeightLast::forced_import(
                db,
                "taproot_adoption",
                version,
                indexes,
            )?,
            segwit: ComputedFromHeightLast::forced_import(db, "segwit_adoption", version, indexes)?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        count: &CountVecs,
        outputs_count: &outputs::CountVecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.taproot.compute_binary::<_, _, RatioU64F32>(
            starting_indexes.height,
            &count.p2tr.height,
            &outputs_count.total_count.full.sum_cumulative.sum.0,
            exit,
        )?;

        self.segwit.compute_binary::<_, _, RatioU64F32>(
            starting_indexes.height,
            &count.segwit.height,
            &outputs_count.total_count.full.sum_cumulative.sum.0,
            exit,
        )?;

        Ok(())
    }
}
