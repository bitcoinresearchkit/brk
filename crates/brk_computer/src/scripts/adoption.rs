use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{BasisPoints16, Indexes, Version};
use vecdb::{Database, Exit, Rw, StorageMode};

use crate::{
    indexes,
    internal::{PercentFromHeight, RatioU64Bp16},
    outputs,
};

use super::count::Vecs as CountVecs;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub taproot: PercentFromHeight<BasisPoints16, M>,
    pub segwit: PercentFromHeight<BasisPoints16, M>,
}

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            taproot: PercentFromHeight::forced_import(
                db,
                "taproot_adoption",
                version,
                indexes,
            )?,
            segwit: PercentFromHeight::forced_import(
                db,
                "segwit_adoption",
                version,
                indexes,
            )?,
        })
    }

    pub(crate) fn compute(
        &mut self,
        count: &CountVecs,
        outputs_count: &outputs::CountVecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.taproot.compute_binary::<_, _, RatioU64Bp16>(
            starting_indexes.height,
            &count.p2tr.height,
            &outputs_count.total_count.full.sum,
            exit,
        )?;

        self.segwit.compute_binary::<_, _, RatioU64Bp16>(
            starting_indexes.height,
            &count.segwit.height,
            &outputs_count.total_count.full.sum,
            exit,
        )?;

        Ok(())
    }
}
